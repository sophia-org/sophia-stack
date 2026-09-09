#![cfg(unix)]

use sophia_protocol::{DRM_FORMAT_ARGB8888, DRM_FORMAT_XRGB8888, NamespaceId, TransactionId};
use sophia_x_authority::*;
use std::{
    fs::File,
    io::{IoSliceMut, Read, Write},
    mem::MaybeUninit,
    os::{fd::OwnedFd, unix::net::UnixStream},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

fn row(format: u32, modifiers: Vec<u64>) -> XServerFrontendDmaBufImportFormat {
    XServerFrontendDmaBufImportFormat { format, modifiers }
}

#[test]
fn legacy_pixmap_export_encodes_the_drm_implicit_modifier() {
    for byte_order in [XByteOrder::LittleEndian, XByteOrder::BigEndian] {
        let namespace = NamespaceId::from_raw(51);
        let pixmap = XResourceId::new(0x200001, 1);
        let mut runtime = XAuthorityRuntime::new();
        runtime
            .create_dri3_pixmap(namespace, pixmap, 1, 256, 3, 1, 256, 32, 32)
            .unwrap();
        runtime
            .attach_dri3_plane_fds(
                namespace,
                pixmap,
                vec![Arc::new(File::open("/dev/null").unwrap().into())],
            )
            .unwrap();
        let result = dispatch_x11_wire_request(
            XDispatchContext {
                byte_order,
                namespace,
                transaction: TransactionId::from_raw(2),
                sequence: 2,
                major_opcode: X_DRI3_MAJOR_OPCODE,
                client_id: 1,
            },
            XWireRequest::Dri3BuffersFromPixmap { pixmap },
            &mut runtime,
            &mut XAtomTable::new(),
            &mut XPropertyTable::new(),
        );
        assert_eq!(result.outputs.len(), 1);
        let output = result.outputs.into_iter().next().unwrap();
        assert!(matches!(
            &output,
            XClientOutput::Reply(XClientReply::Dri3BuffersFromPixmap {
                modifier: 0x00ff_ffff_ffff_ffff,
                depth: 32,
                bits_per_pixel: 32,
                ..
            })
        ));
        let bytes = encode_x_client_output(byte_order, output);
        let expected = match byte_order {
            XByteOrder::LittleEndian => 0x00ff_ffff_ffff_ffff_u64.to_le_bytes(),
            XByteOrder::BigEndian => 0x00ff_ffff_ffff_ffff_u64.to_be_bytes(),
        };
        assert_eq!(&bytes[16..24], &expected);
    }
}

fn query(runtime: &mut XAuthorityRuntime, depth: u8) -> Vec<u64> {
    let result = dispatch_x11_wire_request(
        XDispatchContext {
            byte_order: XByteOrder::LittleEndian,
            namespace: NamespaceId::from_raw(51),
            transaction: TransactionId::from_raw(1),
            sequence: 1,
            major_opcode: X_DRI3_MAJOR_OPCODE,
            client_id: 1,
        },
        XWireRequest::Dri3GetSupportedModifiers {
            window: XResourceId::new(u64::from(X_SETUP_DEFAULT_ROOT), 1),
            depth,
            bits_per_pixel: 32,
        },
        runtime,
        &mut XAtomTable::new(),
        &mut XPropertyTable::new(),
    );
    assert_eq!(result.outputs.len(), 1);
    let XClientOutput::Reply(XClientReply::Dri3GetSupportedModifiers {
        window_modifiers,
        screen_modifiers,
        ..
    }) = &result.outputs[0]
    else {
        panic!("expected modifier reply");
    };
    assert!(window_modifiers.is_empty());
    screen_modifiers.clone()
}

#[test]
fn explicit_import_inventory_is_format_exact_canonical_and_immutable() {
    let mut runtime = XAuthorityRuntime::new();
    assert!(query(&mut runtime, 24).is_empty());
    runtime.set_dma_buf_import_formats(vec![
        row(
            DRM_FORMAT_XRGB8888,
            vec![7, 0, 7, 0x00ff_ffff_ffff_ffff, u64::MAX],
        ),
        row(DRM_FORMAT_ARGB8888, vec![9]),
        row(DRM_FORMAT_XRGB8888, vec![3]),
        row(0x12345678, vec![11]),
    ]);
    assert_eq!(query(&mut runtime, 24), vec![0, 3, 7]);
    assert_eq!(query(&mut runtime, 32), vec![9]);
    runtime.set_dma_buf_import_formats(vec![row(DRM_FORMAT_XRGB8888, vec![88])]);
    assert_eq!(query(&mut runtime, 24), vec![0, 3, 7]);

    let mut opaque_only = XAuthorityRuntime::new();
    opaque_only.set_dma_buf_import_formats(vec![row(DRM_FORMAT_XRGB8888, vec![5])]);
    assert!(query(&mut opaque_only, 32).is_empty());
}

#[test]
fn oversized_import_inventories_are_refused_before_deduplication() {
    for formats in [
        vec![row(DRM_FORMAT_XRGB8888, vec![0]); 513],
        vec![row(DRM_FORMAT_XRGB8888, vec![0; 16_385])],
        vec![
            row(DRM_FORMAT_XRGB8888, vec![0; 8192]),
            row(DRM_FORMAT_ARGB8888, vec![0; 8193]),
        ],
    ] {
        let mut runtime = XAuthorityRuntime::new();
        runtime.set_dma_buf_import_formats(formats);
        assert!(query(&mut runtime, 24).is_empty());
        assert!(query(&mut runtime, 32).is_empty());
    }
    let mut maximum = XAuthorityRuntime::new();
    maximum.set_dma_buf_import_formats(vec![row(DRM_FORMAT_XRGB8888, vec![0; 32]); 512]);
    assert_eq!(query(&mut maximum, 24), vec![0]);
}

struct Provider {
    modifiers: Vec<u64>,
    device: &'static str,
    snapshots: AtomicUsize,
    opens: AtomicUsize,
}

impl Provider {
    fn new(modifier: u64, device: &'static str) -> Arc<Self> {
        Arc::new(Self {
            modifiers: vec![modifier],
            device,
            snapshots: AtomicUsize::new(0),
            opens: AtomicUsize::new(0),
        })
    }
}

impl XServerFrontendRenderDeviceProvider for Provider {
    fn open_render_device_fd(&self) -> Result<OwnedFd, XServerFrontendRenderDeviceError> {
        self.opens.fetch_add(1, Ordering::SeqCst);
        File::open(self.device)
            .map(OwnedFd::from)
            .map_err(|_| XServerFrontendRenderDeviceError::OpenFailed)
    }
    fn dma_buf_import_formats(&self) -> Vec<XServerFrontendDmaBufImportFormat> {
        self.snapshots.fetch_add(1, Ordering::SeqCst);
        vec![row(DRM_FORMAT_XRGB8888, self.modifiers.clone())]
    }
}

fn setup(stream: &mut UnixStream) {
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    stream
        .set_write_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    stream
        .write_all(&[b'l', 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let mut header = [0; 8];
    stream.read_exact(&mut header).unwrap();
    assert_eq!(header[0], 1);
    let mut body = vec![0; usize::from(u16::from_le_bytes(header[6..8].try_into().unwrap())) * 4];
    stream.read_exact(&mut body).unwrap();
}

fn query_socket(stream: &mut UnixStream, depth: u8) -> Vec<u64> {
    let mut request = vec![
        X_DRI3_MAJOR_OPCODE,
        X_DRI3_GET_SUPPORTED_MODIFIERS_MINOR_OPCODE,
        3,
        0,
    ];
    request.extend_from_slice(&X_SETUP_DEFAULT_ROOT.to_le_bytes());
    request.extend_from_slice(&[depth, 32, 0, 0]);
    stream.write_all(&request).unwrap();
    let mut header = [0; 32];
    stream.read_exact(&mut header).unwrap();
    assert_eq!(header[0], 1);
    assert_eq!(&header[8..12], &[0; 4]);
    let count = u32::from_le_bytes(header[12..16].try_into().unwrap()) as usize;
    assert!(count <= 16_384);
    assert_eq!(
        u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize,
        count * 2
    );
    let mut body = vec![0; count * 8];
    stream.read_exact(&mut body).unwrap();
    body.chunks_exact(8)
        .map(|bytes| u64::from_le_bytes(bytes.try_into().unwrap()))
        .collect()
}

fn assert_open_uses_zero_device(stream: &mut UnixStream) {
    let mut request = vec![X_DRI3_MAJOR_OPCODE, X_DRI3_OPEN_MINOR_OPCODE, 3, 0];
    request.extend_from_slice(&X_SETUP_DEFAULT_ROOT.to_le_bytes());
    request.extend_from_slice(&0u32.to_le_bytes());
    stream.write_all(&request).unwrap();
    let mut reply = [0; 32];
    let mut vectors = [IoSliceMut::new(&mut reply)];
    let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(1))];
    let mut ancillary = rustix::net::RecvAncillaryBuffer::new(&mut space);
    let received = rustix::net::recvmsg(
        &*stream,
        &mut vectors,
        &mut ancillary,
        rustix::net::RecvFlags::CMSG_CLOEXEC,
    )
    .unwrap();
    let received_bytes = received.bytes;
    if received_bytes < 32 {
        stream.read_exact(&mut reply[received_bytes..]).unwrap();
    }
    assert_eq!(reply[0], 1);
    assert_eq!(reply[1], 1);
    let fds: Vec<_> = ancillary
        .drain()
        .flat_map(|message| match message {
            rustix::net::RecvAncillaryMessage::ScmRights(fds) => fds.collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .collect();
    assert_eq!(fds.len(), 1);
    let mut device = File::from(fds.into_iter().next().unwrap());
    let mut byte = [1];
    device.read_exact(&mut byte).unwrap();
    assert_eq!(byte, [0]);
}

#[test]
fn state_clones_share_the_first_provider_and_its_modifier_inventory() {
    let state = X11CoreSocketServerState::new();
    let early_clone = state.clone();
    let first = Provider::new(7, "/dev/zero");
    let second = Provider::new(9, "/dev/null");
    let _bound = state.with_render_device_provider(first.clone());
    let rebound = early_clone.with_render_device_provider(second.clone());
    let (mut client, mut server) = UnixStream::pair().unwrap();
    let worker = std::thread::spawn(move || {
        serve_x11_core_socket_client_with_state(&mut server, NamespaceId::from_raw(51), &rebound)
    });
    setup(&mut client);
    assert_eq!(query_socket(&mut client, 24), vec![7]);
    assert_eq!(query_socket(&mut client, 24), vec![7]);
    assert!(query_socket(&mut client, 32).is_empty());
    assert_open_uses_zero_device(&mut client);
    drop(client);
    worker.join().unwrap().unwrap();
    assert_eq!(first.snapshots.load(Ordering::SeqCst), 1);
    assert_eq!(first.opens.load(Ordering::SeqCst), 1);
    assert_eq!(second.opens.load(Ordering::SeqCst), 0);
}

#[test]
fn frontend_bind_latches_the_provider_inventory_without_request_callbacks() {
    let path = std::env::temp_dir().join(format!(
        "sophia-dri3-capability-{}.sock",
        std::process::id()
    ));
    let provider = Provider::new(13, "/dev/zero");
    let config = XServerFrontendConfig::new(&path, NamespaceId::from_raw(52))
        .unwrap()
        .with_render_device_provider(provider.clone());
    let mut frontend = XServerFrontend::bind(config).unwrap();
    let worker = std::thread::spawn(move || frontend.serve_next());
    let mut client = UnixStream::connect(&path).unwrap();
    setup(&mut client);
    assert_eq!(query_socket(&mut client, 24), vec![13]);
    assert_eq!(query_socket(&mut client, 24), vec![13]);
    assert!(query_socket(&mut client, 32).is_empty());
    drop(client);
    worker.join().unwrap().unwrap();
    assert_eq!(provider.snapshots.load(Ordering::SeqCst), 1);
    let _ = std::fs::remove_file(path);
}

#[test]
fn an_unmeasured_provider_keeps_open_without_inventing_explicit_layouts() {
    struct Unmeasured;
    impl XServerFrontendRenderDeviceProvider for Unmeasured {
        fn open_render_device_fd(&self) -> Result<OwnedFd, XServerFrontendRenderDeviceError> {
            File::open("/dev/zero")
                .map(OwnedFd::from)
                .map_err(|_| XServerFrontendRenderDeviceError::OpenFailed)
        }
    }
    let state = X11CoreSocketServerState::new().with_render_device_provider(Arc::new(Unmeasured));
    let (mut client, mut server) = UnixStream::pair().unwrap();
    let worker = std::thread::spawn(move || {
        serve_x11_core_socket_client_with_state(&mut server, NamespaceId::from_raw(53), &state)
    });
    setup(&mut client);
    assert!(query_socket(&mut client, 24).is_empty());
    assert!(query_socket(&mut client, 32).is_empty());
    assert_open_uses_zero_device(&mut client);
    drop(client);
    worker.join().unwrap().unwrap();
}
