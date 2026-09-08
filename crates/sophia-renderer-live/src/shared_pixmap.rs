//! Persistent storage exported to clients, independent of scanout retirement.

use std::collections::BTreeMap;
use std::fs::File;
use std::os::fd::{AsFd, OwnedFd};

use sophia_protocol::{BufferHandle, DmaBufDescriptor, DmaBufPlaneDescriptor, Rect, Size};

use crate::LiveSharedBufferAllocation;

mod service;
pub use service::LiveSharedPixmapService;

const MAX_PIXMAP_BYTES: usize = 64 * 1024 * 1024;
const MAX_STORE_BYTES: usize = 256 * 1024 * 1024;
const MAX_PIXMAPS: usize = 1024;
const MAX_PATCHES: usize = 32;

/// A tightly packed rectangle in the backing's native four-byte pixel format.
#[derive(Debug)]
pub struct LiveSharedPixmapPatch {
    pub rect: Rect,
    pub bytes: Vec<u8>,
}

/// An ordered update to one persistent exported backing.
#[derive(Debug)]
pub struct LiveSharedPixmapUpdate {
    pub handle: BufferHandle,
    pub revision: u64,
    pub size: Size,
    pub format: u32,
    pub patches: Vec<LiveSharedPixmapPatch>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveSharedPixmapError {
    InvalidTarget,
    Capacity,
    IdentityInUse,
    UnknownBacking,
    DeviceRejected,
    ExportFailed,
    UploadFailed,
    Unavailable,
}

impl LiveSharedPixmapUpdate {
    pub fn validate(&self) -> Result<(), LiveSharedPixmapError> {
        if self.handle.raw() == 0
            || self.revision == 0
            || !matches!(
                self.format,
                sophia_protocol::DRM_FORMAT_XRGB8888 | sophia_protocol::DRM_FORMAT_ARGB8888
            )
        {
            return Err(LiveSharedPixmapError::InvalidTarget);
        }
        validate_patches(self.size, &self.patches)
    }
}

impl std::fmt::Display for LiveSharedPixmapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shared pixmap: {self:?}")
    }
}

impl std::error::Error for LiveSharedPixmapError {}

struct SharedPixmap {
    buffer: gbm::BufferObject<()>,
    descriptor: DmaBufDescriptor,
    sync_fd: OwnedFd,
    bytes: usize,
    revision: u64,
}

/// Renderer-owned GBM storage. Callers serialize access outside authority locks.
pub struct LiveSharedPixmapStore {
    device: gbm::Device<File>,
    buffers: BTreeMap<BufferHandle, SharedPixmap>,
    bytes: usize,
}

impl LiveSharedPixmapStore {
    pub fn new(device: File) -> Result<Self, LiveSharedPixmapError> {
        Ok(Self {
            device: gbm::Device::new(device).map_err(|_| LiveSharedPixmapError::DeviceRejected)?,
            buffers: BTreeMap::new(),
            bytes: 0,
        })
    }

    pub fn allocate(
        &mut self,
        handle: BufferHandle,
        size: Size,
        depth: u8,
    ) -> Result<LiveSharedBufferAllocation, LiveSharedPixmapError> {
        let bytes = pixmap_bytes(size)?;
        if handle.raw() == 0 {
            return Err(LiveSharedPixmapError::InvalidTarget);
        }
        if self.buffers.contains_key(&handle) {
            return Err(LiveSharedPixmapError::IdentityInUse);
        }
        if self.buffers.len() >= MAX_PIXMAPS || bytes > MAX_STORE_BYTES.saturating_sub(self.bytes) {
            return Err(LiveSharedPixmapError::Capacity);
        }
        let format = match depth {
            24 => gbm::Format::Xrgb8888,
            32 => gbm::Format::Argb8888,
            _ => return Err(LiveSharedPixmapError::InvalidTarget),
        };
        let mut buffer = self
            .device
            .create_buffer_object::<()>(
                size.width as u32,
                size.height as u32,
                format,
                gbm::BufferObjectFlags::RENDERING,
            )
            .map_err(|_| LiveSharedPixmapError::DeviceRejected)?;
        let plane_count = buffer.plane_count();
        if plane_count == 0 || plane_count as usize > sophia_protocol::DMA_BUF_MAX_PLANES {
            return Err(LiveSharedPixmapError::InvalidTarget);
        }
        let mut planes = [None; sophia_protocol::DMA_BUF_MAX_PLANES];
        let mut plane_fds = Vec::with_capacity(plane_count as usize);
        let mut allocated_bytes = 0usize;
        for index in 0..plane_count {
            let fd = buffer
                .fd_for_plane(index as i32)
                .map_err(|_| LiveSharedPixmapError::ExportFailed)?;
            let length = rustix::fs::fstat(&fd)
                .ok()
                .and_then(|stat| usize::try_from(stat.st_size).ok())
                .filter(|length| *length != 0)
                .ok_or(LiveSharedPixmapError::ExportFailed)?;
            allocated_bytes = allocated_bytes
                .checked_add(length)
                .ok_or(LiveSharedPixmapError::Capacity)?;
            planes[index as usize] = Some(DmaBufPlaneDescriptor {
                offset: buffer.offset(index as i32),
                stride: buffer.stride_for_plane(index as i32),
            });
            plane_fds.push(fd);
        }
        if allocated_bytes > MAX_PIXMAP_BYTES
            || allocated_bytes > MAX_STORE_BYTES.saturating_sub(self.bytes)
        {
            return Err(LiveSharedPixmapError::Capacity);
        }
        let sync_fd = plane_fds[0]
            .try_clone()
            .map_err(|_| LiveSharedPixmapError::ExportFailed)?;
        cpu_access(&sync_fd, false)?;
        let cleared = buffer
            .map_mut(0, 0, size.width as u32, size.height as u32, |mapped| {
                mapped.buffer_mut().fill(0);
            })
            .map_err(|_| LiveSharedPixmapError::UploadFailed);
        let ended = cpu_access(&sync_fd, true);
        cleared?;
        ended?;
        let descriptor = DmaBufDescriptor {
            handle,
            size,
            format: format as u32,
            modifier: u64::from(buffer.modifier()),
            plane_count: plane_count as u8,
            planes,
        };
        descriptor
            .validate()
            .map_err(|_| LiveSharedPixmapError::InvalidTarget)?;
        self.buffers.insert(
            handle,
            SharedPixmap {
                buffer,
                descriptor,
                sync_fd,
                bytes: allocated_bytes,
                revision: 0,
            },
        );
        self.bytes += allocated_bytes;
        Ok(LiveSharedBufferAllocation {
            descriptor,
            plane_fds,
        })
    }

    pub fn update(&mut self, update: LiveSharedPixmapUpdate) -> Result<(), LiveSharedPixmapError> {
        update.validate()?;
        let entry = self
            .buffers
            .get_mut(&update.handle)
            .ok_or(LiveSharedPixmapError::UnknownBacking)?;
        if update.size != entry.descriptor.size || update.format != entry.descriptor.format {
            return Err(LiveSharedPixmapError::InvalidTarget);
        }
        if update.revision <= entry.revision {
            return Ok(());
        }
        if !update.patches.is_empty() {
            cpu_access(&entry.sync_fd, false)?;
            let written = entry
                .buffer
                .map_mut(
                    0,
                    0,
                    update.size.width as u32,
                    update.size.height as u32,
                    |mapped| {
                        let stride = mapped.stride() as usize;
                        let target = mapped.buffer_mut();
                        if stride < update.size.width as usize * 4
                            || stride
                                .checked_mul(update.size.height as usize)
                                .is_none_or(|length| length > target.len())
                        {
                            return Err(LiveSharedPixmapError::InvalidTarget);
                        }
                        for patch in &update.patches {
                            let row_bytes = patch.rect.width as usize * 4;
                            for row in 0..patch.rect.height as usize {
                                let start = (patch.rect.y as usize + row) * stride
                                    + patch.rect.x as usize * 4;
                                let end = start
                                    .checked_add(row_bytes)
                                    .ok_or(LiveSharedPixmapError::InvalidTarget)?;
                                let dest = target
                                    .get_mut(start..end)
                                    .ok_or(LiveSharedPixmapError::InvalidTarget)?;
                                dest.copy_from_slice(
                                    &patch.bytes[row * row_bytes..(row + 1) * row_bytes],
                                );
                            }
                        }
                        Ok(())
                    },
                )
                .map_err(|_| LiveSharedPixmapError::UploadFailed);
            let ended = cpu_access(&entry.sync_fd, true);
            written??;
            ended?;
        }
        entry.revision = update.revision;
        Ok(())
    }

    pub fn release(&mut self, handle: BufferHandle) -> bool {
        if let Some(entry) = self.buffers.remove(&handle) {
            self.bytes -= entry.bytes;
            true
        } else {
            false
        }
    }

    pub fn resident_bytes(&self) -> usize {
        self.bytes
    }
    pub fn resident_count(&self) -> usize {
        self.buffers.len()
    }
}

fn pixmap_bytes(size: Size) -> Result<usize, LiveSharedPixmapError> {
    let width = usize::try_from(size.width).ok().filter(|v| *v != 0);
    let height = usize::try_from(size.height).ok().filter(|v| *v != 0);
    width
        .zip(height)
        .and_then(|(w, h)| w.checked_mul(h)?.checked_mul(4))
        .filter(|bytes| *bytes <= MAX_PIXMAP_BYTES)
        .ok_or(LiveSharedPixmapError::InvalidTarget)
}

fn validate_patches(
    size: Size,
    patches: &[LiveSharedPixmapPatch],
) -> Result<(), LiveSharedPixmapError> {
    pixmap_bytes(size)?;
    if patches.len() > MAX_PATCHES {
        return Err(LiveSharedPixmapError::Capacity);
    }
    let mut bytes = 0usize;
    for patch in patches {
        let r = patch.rect;
        if r.x < 0
            || r.y < 0
            || r.width <= 0
            || r.height <= 0
            || r.x
                .checked_add(r.width)
                .is_none_or(|right| right > size.width)
            || r.y
                .checked_add(r.height)
                .is_none_or(|bottom| bottom > size.height)
            || pixmap_bytes(Size {
                width: r.width,
                height: r.height,
            })? != patch.bytes.len()
        {
            return Err(LiveSharedPixmapError::InvalidTarget);
        }
        bytes = bytes
            .checked_add(patch.bytes.len())
            .ok_or(LiveSharedPixmapError::Capacity)?;
        if bytes > MAX_PIXMAP_BYTES {
            return Err(LiveSharedPixmapError::Capacity);
        }
    }
    Ok(())
}

fn cpu_access(fd: impl AsFd, end: bool) -> Result<(), LiveSharedPixmapError> {
    sophia_renderer_native_egl::native_dmabuf_cpu_write_access(fd, end)
        .map_err(|_| LiveSharedPixmapError::UploadFailed)
}
