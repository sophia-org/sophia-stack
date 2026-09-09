#![cfg(test)]

use super::*;

fn run_bounded(command: &mut Command, label: &str) -> String {
    let path = std::env::temp_dir().join(format!(
        "sophia-dri3-layout-{label}-{}.log",
        std::process::id()
    ));
    let log = ProbeBinary(path);
    let file = File::create(&log.0).unwrap();
    // File output prevents large modifier catalogs from filling a pipe while
    // the parent waits for the bounded child to exit.
    let mut child = ProbeChild(
        command
            .stdin(Stdio::null())
            .stdout(file.try_clone().unwrap())
            .stderr(file)
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + Duration::from_secs(30);
    let status = loop {
        if let Some(status) = child.0.try_wait().unwrap() {
            break status;
        }
        assert!(Instant::now() < deadline, "{label} exceeded its deadline");
        std::thread::sleep(Duration::from_millis(10));
    };
    const MAX_LOG_BYTES: u64 = 16 * 1024 * 1024;
    assert!(fs::metadata(&log.0).unwrap().len() <= MAX_LOG_BYTES);
    let output = fs::read_to_string(&log.0).unwrap();
    assert!(status.success(), "{label}: {status}\n{output}");
    output
}

fn field<'a>(line: &'a str, key: &str) -> &'a str {
    line.split_whitespace()
        .filter_map(|part| part.split_once('='))
        .find_map(|(name, value)| (name == key).then_some(value))
        .unwrap_or_else(|| panic!("missing {key} in {line}"))
}

fn number(line: &str, key: &str) -> u64 {
    let value = field(line, key);
    match value.strip_prefix("0x") {
        Some(hex) => u64::from_str_radix(hex, 16).unwrap(),
        None => value.parse().unwrap(),
    }
}

fn one_line<'a>(output: &'a str, prefix: &str) -> &'a str {
    let mut lines = output.lines().filter(|line| line.starts_with(prefix));
    let line = lines
        .next()
        .unwrap_or_else(|| panic!("missing {prefix}\n{output}"));
    assert!(lines.next().is_none(), "duplicate {prefix}");
    line
}

#[test]
#[ignore = "requires SOPHIA_PIXMAP_TEST_DEVICE and xcb/dri3/present/gbm development files; private X server allocation/list test only, no KMS"]
fn list_only_allocates_exact_layouts_without_mapping_importing_or_presenting() {
    let node =
        PathBuf::from(std::env::var_os("SOPHIA_PIXMAP_TEST_DEVICE").expect("select a render node"));
    let device = open(&node);
    let identity = rustix::fs::fstat(&device).unwrap();
    let measured =
        sophia_backend_live::query_dma_buf_import_formats(device.try_clone().unwrap()).unwrap();
    let formats: Vec<_> = measured
        .into_iter()
        .map(
            |row| sophia_x_authority::XServerFrontendDmaBufImportFormat {
                format: row.format,
                modifiers: row.modifiers,
            },
        )
        .collect();
    let xr24 = sophia_protocol::DRM_FORMAT_XRGB8888;
    let ar24 = sophia_protocol::DRM_FORMAT_ARGB8888;
    for format in [xr24, ar24] {
        assert!(
            formats
                .iter()
                .any(|row| row.format == format && row.modifiers.contains(&0)),
            "selected device must measure LINEAR import for {format:#x}"
        );
    }
    let binary = ProbeBinary(
        std::env::temp_dir().join(format!("sophia-dri3-layout-test-{}", std::process::id())),
    );
    let mut pkg = Command::new("pkg-config");
    pkg.args([
        "--cflags",
        "--libs",
        "xcb",
        "xcb-dri3",
        "xcb-present",
        "gbm",
    ]);
    let flags = run_bounded(&mut pkg, "pkg-config");
    let mut compile = Command::new("cc");
    compile
        .args(["-std=c11", "-O2", "-Wall", "-Wextra", "-Werror"])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/probes/dri3_layout.c"))
        .arg("-o")
        .arg(&binary.0)
        .args(flags.split_whitespace());
    run_bounded(&mut compile, "compile");

    let display = 74_000 + std::process::id() % 10_000;
    let socket = PathBuf::from(format!("/tmp/.X11-unix/X{display}"));
    assert!(!socket.exists(), "private display is occupied");
    let config = XServerFrontendConfig::new(&socket, NamespaceId::from_raw(u64::from(display)))
        .unwrap()
        .with_render_device_provider(Arc::new(LiveXRenderDeviceProvider {
            device,
            import_formats: formats.clone(),
        }))
        .with_pixmap_allocator(Arc::new(LiveXPixmapAllocator::new(open(&node))));
    let mut frontend = XServerFrontend::bind(config).unwrap();
    let requests = Arc::new(Mutex::new(Vec::new()));
    let observed = requests.clone();
    let observer: Arc<X11CoreTraceObserver> = Arc::new(move |trace| {
        let mut requests = observed.lock().unwrap();
        if requests.len() == 128 {
            return Err(X11SetupSocketError::new(
                "list-only request capacity exceeded",
            ));
        }
        requests.push((
            trace.major_opcode,
            trace.minor_opcode,
            trace.failure.is_some(),
            trace.dri3_pixmap_import.is_some(),
            trace.present_submission.is_some(),
        ));
        Ok(None)
    });
    let stop = Arc::new(AtomicBool::new(false));
    let worker_stop = stop.clone();
    let worker = std::thread::spawn(move || {
        let broker = XServerFrontendRouteBroker::new(NonZeroUsize::new(16).unwrap());
        while !worker_stop.load(Ordering::Acquire) {
            frontend.try_serve_next_concurrently_routed_traced(&broker, observer.clone())?;
            frontend.poll_client_workers()?;
            std::thread::sleep(Duration::from_millis(1));
        }
        frontend.shutdown_all_client_workers()?;
        frontend.wait_for_clients()
    });
    let mut server = PrivateFrontend {
        stop,
        worker: Some(worker),
        socket,
    };

    for (name, format) in [("XR24", xr24), ("AR24", ar24)] {
        let mut command = Command::new(&binary.0);
        command
            .args([
                "--list-only",
                "--modifier",
                "0",
                "--geometry",
                "0,0,16,16",
                "--format",
                name,
                "--timeout-ms",
                "4000",
            ])
            .env("DISPLAY", format!(":{display}"))
            .env("XAUTHORITY", "/dev/null")
            .env_remove("LD_PRELOAD")
            .env_remove("DRI_PRIME")
            .env_remove("MESA_LOADER_DRIVER_OVERRIDE");
        let output = run_bounded(&mut command, name);
        let reported = one_line(&output, "dri3_layout stage=device ");
        assert_eq!(number(reported, "fs_device"), identity.st_dev);
        assert_eq!(number(reported, "inode"), identity.st_ino);
        assert_eq!(number(reported, "rdev"), identity.st_rdev);
        let allocation = one_line(&output, "dri3_layout event=allocation ");
        assert_eq!(number(allocation, "format"), u64::from(format));
        assert_eq!(number(allocation, "modifier"), 0);
        assert_eq!(number(allocation, "width"), 16);
        assert_eq!(number(allocation, "height"), 16);
        let planes = number(allocation, "planes");
        assert!((1..=4).contains(&planes));
        let rows: Vec<_> = output
            .lines()
            .filter(|line| line.starts_with("dri3_layout stage=plane "))
            .collect();
        assert_eq!(rows.len() as u64, planes);
        for (index, row) in rows.iter().enumerate() {
            assert_eq!(number(row, "plane"), index as u64);
            let stride = number(row, "stride");
            let offset = number(row, "offset");
            let bytes = number(row, "allocation_bytes");
            assert!(stride > 0 && offset < bytes);
            if index == 0 {
                assert!(stride >= 16 * 4);
                assert!(offset + 15 * stride + 16 * 4 <= bytes);
            }
        }
        for expected in formats
            .iter()
            .filter(|row| [xr24, ar24].contains(&row.format))
        {
            let catalog: Vec<_> = output
                .lines()
                .filter(|line| line.starts_with("dri3_layout stage=modifier "))
                .filter(|line| {
                    field(line, "scope") == "screen"
                        && number(line, "format") == u64::from(expected.format)
                })
                .map(|line| number(line, "modifier"))
                .collect();
            assert_eq!(catalog, expected.modifiers, "exact measured screen catalog");
        }
        let finished = one_line(&output, "dri3_layout event=finished ");
        assert_eq!(field(finished, "result"), "pass");
        for key in ["submitted", "completed", "idle"] {
            assert_eq!(number(finished, key), 0);
        }
        eprintln!("{reported}\n{allocation}\n{}\n{finished}", rows.join("\n"));
    }
    server.finish();
    let requests = requests.lock().unwrap();
    let count = |major, minor| {
        requests
            .iter()
            .filter(|row| row.0 == major && row.1 == minor)
            .count()
    };
    assert!(
        requests.iter().all(|row| !row.2 && !row.3 && !row.4),
        "{requests:?}"
    );
    assert_eq!(count(1, 0), 2, "both unmapped windows were created");
    assert_eq!(
        count(137, 1),
        2,
        "both clients opened the advertised DRM device"
    );
    assert_eq!(count(137, 6), 4, "both clients queried both exact formats");
    assert_eq!(
        requests.iter().filter(|row| row.0 == 8).count(),
        0,
        "no MapWindow"
    );
    assert_eq!(count(137, 2) + count(137, 7), 0, "no DRI3 pixmap import");
    assert_eq!(count(138, 1), 0, "no PresentPixmap");
    eprintln!("dri3_layout_trace opens=2 modifier_queries=4 map=0 import=0 present=0");
}
