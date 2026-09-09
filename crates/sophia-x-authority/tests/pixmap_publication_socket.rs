#![cfg(target_os = "linux")]

use sophia_protocol::{
    BufferHandle, ClientAdmissionContext, ClientAdmissionId, ClientAuthProvenance,
    DRM_FORMAT_ARGB8888, DmaBufDescriptor, DmaBufPlaneDescriptor, NamespaceCapabilities,
    NamespaceContext, NamespaceId, NamespaceProfile,
};
use sophia_x_authority::*;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{IoSliceMut, Read, Write},
    mem::MaybeUninit,
    os::{
        fd::OwnedFd,
        unix::{fs::FileExt, net::UnixStream},
    },
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc,
    },
    thread::JoinHandle,
    time::Duration,
};

const WAIT: Duration = Duration::from_secs(5);
static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Allocate,
    Update,
}

struct Gate {
    operation: Operation,
    entered: mpsc::SyncSender<()>,
    resume: mpsc::Receiver<()>,
}

struct BlockedCall {
    entered: mpsc::Receiver<()>,
    resume: mpsc::SyncSender<()>,
}

impl BlockedCall {
    fn wait(&self) {
        self.entered
            .recv_timeout(WAIT)
            .expect("provider call did not start");
    }
    fn release(&self) {
        let _ = self.resume.try_send(());
    }
}

impl Drop for BlockedCall {
    fn drop(&mut self) {
        self.release();
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Release {
    Refused(BufferHandle),
    Completed(BufferHandle),
}

#[derive(Default)]
struct ProviderState {
    buffers: BTreeMap<BufferHandle, File>,
    updates: Vec<XServerFrontendPixmapUpdate>,
    releases: Vec<BufferHandle>,
}

struct Provider {
    state: Mutex<ProviderState>,
    gate: Mutex<Option<Gate>>,
    refuse_release: AtomicBool,
    always_refuse_release: AtomicBool,
    refuse_update: AtomicBool,
    release_events: mpsc::Sender<Release>,
}

impl Provider {
    fn new() -> (Arc<Self>, mpsc::Receiver<Release>) {
        let (release_events, events) = mpsc::channel();
        (
            Arc::new(Self {
                state: Mutex::new(ProviderState::default()),
                gate: Mutex::new(None),
                refuse_release: AtomicBool::new(false),
                always_refuse_release: AtomicBool::new(false),
                refuse_update: AtomicBool::new(false),
                release_events,
            }),
            events,
        )
    }

    fn block(&self, operation: Operation) -> BlockedCall {
        let (entered, observed) = mpsc::sync_channel(1);
        let (resume, released) = mpsc::sync_channel(1);
        *self.gate.lock().unwrap() = Some(Gate {
            operation,
            entered,
            resume: released,
        });
        BlockedCall {
            entered: observed,
            resume,
        }
    }

    fn wait_if_blocked(&self, operation: Operation) {
        let gate = {
            let mut pending = self.gate.lock().unwrap();
            if pending
                .as_ref()
                .is_some_and(|gate| gate.operation == operation)
            {
                pending.take()
            } else {
                None
            }
        };
        if let Some(gate) = gate {
            gate.entered.send(()).unwrap();
            gate.resume
                .recv_timeout(WAIT)
                .expect("provider gate was not released");
        }
    }
}

impl XServerFrontendPixmapAllocator for Provider {
    fn supports_pixmap_textures(&self) -> bool {
        true
    }

    fn allocate_pixmap_buffer(
        &self,
        request: XServerFrontendPixmapAllocation,
    ) -> Result<XServerFrontendAllocatedPixmap, XServerFrontendPixmapAllocationError> {
        self.wait_if_blocked(Operation::Allocate);
        assert_eq!(request.depth, 32);
        let handle = BufferHandle::from_raw(request.handle);
        let file = File::from(
            rustix::fs::memfd_create(
                c"sophia-pixmap-publication",
                rustix::fs::MemfdFlags::CLOEXEC,
            )
            .unwrap(),
        );
        file.set_len(u64::try_from(request.size.width * request.size.height * 4).unwrap())
            .unwrap();
        let descriptor = DmaBufDescriptor {
            handle,
            size: request.size,
            format: DRM_FORMAT_ARGB8888,
            modifier: 0,
            plane_count: 1,
            planes: [
                Some(DmaBufPlaneDescriptor {
                    offset: 0,
                    stride: u32::try_from(request.size.width * 4).unwrap(),
                }),
                None,
                None,
                None,
            ],
        };
        let plane_fds = vec![OwnedFd::from(file.try_clone().unwrap())];
        assert!(
            self.state
                .lock()
                .unwrap()
                .buffers
                .insert(handle, file)
                .is_none()
        );
        Ok(XServerFrontendAllocatedPixmap {
            descriptor,
            plane_fds,
        })
    }

    fn update_pixmap_buffer(
        &self,
        request: XServerFrontendPixmapUpdate,
    ) -> Result<(), XServerFrontendPixmapAllocationError> {
        self.wait_if_blocked(Operation::Update);
        if self.refuse_update.swap(false, Ordering::AcqRel) {
            return Err(XServerFrontendPixmapAllocationError::Unavailable);
        }
        let mut state = self.state.lock().unwrap();
        let file = state
            .buffers
            .get(&request.handle)
            .ok_or(XServerFrontendPixmapAllocationError::UnknownBacking)?;
        for patch in &request.patches {
            let row_bytes = usize::try_from(patch.rect.width * 4).unwrap();
            for (row, bytes) in patch.bytes.chunks_exact(row_bytes).enumerate() {
                let offset = ((i64::from(patch.rect.y) + i64::try_from(row).unwrap())
                    * i64::from(request.size.width)
                    + i64::from(patch.rect.x))
                    * 4;
                file.write_all_at(bytes, u64::try_from(offset).unwrap())
                    .unwrap();
            }
        }
        state.updates.push(request);
        Ok(())
    }

    fn release_pixmap_buffer(
        &self,
        handle: BufferHandle,
    ) -> Result<(), XServerFrontendPixmapAllocationError> {
        if self.always_refuse_release.load(Ordering::Acquire)
            || self.refuse_release.swap(false, Ordering::AcqRel) {
            let _ = self.release_events.send(Release::Refused(handle));
            return Err(XServerFrontendPixmapAllocationError::Unavailable);
        }
        let mut state = self.state.lock().unwrap();
        assert!(
            state.buffers.remove(&handle).is_some(),
            "release must name a live provider allocation"
        );
        state.releases.push(handle);
        let _ = self.release_events.send(Release::Completed(handle));
        Ok(())
    }
}

impl XServerFrontendRenderDeviceProvider for Provider {
    fn open_render_device_fd(&self) -> Result<OwnedFd, XServerFrontendRenderDeviceError> {
        File::open("/dev/zero")
            .map(OwnedFd::from)
            .map_err(|_| XServerFrontendRenderDeviceError::OpenFailed)
    }
    fn dma_buf_import_formats(&self) -> Vec<XServerFrontendDmaBufImportFormat> {
        vec![XServerFrontendDmaBufImportFormat {
            format: DRM_FORMAT_ARGB8888,
            modifiers: vec![0],
        }]
    }
}

struct SeparateNamespaces {
    next: AtomicU64,
    shared: bool,
}

impl XServerFrontendAdmissionPolicy for SeparateNamespaces {
    fn admit(
        &self,
        request: XServerFrontendAdmissionRequest,
    ) -> Result<ClientAdmissionContext, XServerFrontendAdmissionError> {
        let next = self.next.fetch_add(1, Ordering::Relaxed) + 1;
        ClientAdmissionContext::new(
            ClientAdmissionId::from_raw(next),
            NamespaceContext::new(
                NamespaceId::from_raw(917 + if self.shared { 0 } else { next }),
                NamespaceProfile::Confined,
                NamespaceCapabilities::NONE,
            )
            .unwrap(),
            ClientAuthProvenance::new(request.setup_authentication, 1).unwrap(),
        )
        .ok_or(XServerFrontendAdmissionError::Unavailable)
    }

    fn revoke(
        &self,
        _context: ClientAdmissionContext,
    ) -> Result<(), XServerFrontendAdmissionError> {
        Ok(())
    }
}

enum Command {
    Accept(mpsc::SyncSender<()>),
    Install(
        Arc<XServerFrontendDeviceBundle>,
        mpsc::SyncSender<Result<(), XServerFrontendDeviceBundleError>>,
    ),
    Lose(
        u64,
        mpsc::SyncSender<Result<(), XServerFrontendDeviceBundleError>>,
    ),
    Stop,
}

struct Fixture {
    path: PathBuf,
    commands: mpsc::Sender<Command>,
    worker: Option<JoinHandle<Result<(), X11SetupSocketError>>>,
    provider: Arc<Provider>,
    releases: mpsc::Receiver<Release>,
}

impl Fixture {
    fn new() -> Self {
        Self::with_shared_namespace(false)
    }

    fn with_shared_namespace(shared: bool) -> Self {
        let path = std::env::temp_dir().join(format!(
            "sophia-pixmap-publication-{}-{}.sock",
            std::process::id(),
            NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
        ));
        let (provider, releases) = Provider::new();
        let config = XServerFrontendConfig::new(&path, NamespaceId::from_raw(917))
            .unwrap()
            .with_device_bundle(Arc::new(
                XServerFrontendDeviceBundle::new(1, provider.clone(), Some(provider.clone()))
                    .unwrap(),
            ))
            .with_admission_policy(Arc::new(SeparateNamespaces {
                next: AtomicU64::new(0),
                shared,
            }));
        let mut frontend = XServerFrontend::bind(config).unwrap();
        let (commands, incoming) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            loop {
                match incoming.recv_timeout(Duration::from_millis(1)) {
                    Ok(Command::Accept(accepted)) => {
                        frontend.serve_next_concurrently()?;
                        let _ = accepted.send(());
                    }
                    Ok(Command::Install(bundle, reply)) => {
                        let _ = reply.send(frontend.install_device_bundle(bundle));
                    }
                    Ok(Command::Lose(generation, reply)) => {
                        let _ = reply.send(frontend.mark_device_generation_unavailable(generation));
                    }
                    Ok(Command::Stop) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                }
                frontend.poll_client_workers()?;
            }
            frontend.shutdown_all_client_workers()?;
            frontend.wait_for_clients()
        });
        Self {
            path,
            commands,
            worker: Some(worker),
            provider,
            releases,
        }
    }

    fn install(&self, generation: u64, provider: Arc<Provider>) {
        let bundle = Arc::new(
            XServerFrontendDeviceBundle::new(generation, provider.clone(), Some(provider)).unwrap(),
        );
        let (reply, receiver) = mpsc::sync_channel(1);
        self.commands.send(Command::Install(bundle, reply)).unwrap();
        receiver.recv_timeout(WAIT).unwrap().unwrap();
    }

    fn lose(&self, generation: u64) {
        let (reply, receiver) = mpsc::sync_channel(1);
        self.commands
            .send(Command::Lose(generation, reply))
            .unwrap();
        receiver.recv_timeout(WAIT).unwrap().unwrap();
    }

    fn connect(&self) -> Client {
        let mut stream = UnixStream::connect(&self.path).unwrap();
        stream.set_read_timeout(Some(WAIT)).unwrap();
        stream.set_write_timeout(Some(WAIT)).unwrap();
        let (accepted, acknowledgment) = mpsc::sync_channel(1);
        self.commands.send(Command::Accept(accepted)).unwrap();
        acknowledgment.recv_timeout(WAIT).unwrap();
        stream
            .write_all(&[b'l', 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0])
            .unwrap();
        let mut header = [0; 8];
        stream.read_exact(&mut header).unwrap();
        assert_eq!(header[0], 1);
        let mut body =
            vec![0; usize::from(u16::from_le_bytes(header[6..8].try_into().unwrap())) * 4];
        stream.read_exact(&mut body).unwrap();
        Client {
            stream,
            sequence: 0,
            base: u32::from_le_bytes(body[4..8].try_into().unwrap()),
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = self.commands.send(Command::Stop);
        if let Some(worker) = self.worker.take() {
            let result = worker.join();
            if !std::thread::panicking() {
                result.unwrap().unwrap();
            }
        }
        let _ = std::fs::remove_file(&self.path);
    }
}

struct Client {
    stream: UnixStream,
    sequence: u16,
    base: u32,
}

impl Client {
    fn request(&mut self, opcode: u8, minor: u8, body: &[u8]) -> u16 {
        assert_eq!(body.len() % 4, 0);
        let mut request = vec![opcode, minor];
        request.extend_from_slice(&u16::try_from((body.len() + 4) / 4).unwrap().to_le_bytes());
        request.extend_from_slice(body);
        self.stream.write_all(&request).unwrap();
        self.sequence += 1;
        self.sequence
    }

    fn reply(&mut self, sequence: u16) -> (Vec<u8>, Vec<OwnedFd>) {
        let mut reply = vec![0; 32];
        let mut ancillary_space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(4))];
        let mut ancillary = rustix::net::RecvAncillaryBuffer::new(&mut ancillary_space);
        let received = rustix::net::recvmsg(
            &self.stream,
            &mut [IoSliceMut::new(&mut reply)],
            &mut ancillary,
            rustix::net::RecvFlags::CMSG_CLOEXEC,
        )
        .unwrap();
        assert!(received.bytes > 0);
        if received.bytes < 32 {
            self.stream
                .read_exact(&mut reply[received.bytes..])
                .unwrap();
        }
        let fds = ancillary
            .drain()
            .flat_map(|message| match message {
                rustix::net::RecvAncillaryMessage::ScmRights(fds) => fds.collect::<Vec<_>>(),
                _ => Vec::new(),
            })
            .collect();
        assert_eq!(
            reply[0], 1,
            "expected reply for sequence {sequence}: {reply:?}"
        );
        assert_eq!(
            u16::from_le_bytes(reply[2..4].try_into().unwrap()),
            sequence
        );
        let extra =
            usize::try_from(u32::from_le_bytes(reply[4..8].try_into().unwrap())).unwrap() * 4;
        reply.resize(32 + extra, 0);
        self.stream.read_exact(&mut reply[32..]).unwrap();
        (reply, fds)
    }

    fn sync(&mut self) {
        let sequence = self.request(43, 0, &[]);
        let (_, fds) = self.reply(sequence);
        assert!(fds.is_empty());
    }

    fn create(&mut self) -> (u32, u32) {
        let pixmap = self.base + 1;
        let gc = self.base + 2;
        let mut body = words(&[pixmap, X_SETUP_DEFAULT_ROOT]);
        body.extend_from_slice(&3u16.to_le_bytes());
        body.extend_from_slice(&1u16.to_le_bytes());
        self.request(53, 32, &body);
        self.request(55, 0, &words(&[gc, pixmap, 1 << 2, 0xff654321]));
        self.put_pixels(pixmap, gc, 0, &[0xff654321; 3]);
        (pixmap, gc)
    }

    fn put_pixels(&mut self, pixmap: u32, gc: u32, x: u16, pixels: &[u32]) {
        let mut body = words(&[pixmap, gc]);
        for value in [u16::try_from(pixels.len()).unwrap(), 1, x, 0] {
            body.extend_from_slice(&value.to_le_bytes());
        }
        body.extend_from_slice(&[0, 32, 0, 0]);
        body.extend_from_slice(&words(pixels));
        self.request(72, 2, &body);
    }

    fn fill(&mut self, pixmap: u32, gc: u32, x: u16, width: u16) {
        let mut body = words(&[pixmap, gc]);
        for value in [x, 0, width, 1] {
            body.extend_from_slice(&value.to_le_bytes());
        }
        self.request(70, 0, &body);
    }

    fn cpu_pixels(&mut self, pixmap: u32) -> [u32; 3] {
        let mut body = words(&[pixmap]);
        for value in [0u16, 0, 3, 1] {
            body.extend_from_slice(&value.to_le_bytes());
        }
        body.extend_from_slice(&u32::MAX.to_le_bytes());
        let sequence = self.request(73, 2, &body);
        let (reply, fds) = self.reply(sequence);
        assert!(fds.is_empty());
        assert_eq!(reply.len(), 44);
        std::array::from_fn(|index| {
            u32::from_le_bytes(reply[32 + index * 4..36 + index * 4].try_into().unwrap())
        })
    }

    fn export_request(&mut self, pixmap: u32) -> u16 {
        self.request(
            X_DRI3_MAJOR_OPCODE,
            X_DRI3_BUFFER_FROM_PIXMAP_MINOR_OPCODE,
            &words(&[pixmap]),
        )
    }

    fn export(&mut self, pixmap: u32) -> File {
        let sequence = self.export_request(pixmap);
        let (reply, mut fds) = self.reply(sequence);
        assert_eq!(reply[1], 1);
        assert_eq!(fds.len(), 1);
        File::from(fds.remove(0))
    }

    fn wrap_glx(&mut self, pixmap: u32) -> u32 {
        let alias = self.base + 3;
        self.request(
            X_GLX_MAJOR_OPCODE,
            X_GLX_CREATE_PIXMAP_MINOR_OPCODE,
            &words(&[0, 2, pixmap, alias, 0]),
        );
        self.sync();
        alias
    }
}

fn words(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn pixels(file: &File) -> [u32; 3] {
    let mut bytes = [0; 12];
    file.read_exact_at(&mut bytes, 0).unwrap();
    std::array::from_fn(|index| {
        u32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap())
    })
}

#[test]
fn drawing_is_published_before_export_and_later_sync_without_another_export() {
    let fixture = Fixture::new();
    let mut client = fixture.connect();
    let (pixmap, gc) = client.create();
    assert_eq!(
        client.cpu_pixels(pixmap),
        [0xff654321; 3],
        "the CPU drawing must exist before export"
    );
    let backing = client.export(pixmap);
    assert_eq!(
        pixels(&backing),
        [0xff654321; 3],
        "first export must contain earlier drawing"
    );
    client.request(56, 0, &words(&[gc, 1 << 2, 0xffabcdef]));
    client.put_pixels(pixmap, gc, 1, &[0xffabcdef]);
    client.sync();
    assert_eq!(
        pixels(&backing),
        [0xff654321, 0xffabcdef, 0xff654321],
        "a sync reply must publish later partial damage to the already exported fd"
    );
    let state = fixture.provider.state.lock().unwrap();
    assert_eq!(state.buffers.len(), 1);
    assert!(state.updates.len() >= 2);
    assert!(
        state
            .updates
            .windows(2)
            .all(|updates| updates[0].revision < updates[1].revision)
    );
}

#[test]
fn a_retained_glx_backing_is_released_once_after_its_last_alias() {
    let fixture = Fixture::new();
    let mut client = fixture.connect();
    let (pixmap, _) = client.create();
    let backing = client.export(pixmap);
    let alias = client.wrap_glx(pixmap);
    client.request(54, 0, &words(&[pixmap]));
    client.sync();
    assert!(fixture.provider.state.lock().unwrap().releases.is_empty());
    assert_eq!(pixels(&backing), [0xff654321; 3]);
    client.request(
        X_GLX_MAJOR_OPCODE,
        X_GLX_DESTROY_PIXMAP_MINOR_OPCODE,
        &words(&[alias]),
    );
    client.sync();
    let handle = match fixture.releases.recv_timeout(WAIT).unwrap() {
        Release::Completed(handle) => handle,
        other => panic!("unexpected release: {other:?}"),
    };
    client.sync();
    assert_eq!(fixture.provider.state.lock().unwrap().releases, [handle]);
    assert_eq!(
        pixels(&backing),
        [0xff654321; 3],
        "the exported descriptor retains storage independently"
    );
}

#[test]
fn disconnect_retries_a_refused_provider_release_during_frontend_poll() {
    let fixture = Fixture::new();
    let mut client = fixture.connect();
    let (pixmap, _) = client.create();
    let _backing = client.export(pixmap);
    fixture
        .provider
        .refuse_release
        .store(true, Ordering::Release);
    drop(client);
    let handle = match fixture.releases.recv_timeout(WAIT).unwrap() {
        Release::Refused(handle) => handle,
        other => panic!("expected first release refusal: {other:?}"),
    };
    assert_eq!(
        fixture.releases.recv_timeout(WAIT).unwrap(),
        Release::Completed(handle)
    );
    assert_eq!(fixture.provider.state.lock().unwrap().releases, [handle]);
}

#[test]
fn provider_allocation_and_upload_leave_other_clients_runtime_access() {
    for operation in [Operation::Allocate, Operation::Update] {
        let fixture = Fixture::new();
        let mut first = fixture.connect();
        let mut second = fixture.connect();
        let (pixmap, gc) = first.create();
        let retained = if operation == Operation::Update {
            Some(first.export(pixmap))
        } else {
            None
        };
        let blocked = fixture.provider.block(operation);
        let sequence = if operation == Operation::Allocate {
            first.export_request(pixmap)
        } else {
            first.request(56, 0, &words(&[gc, 1 << 2, 0xffabcdef]));
            first.put_pixels(pixmap, gc, 1, &[0xffabcdef]);
            first.request(43, 0, &[])
        };
        blocked.wait();
        // The independent namespace must answer before the provider's gate expires.
        second
            .stream
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        second.sync();
        blocked.release();
        let (_, fds) = first.reply(sequence);
        if operation == Operation::Allocate {
            assert_eq!(fds.len(), 1);
        } else {
            assert!(fds.is_empty());
            assert_eq!(
                pixels(retained.as_ref().unwrap()),
                [0xff654321, 0xffabcdef, 0xff654321]
            );
        }
    }
}

#[test]
fn pixmap_rectangle_fill_produces_pixels_before_export() {
    let fixture = Fixture::new();
    let mut client = fixture.connect();
    let (pixmap, gc) = client.create();
    client.request(56, 0, &words(&[gc, 1 << 2, 0xffabcdef]));
    client.fill(pixmap, gc, 1, 1);
    assert_eq!(
        client.cpu_pixels(pixmap),
        [0xff654321, 0xffabcdef, 0xff654321]
    );
    let backing = client.export(pixmap);
    assert_eq!(pixels(&backing), [0xff654321, 0xffabcdef, 0xff654321]);
}

#[test]
fn allocation_finishing_after_replacement_keeps_its_original_provider() {
    let fixture = Fixture::new();
    let mut old = fixture.connect();
    let (pixmap, gc) = old.create();
    let gate = fixture.provider.block(Operation::Allocate);
    let sequence = old.export_request(pixmap);
    gate.wait();
    let (replacement, replacement_releases) = Provider::new();
    fixture.install(2, replacement.clone());
    let mut new = fixture.connect();
    let (other, _) = new.create();
    let other_backing = new.export(other);
    gate.release();
    let (_, fds) = old.reply(sequence);
    assert_eq!(fds.len(), 1);
    let backing = File::from(fds.into_iter().next().unwrap());
    old.put_pixels(pixmap, gc, 1, &[0xffabcdef]);
    old.sync();
    assert_eq!(pixels(&backing), [0xff654321, 0xffabcdef, 0xff654321]);
    assert_eq!(pixels(&other_backing), [0xff654321; 3]);
    let old_handle = *fixture
        .provider
        .state
        .lock()
        .unwrap()
        .buffers
        .keys()
        .next()
        .unwrap();
    let new_handle = *replacement
        .state
        .lock()
        .unwrap()
        .buffers
        .keys()
        .next()
        .unwrap();
    assert_ne!(
        old_handle, new_handle,
        "provider replacement must not reset buffer identity"
    );
    fixture
        .provider
        .refuse_release
        .store(true, Ordering::Release);
    drop(old);
    assert_eq!(
        fixture.releases.recv_timeout(WAIT).unwrap(),
        Release::Refused(old_handle)
    );
    assert_eq!(
        fixture.releases.recv_timeout(WAIT).unwrap(),
        Release::Completed(old_handle)
    );
    assert!(
        replacement
            .state
            .lock()
            .unwrap()
            .buffers
            .contains_key(&new_handle)
    );
    assert!(replacement_releases.try_recv().is_err());
    drop(new);
    assert_eq!(
        replacement_releases.recv_timeout(WAIT).unwrap(),
        Release::Completed(new_handle)
    );
}

#[test]
fn namespace_publication_routes_each_backing_to_its_own_generation() {
    let fixture = Fixture::with_shared_namespace(true);
    let mut old = fixture.connect();
    let (old_pixmap, old_gc) = old.create();
    let old_backing = old.export(old_pixmap);
    let (replacement, _) = Provider::new();
    fixture.install(2, replacement.clone());
    let mut new = fixture.connect();
    let (new_pixmap, new_gc) = new.create();
    let new_backing = new.export(new_pixmap);
    fixture
        .provider
        .refuse_update
        .store(true, Ordering::Release);
    old.put_pixels(old_pixmap, old_gc, 1, &[0xffabcdef]);
    let mut error = [0; 32];
    old.stream.read_exact(&mut error).unwrap();
    assert_eq!(
        error[0], 0,
        "the first upload must leave a refused publication obligation"
    );
    new.put_pixels(new_pixmap, new_gc, 0, &[0xff112233]);
    new.sync();
    assert_eq!(pixels(&old_backing), [0xff654321, 0xffabcdef, 0xff654321]);
    assert_eq!(pixels(&new_backing), [0xff112233, 0xff654321, 0xff654321]);
    assert!(!fixture.provider.state.lock().unwrap().updates.is_empty());
    assert!(!replacement.state.lock().unwrap().updates.is_empty());
}

#[test]
fn device_loss_refuses_new_allocations_but_keeps_existing_release_ownership() {
    let fixture = Fixture::new();
    let mut old = fixture.connect();
    let (pixmap, _) = old.create();
    let retained = old.export(pixmap);
    let handle = *fixture
        .provider
        .state
        .lock()
        .unwrap()
        .buffers
        .keys()
        .next()
        .unwrap();
    fixture.lose(1);
    // A second drawable has no provider allocation yet.
    let second = old.base + 71;
    let mut body = words(&[second, X_SETUP_DEFAULT_ROOT]);
    body.extend_from_slice(&3u16.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes());
    old.request(53, 32, &body);
    old.export_request(second);
    let mut error = [0; 32];
    old.stream.read_exact(&mut error).unwrap();
    assert_eq!(error[0], 0, "loss must refuse a new provider allocation");
    assert_eq!(fixture.provider.state.lock().unwrap().buffers.len(), 1);
    assert_eq!(pixels(&retained), [0xff654321; 3]);
    drop(old);
    assert_eq!(
        fixture.releases.recv_timeout(WAIT).unwrap(),
        Release::Completed(handle)
    );
}

#[test]
fn a_lost_provider_does_not_block_cleanup_of_a_healthy_generation() {
    let fixture = Fixture::new();
    let mut old = fixture.connect();
    let (pixmap, _) = old.create();
    let _old_backing = old.export(pixmap);
    fixture.provider.always_refuse_release.store(true, Ordering::Release);
    drop(old);
    assert!(matches!(fixture.releases.recv_timeout(WAIT).unwrap(), Release::Refused(_)));
    let (replacement, releases) = Provider::new();
    fixture.install(2, replacement.clone());
    let mut new = fixture.connect();
    let (pixmap, _) = new.create();
    let _new_backing = new.export(pixmap);
    let handle = *replacement.state.lock().unwrap().buffers.keys().next().unwrap();
    drop(new);
    assert_eq!(releases.recv_timeout(WAIT).unwrap(), Release::Completed(handle));
    assert!(replacement.state.lock().unwrap().buffers.is_empty());
    assert!(!fixture.provider.state.lock().unwrap().buffers.is_empty());
    fixture.provider.always_refuse_release.store(false, Ordering::Release);
}
