#![cfg(unix)]

use sophia_protocol::{DRM_FORMAT_XRGB8888, NamespaceId, SurfaceId};
use sophia_x_authority::*;
use std::{
    fs::File,
    io::{Read, Write},
    os::{fd::OwnedFd, unix::net::UnixStream},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    time::Duration,
};

const TILED: u64 = 0x0200_0000_0040_1b03;

struct Provider {
    identity: Option<XRenderDeviceIdentity>,
    modifiers: Vec<u64>,
    observations: AtomicUsize,
}
impl XServerFrontendRenderDeviceProvider for Provider {
    fn open_render_device_fd(&self) -> Result<OwnedFd, XServerFrontendRenderDeviceError> {
        Ok(File::open("/dev/null").unwrap().into())
    }
    fn render_device_identity(&self) -> Option<XRenderDeviceIdentity> {
        self.observations.fetch_add(1, Ordering::SeqCst);
        self.identity
    }
    fn dma_buf_import_formats(&self) -> Vec<XServerFrontendDmaBufImportFormat> {
        vec![XServerFrontendDmaBufImportFormat {
            format: DRM_FORMAT_XRGB8888,
            modifiers: self.modifiers.clone(),
        }]
    }
}

fn identity() -> XRenderDeviceIdentity {
    let stat = rustix::fs::fstat(File::open("/dev/null").unwrap()).unwrap();
    XRenderDeviceIdentity {
        device: stat.st_dev,
        inode: stat.st_ino,
        device_number: stat.st_rdev,
    }
}

struct Client {
    socket: UnixStream,
    window: u32,
    surface: SurfaceId,
}

fn client(frontend: &mut XServerFrontend) -> Client {
    let (sent, observed) = mpsc::sync_channel(1);
    let mut socket = UnixStream::connect(frontend.config().socket_path()).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    frontend
        .serve_next_concurrently_traced(Arc::new(move |trace| {
            if trace.major_opcode == 1 {
                assert!(
                    trace.failure.is_none(),
                    "create failed: {:?}",
                    trace.failure
                );
                let response = trace.result.response.as_ref().expect("create response");
                assert_eq!(
                    response.outcome,
                    XAuthorityResponseOutcome::Accepted,
                    "{response:?}"
                );
                sent.send(response.surfaces[0].surface).unwrap();
            }
            Ok(None)
        }))
        .unwrap();
    socket
        .write_all(&[b'l', 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let mut header = [0; 8];
    socket.read_exact(&mut header).unwrap();
    assert_eq!(header[0], 1);
    let mut setup = vec![0; usize::from(u16::from_le_bytes(header[6..8].try_into().unwrap())) * 4];
    socket.read_exact(&mut setup).unwrap();
    let window = u32::from_le_bytes(setup[4..8].try_into().unwrap()) | 1;
    let mut request = vec![1, 0, 8, 0];
    request.extend(window.to_le_bytes());
    request.extend(X_SETUP_DEFAULT_ROOT.to_le_bytes());
    request.extend([0; 4]); // x, y
    request.extend(32u16.to_le_bytes());
    request.extend(32u16.to_le_bytes());
    request.extend([0; 12]); // border, class, visual, value mask
    socket.write_all(&request).unwrap();
    let surface = observed.recv_timeout(Duration::from_secs(3)).unwrap();
    Client {
        socket,
        window,
        surface,
    }
}

fn query(client: &mut Client) -> (Vec<u64>, Vec<u64>) {
    let mut request = vec![
        X_DRI3_MAJOR_OPCODE,
        X_DRI3_GET_SUPPORTED_MODIFIERS_MINOR_OPCODE,
        3,
        0,
    ];
    request.extend(client.window.to_le_bytes());
    request.extend([24, 32, 0, 0]);
    client.socket.write_all(&request).unwrap();
    let mut header = [0; 32];
    client.socket.read_exact(&mut header).unwrap();
    assert_eq!(header[0], 1, "modifier reply: {header:?}");
    let count = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
    let windows = u32::from_le_bytes(header[8..12].try_into().unwrap()) as usize;
    let screens = u32::from_le_bytes(header[12..16].try_into().unwrap()) as usize;
    assert_eq!(count, (windows + screens) * 2);
    assert!(count <= 8);
    let mut bytes = vec![0; count * 4];
    client.socket.read_exact(&mut bytes).unwrap();
    let modifiers: Vec<_> = bytes
        .chunks_exact(8)
        .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
        .collect();
    (modifiers[..windows].to_vec(), modifiers[windows..].to_vec())
}

fn hint(client: &mut Client, number: u64) {
    let mut request = vec![
        X_DRI3_MAJOR_OPCODE,
        X_DRI3_SET_DRM_DEVICE_IN_USE_MINOR_OPCODE,
        4,
        0,
    ];
    request.extend(client.window.to_le_bytes());
    request.extend(rustix::fs::major(number).to_le_bytes());
    request.extend(rustix::fs::minor(number).to_le_bytes());
    client.socket.write_all(&request).unwrap();
}

fn preference(
    surface: SurfaceId,
    identity: Option<XRenderDeviceIdentity>,
) -> XWindowAllocationPreference {
    let number = identity.map_or_else(
        || self::identity().device_number,
        |identity| identity.device_number,
    );
    XWindowAllocationPreference {
        surface,
        identity,
        device: XDrmDeviceHint {
            major: rustix::fs::major(number),
            minor: rustix::fs::minor(number),
        },
        formats: vec![XServerFrontendDmaBufImportFormat {
            format: DRM_FORMAT_XRGB8888,
            modifiers: vec![0, TILED],
        }],
    }
}

#[test]
fn tiled_preferences_require_exact_available_connection_identity() {
    let first = identity();
    let provider = Arc::new(Provider {
        identity: Some(first),
        modifiers: vec![0, TILED],
        observations: AtomicUsize::new(0),
    });
    let path =
        std::env::temp_dir().join(format!("sophia-window-device-{}.sock", std::process::id()));
    let config = XServerFrontendConfig::new(&path, NamespaceId::from_raw(83))
        .unwrap()
        .with_device_bundle(Arc::new(
            XServerFrontendDeviceBundle::new(1, provider.clone(), None).unwrap(),
        ));
    let topology_generation = config.output_topology().generation;
    let mut frontend = XServerFrontend::bind(config).unwrap();
    let mut old = client(&mut frontend);
    let mut generation = 0;
    let mut publish = |frontend: &XServerFrontend, windows| {
        generation += 1;
        assert_eq!(
            frontend
                .update_window_allocation_preferences(XWindowAllocationPreferences {
                    generation,
                    topology_generation,
                    windows,
                })
                .unwrap(),
            XWindowAllocationUpdate::Applied
        );
    };
    publish(
        &frontend,
        vec![preference(
            old.surface,
            Some(XRenderDeviceIdentity {
                inode: first.inode + 1,
                ..first
            }),
        )],
    );
    assert_eq!(
        query(&mut old),
        (vec![0], vec![0, TILED]),
        "no client hint is not identity proof"
    );
    for candidate in [
        Some(first),
        None,
        Some(XRenderDeviceIdentity {
            inode: first.inode + 1,
            ..first
        }),
        Some(XRenderDeviceIdentity {
            device: first.device + 1,
            ..first
        }),
        Some(XRenderDeviceIdentity {
            device_number: rustix::fs::makedev(226, 129),
            ..first
        }),
    ] {
        publish(&frontend, vec![preference(old.surface, candidate)]);
        // Even a matching client hint cannot turn a different node into identity evidence.
        hint(&mut old, candidate.unwrap_or(first).device_number);
        let expected = if candidate == Some(first) {
            vec![0, TILED]
        } else {
            vec![0]
        };
        assert_eq!(query(&mut old), (expected, vec![0, TILED]));
    }
    publish(&frontend, vec![preference(old.surface, Some(first))]);
    hint(&mut old, rustix::fs::makedev(226, 999));
    assert_eq!(query(&mut old), (vec![], vec![0, TILED]));
    hint(&mut old, first.device_number);
    assert_eq!(query(&mut old).0, vec![0, TILED]);

    let mut inconsistent = preference(old.surface, Some(first));
    inconsistent.device.minor += 1;
    assert_eq!(
        frontend
            .update_window_allocation_preferences(XWindowAllocationPreferences {
                generation: 100,
                topology_generation,
                windows: vec![inconsistent],
            })
            .unwrap(),
        XWindowAllocationUpdate::Invalid
    );
    assert_eq!(query(&mut old), (vec![0, TILED], vec![0, TILED]));

    let second = XRenderDeviceIdentity {
        inode: first.inode + 1,
        ..first
    };
    frontend
        .install_device_bundle(Arc::new(
            XServerFrontendDeviceBundle::new(
                2,
                Arc::new(Provider {
                    identity: Some(second),
                    modifiers: vec![0, TILED],
                    observations: AtomicUsize::new(0),
                }),
                None,
            )
            .unwrap(),
        ))
        .unwrap();
    let mut new = client(&mut frontend);
    publish(
        &frontend,
        vec![
            preference(old.surface, Some(second)),
            preference(new.surface, Some(second)),
        ],
    );
    assert_eq!(query(&mut old), (vec![0], vec![0, TILED]));
    assert_eq!(query(&mut new), (vec![0, TILED], vec![0, TILED]));
    publish(
        &frontend,
        vec![
            preference(old.surface, Some(first)),
            preference(new.surface, Some(second)),
        ],
    );
    frontend.mark_device_generation_unavailable(1).unwrap();
    assert_eq!(query(&mut old), (vec![0], vec![0, TILED]));
    assert_eq!(query(&mut new), (vec![0, TILED], vec![0, TILED]));
    let mut without_linear = preference(old.surface, Some(second));
    without_linear.formats[0].modifiers = vec![TILED];
    publish(&frontend, vec![without_linear]);
    assert_eq!(
        query(&mut old),
        (vec![], vec![0, TILED]),
        "never add LINEAR missing from output preferences"
    );
    frontend
        .install_device_bundle(Arc::new(
            XServerFrontendDeviceBundle::new(
                3,
                Arc::new(Provider {
                    identity: None,
                    modifiers: vec![TILED],
                    observations: AtomicUsize::new(0),
                }),
                None,
            )
            .unwrap(),
        ))
        .unwrap();
    let mut unidentified = client(&mut frontend);
    publish(
        &frontend,
        vec![preference(unidentified.surface, Some(first))],
    );
    hint(&mut unidentified, first.device_number);
    assert_eq!(
        query(&mut unidentified),
        (vec![], vec![TILED]),
        "never add LINEAR absent from the pinned screen inventory"
    );
    publish(&frontend, vec![preference(unidentified.surface, None)]);
    assert_eq!(
        query(&mut unidentified),
        (vec![], vec![TILED]),
        "two absent identities are not evidence"
    );
    publish(&frontend, vec![preference(new.surface, Some(second))]);
    assert_eq!(query(&mut new).0, vec![0, TILED]);
    let mut topology = frontend.config().output_topology().clone();
    topology.generation += 1;
    frontend.update_output_topology(topology).unwrap();
    assert_eq!(
        query(&mut new),
        (vec![], vec![0, TILED]),
        "stale output identity must not retain tiled preferences"
    );
    for _ in 0..3 {
        assert_eq!(query(&mut old).1, vec![0, TILED]);
    }
    assert_eq!(
        provider.observations.load(Ordering::SeqCst),
        1,
        "queries must use the cached identity"
    );

    drop((old, new, unidentified));
    frontend.wait_for_clients().unwrap();
    drop(frontend);
    std::fs::remove_file(path).unwrap();
}
