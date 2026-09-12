/// A selection change reaches only the namespace it belongs to.
///
/// Atoms are global: the same selection name means the same number in every
/// namespace, so the name alone does not say whose selection changed. A watcher
/// that subscribed in one namespace and is told about another's change learns
/// an owner window it was never entitled to see, which is the disclosure the
/// namespace boundary exists to prevent.
///
/// The conformance gate's host runs one ClassicShared namespace and cannot
/// reach this at all, so it is checked here.
#[cfg(unix)]
#[test]
fn a_selection_change_is_not_disclosed_across_namespaces() {
    use std::io::Write;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    const PRIMARY: u32 = 1;

    let socket_path = std::env::temp_dir().join(format!(
        "sophia-x-xfixes-namespaces-{}-{}.sock",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    // Two confined namespaces: the watcher connects first, the owner second.
    let watcher_namespace = NamespaceContext::new(
        NamespaceId::from_raw(851),
        NamespaceProfile::Confined,
        NamespaceCapabilities::NONE,
    )
    .unwrap();
    let owner_namespace = NamespaceContext::new(
        NamespaceId::from_raw(852),
        NamespaceProfile::Confined,
        NamespaceCapabilities::NONE,
    )
    .unwrap();
    let policy = Arc::new(SequencedXAdmissionPolicy {
        namespaces: [watcher_namespace, owner_namespace],
        next_client: std::sync::atomic::AtomicU64::new(0),
        revoked: std::sync::Mutex::new(Vec::new()),
    });
    let config = XServerFrontendConfig::new_with_namespace_context(&socket_path, watcher_namespace)
        .unwrap()
        .with_admission_policy(policy.clone())
        .with_max_concurrent_clients(std::num::NonZeroUsize::new(4).unwrap());

    let broker = XServerFrontendRouteBroker::new(std::num::NonZeroUsize::new(8).unwrap());
    let server = thread::spawn(move || -> Result<(), X11SetupSocketError> {
        let mut frontend = XServerFrontend::bind(config).unwrap();
        for _ in 0..2 {
            frontend.serve_next_concurrently_routed(&broker)?;
        }
        frontend.wait_for_clients()
    });

    wait_for_socket(&socket_path);

    let mut watcher = connect_x_socket(&socket_path);
    watcher
        .write_all(&setup_request(XByteOrder::LittleEndian, 11, 0, b"", b""))
        .unwrap();
    let watched = read_setup_resource_id_base(&mut watcher, XByteOrder::LittleEndian) + 1;
    watcher
        .write_all(&create_window_request(
            XByteOrder::LittleEndian,
            watched,
            0,
            0,
            32,
            24,
        ))
        .unwrap();
    sync_x_connection(&mut watcher, XByteOrder::LittleEndian, watched);
    watcher
        .write_all(&xfixes_select_selection_input_request(
            XByteOrder::LittleEndian,
            watched,
            PRIMARY,
            0b111,
        ))
        .unwrap();
    sync_x_connection(&mut watcher, XByteOrder::LittleEndian, watched);

    // The owner is in the other namespace and names the same global atom.
    let mut owner = connect_x_socket(&socket_path);
    owner
        .write_all(&setup_request(XByteOrder::LittleEndian, 11, 0, b"", b""))
        .unwrap();
    let owner_window = read_setup_resource_id_base(&mut owner, XByteOrder::LittleEndian) + 1;
    owner
        .write_all(&create_window_request(
            XByteOrder::LittleEndian,
            owner_window,
            0,
            0,
            32,
            24,
        ))
        .unwrap();
    sync_x_connection(&mut owner, XByteOrder::LittleEndian, owner_window);
    owner
        .write_all(&set_selection_owner_request(
            XByteOrder::LittleEndian,
            owner_window,
            PRIMARY,
            7,
        ))
        .unwrap();
    sync_x_connection(&mut owner, XByteOrder::LittleEndian, owner_window);

    // The owner then departs, which ends the ownership: subtype 2 would be
    // delivered to a same-namespace watcher. This one must still hear nothing.
    drop(owner);

    // Nothing may arrive. A reply to the watcher's own request is fine; an
    // event is not.
    watcher
        .set_read_timeout(Some(Duration::from_millis(750)))
        .unwrap();
    watcher
        .write_all(&resource_request(XByteOrder::LittleEndian, 14, watched))
        .unwrap();
    let record = read_x_reply(&mut watcher, XByteOrder::LittleEndian);
    assert_eq!(
        record[0], 1,
        "the watcher received {record:?} where only its own reply belongs; a \
         selection change in another namespace was disclosed to it"
    );

    drop(watcher);
    let outcome = server.join().expect("the frontend thread must not panic");
    assert!(outcome.is_ok(), "{:?}", outcome.err());
    let _ = std::fs::remove_file(&socket_path);
}
