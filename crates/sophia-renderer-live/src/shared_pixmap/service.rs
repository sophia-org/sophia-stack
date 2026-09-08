use super::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::time::{Duration, Instant};

const QUEUE_CAPACITY: usize = 4;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(4);

enum Operation {
    Allocate(BufferHandle, Size, u8, AllocationReply),
    Update(
        LiveSharedPixmapUpdate,
        SyncSender<Result<(), LiveSharedPixmapError>>,
    ),
    Release(BufferHandle, SyncSender<Result<(), LiveSharedPixmapError>>),
}

struct Command {
    deadline: Instant,
    operation: Operation,
}

const ALLOCATION_PENDING: u8 = 0;
const ALLOCATION_ADOPTED: u8 = 1;
const ALLOCATION_CANCELLED: u8 = 2;

struct AllocationAdoption(AtomicU8);

impl AllocationAdoption {
    fn adopt(&self) -> bool {
        self.0
            .compare_exchange(
                ALLOCATION_PENDING,
                ALLOCATION_ADOPTED,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    fn cancel(&self) {
        let _ = self.0.compare_exchange(
            ALLOCATION_PENDING,
            ALLOCATION_CANCELLED,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
    }
}

#[derive(Debug, Eq, PartialEq)]
enum AllocationDisposition {
    NoAllocation,
    Adopted,
    Unclaimed,
}

struct AllocationReply {
    sender: SyncSender<Result<LiveSharedBufferAllocation, LiveSharedPixmapError>>,
    adoption: Arc<AllocationAdoption>,
    notification: Receiver<()>,
}

struct AllocationReceipt {
    receiver: Receiver<Result<LiveSharedBufferAllocation, LiveSharedPixmapError>>,
    adoption: Arc<AllocationAdoption>,
    notification: SyncSender<()>,
}

fn allocation_reply() -> (AllocationReply, AllocationReceipt) {
    let (sender, receiver) = mpsc::sync_channel(1);
    let (notify, notification) = mpsc::sync_channel(1);
    let adoption = Arc::new(AllocationAdoption(AtomicU8::new(ALLOCATION_PENDING)));
    (
        AllocationReply {
            sender,
            adoption: adoption.clone(),
            notification,
        },
        AllocationReceipt {
            receiver,
            adoption,
            notification: notify,
        },
    )
}

impl AllocationReply {
    /// A buffered reply transfers descriptors; only adoption transfers ownership.
    fn deliver(
        self,
        result: Result<LiveSharedBufferAllocation, LiveSharedPixmapError>,
        deadline: Instant,
    ) -> AllocationDisposition {
        let allocated = result.is_ok();
        let delivered = self.sender.send(result).is_ok();
        if !allocated {
            return AllocationDisposition::NoAllocation;
        }
        if delivered {
            let _ = self
                .notification
                .recv_timeout(deadline.saturating_duration_since(Instant::now()));
        }
        self.adoption.cancel();
        if self.adoption.0.load(Ordering::Acquire) == ALLOCATION_ADOPTED {
            AllocationDisposition::Adopted
        } else {
            AllocationDisposition::Unclaimed
        }
    }
}

impl AllocationReceipt {
    fn receive(
        &self,
        deadline: Instant,
    ) -> Result<LiveSharedBufferAllocation, LiveSharedPixmapError> {
        let result = self
            .receiver
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .map_err(|_| LiveSharedPixmapError::Unavailable)
            .and_then(|result| result);
        let result = match result {
            Ok(allocation) if Instant::now() < deadline && self.adoption.adopt() => Ok(allocation),
            Ok(_) => Err(LiveSharedPixmapError::Unavailable),
            Err(error) => Err(error),
        };
        self.adoption.cancel();
        let _ = self.notification.try_send(());
        result
    }
}

/// A bounded renderer worker; GPU synchronization never holds an authority lock.
#[derive(Clone)]
pub struct LiveSharedPixmapService {
    sender: SyncSender<Command>,
}

impl LiveSharedPixmapService {
    pub fn new(device: File) -> Result<Self, LiveSharedPixmapError> {
        let (sender, receiver) = mpsc::sync_channel::<Command>(QUEUE_CAPACITY);
        let (ready, startup) = mpsc::sync_channel(1);
        std::thread::Builder::new()
            .name("sophia-pixmap-export".into())
            .spawn(move || {
                let mut store = match LiveSharedPixmapStore::new(device) {
                    Ok(store) => store,
                    Err(error) => {
                        let _ = ready.send(Err(error));
                        return;
                    }
                };
                let probe = probe_storage(&mut store);
                if ready.send(probe).is_err() || probe.is_err() {
                    return;
                }
                while let Ok(command) = receiver.recv() {
                    let expired = Instant::now() >= command.deadline;
                    match command.operation {
                        Operation::Allocate(handle, size, depth, reply) => {
                            let result = if expired {
                                Err(LiveSharedPixmapError::Unavailable)
                            } else {
                                store.allocate(handle, size, depth)
                            };
                            if reply.deliver(result, command.deadline)
                                == AllocationDisposition::Unclaimed
                            {
                                store.release(handle);
                            }
                        }
                        Operation::Update(update, reply) => {
                            let result = if expired {
                                Err(LiveSharedPixmapError::Unavailable)
                            } else {
                                store.update(update)
                            };
                            let _ = reply.send(result);
                        }
                        Operation::Release(handle, reply) => {
                            // Releases remain effective after their caller times out.
                            store.release(handle);
                            let _ = reply.send(Ok(()));
                        }
                    }
                }
            })
            .map_err(|_| LiveSharedPixmapError::Unavailable)?;
        startup
            .recv_timeout(REQUEST_TIMEOUT)
            .map_err(|_| LiveSharedPixmapError::Unavailable)??;
        Ok(Self { sender })
    }

    pub fn allocate(
        &self,
        handle: BufferHandle,
        size: Size,
        depth: u8,
    ) -> Result<LiveSharedBufferAllocation, LiveSharedPixmapError> {
        let (reply, receipt) = allocation_reply();
        let deadline = self.enqueue(Operation::Allocate(handle, size, depth, reply))?;
        receipt.receive(deadline)
    }

    pub fn update(&self, update: LiveSharedPixmapUpdate) -> Result<(), LiveSharedPixmapError> {
        update.validate()?;
        let (sender, receiver) = mpsc::sync_channel(1);
        let deadline = self.enqueue(Operation::Update(update, sender))?;
        receiver
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .map_err(|_| LiveSharedPixmapError::Unavailable)?
    }

    pub fn release(&self, handle: BufferHandle) -> Result<(), LiveSharedPixmapError> {
        let (sender, receiver) = mpsc::sync_channel(1);
        let deadline = self.enqueue(Operation::Release(handle, sender))?;
        receiver
            .recv_timeout(deadline.saturating_duration_since(Instant::now()))
            .map_err(|_| LiveSharedPixmapError::Unavailable)?
    }

    fn enqueue(&self, operation: Operation) -> Result<Instant, LiveSharedPixmapError> {
        let deadline = Instant::now() + REQUEST_TIMEOUT;
        self.sender
            .try_send(Command {
                deadline,
                operation,
            })
            .map(|()| deadline)
            .map_err(|error| match error {
                mpsc::TrySendError::Full(_) => LiveSharedPixmapError::Capacity,
                mpsc::TrySendError::Disconnected(_) => LiveSharedPixmapError::Unavailable,
            })
    }
}

fn probe_storage(store: &mut LiveSharedPixmapStore) -> Result<(), LiveSharedPixmapError> {
    use sophia_renderer_native_egl::{
        NativeDmaBufPlane, NativeMultiPlaneDmaBufFrame, NativePixmapImportProbe,
    };
    for depth in [24, 32] {
        let handle = BufferHandle::from_raw(u64::MAX - u64::from(depth));
        let size = Size {
            width: 2,
            height: 1,
        };
        let allocation = store.allocate(handle, size, depth)?;
        let device = store
            .device
            .as_fd()
            .try_clone_to_owned()
            .map_err(|_| LiveSharedPixmapError::Unavailable)?;
        let frame = NativeMultiPlaneDmaBufFrame {
            width: size.width as u32,
            height: size.height as u32,
            format: allocation.descriptor.format,
            modifier: allocation.descriptor.modifier,
            plane_count: allocation.descriptor.plane_count,
            planes: std::array::from_fn(|index| {
                allocation.descriptor.planes[index].map(|plane| NativeDmaBufPlane {
                    fd: allocation.plane_fds[index].as_fd(),
                    offset: plane.offset,
                    stride: plane.stride,
                })
            }),
        };
        let consumer = NativePixmapImportProbe::new(File::from(device), frame)
            .map_err(|_| LiveSharedPixmapError::ExportFailed)?;
        // Rebinding one retained image must expose full and partial writes.
        let mut expected = vec![0; 8];
        for (revision, marker) in [(1, [0x21, 0x43, 0x65, 0xff]), (2, [0x76, 0x54, 0x32, 0xff])] {
            let (x, width) = if revision == 1 { (0, 2) } else { (1, 1) };
            store.update(LiveSharedPixmapUpdate {
                handle,
                revision,
                size,
                format: allocation.descriptor.format,
                patches: vec![LiveSharedPixmapPatch {
                    rect: Rect {
                        x,
                        y: 0,
                        width,
                        height: 1,
                    },
                    bytes: marker.repeat(width as usize),
                }],
            })?;
            expected[x as usize * 4..].copy_from_slice(
                &[marker[2], marker[1], marker[0], marker[3]].repeat(width as usize),
            );
            let pixels = consumer
                .read_rgba()
                .map_err(|_| LiveSharedPixmapError::UploadFailed)?;
            if pixels != expected {
                return Err(LiveSharedPixmapError::UploadFailed);
            }
        }
        store.release(handle);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/support/shared_pixmap_adoption.rs"
    ));
}
