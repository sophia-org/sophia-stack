#[cfg(unix)]
struct ThreeClientNamespacePolicy {
    namespaces: [NamespaceContext; 3],
    next_client: std::sync::atomic::AtomicU64,
}

#[cfg(unix)]
impl XServerFrontendAdmissionPolicy for ThreeClientNamespacePolicy {
    fn admit(
        &self,
        request: XServerFrontendAdmissionRequest,
    ) -> Result<ClientAdmissionContext, XServerFrontendAdmissionError> {
        let index = self
            .next_client
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let namespace = self
            .namespaces
            .get(usize::try_from(index).map_err(|_| XServerFrontendAdmissionError::Unavailable)?)
            .copied()
            .ok_or(XServerFrontendAdmissionError::Unavailable)?;
        ClientAdmissionContext::new(
            ClientAdmissionId::from_raw(index + 1),
            namespace,
            ClientAuthProvenance::new(request.setup_authentication, 9).unwrap(),
        )
        .ok_or(XServerFrontendAdmissionError::Unavailable)
    }

    fn revoke(&self, _context: ClientAdmissionContext) -> Result<(), XServerFrontendAdmissionError> {
        Ok(())
    }
}

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
    // Three clients: the watcher that must hear nothing, a control in the
    // owner's namespace that must hear everything, and the owner.
    let policy = Arc::new(ThreeClientNamespacePolicy {
        namespaces: [watcher_namespace, owner_namespace, owner_namespace],
        next_client: std::sync::atomic::AtomicU64::new(0),
    });
    let config = XServerFrontendConfig::new_with_namespace_context(&socket_path, watcher_namespace)
        .unwrap()
        .with_admission_policy(policy.clone())
        .with_max_concurrent_clients(std::num::NonZeroUsize::new(4).unwrap());

    let broker = XServerFrontendRouteBroker::new(std::num::NonZeroUsize::new(8).unwrap());
    let server = thread::spawn(move || -> Result<(), X11SetupSocketError> {
        let mut frontend = XServerFrontend::bind(config).unwrap();
        for _ in 0..3 {
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

    // The control subscribes in the owner's namespace. It is the barrier:
    // once it has an event, delivery for that change has happened, so a
    // watcher that still has nothing was excluded rather than merely slower.
    let mut control = connect_x_socket(&socket_path);
    control
        .write_all(&setup_request(XByteOrder::LittleEndian, 11, 0, b"", b""))
        .unwrap();
    let control_window = read_setup_resource_id_base(&mut control, XByteOrder::LittleEndian) + 1;
    control
        .write_all(&create_window_request(
            XByteOrder::LittleEndian,
            control_window,
            0,
            0,
            32,
            24,
        ))
        .unwrap();
    sync_x_connection(&mut control, XByteOrder::LittleEndian, control_window);
    control
        .write_all(&xfixes_select_selection_input_request(
            XByteOrder::LittleEndian,
            control_window,
            PRIMARY,
            0b111,
        ))
        .unwrap();
    sync_x_connection(&mut control, XByteOrder::LittleEndian, control_window);

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

    // The control receives the set, which proves the change was delivered.
    control
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let set = read_x_record(&mut control);
    assert_eq!(
        set[0],
        X_XFIXES_FIRST_EVENT,
        "the control is in the owner's namespace and must receive the set"
    );
    assert_eq!(set[1], 0, "subtype 0, the owner was set");

    // The owner then departs, ending the ownership. That teardown travels a
    // different path from the set above, so it is observed on its own.
    drop(owner);
    let closed = read_x_record(&mut control);
    assert_eq!(
        closed[0],
        X_XFIXES_FIRST_EVENT,
        "the control must receive the ownership ending"
    );
    assert_eq!(closed[1], 2, "subtype 2, the owning client closed");

    // Both changes have now been delivered to someone. Only after that is a
    // quiet watcher evidence of exclusion rather than of timing.
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

    // A bounded quiet read after the reply. The reply is not a barrier for the
    // asynchronous writer, so an event delayed behind it would arrive here;
    // dropping the connection at the reply would never see one.
    watcher
        .set_read_timeout(Some(Duration::from_millis(750)))
        .unwrap();
    let mut stray = [0u8; 32];
    match std::io::Read::read(&mut watcher, &mut stray) {
        Ok(0) => {}
        Ok(_) => panic!(
            "an event arrived after the watcher's own reply: {stray:?}; a \
             selection change in another namespace was disclosed to it"
        ),
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
            ) => {}
        Err(error) => panic!("quiet read failed: {error}"),
    }

    drop(control);
    drop(watcher);
    let outcome = server.join().expect("the frontend thread must not panic");
    assert!(outcome.is_ok(), "{:?}", outcome.err());
    let _ = std::fs::remove_file(&socket_path);
}
