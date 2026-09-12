/// A peer that never completes the X11 handshake must not take the server with
/// it.
///
/// Connecting and closing is ordinary: port scans do it, health checks do it,
/// and any client that gives up mid-connect does it. The setup path classifies
/// what went wrong, and `reap_client_worker` retires a worker quietly only when
/// that classification says the client was at fault. An unclassified error is
/// treated as a server fault and propagates out of the service, ending every
/// other client's session with it.
///
/// The second client is the assertion that matters. A test that only checked
/// the first would pass against a server that had already died.
/// Marks the case whose handshake is complete but whose peer leaves before the
/// answer is written. Its bytes are built inside the loop.
#[cfg(unix)]
const COMPLETE_SETUP_THEN_LEAVE: &[u8] = b"complete-setup";

#[cfg(unix)]
#[test]
fn a_peer_that_never_handshakes_does_not_end_the_service() {
    use std::io::Write;
    use std::sync::Arc;
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Each case leaves the handshake incomplete in a different way.
    for (label, prelude) in [
        ("closes before sending anything", &[][..]),
        ("closes part way through the prefix", &[0x6c, 0, 11][..]),
        ("sends a prefix that is not a setup request", &[0xff; 12][..]),
        ("completes setup then leaves before reading the answer", COMPLETE_SETUP_THEN_LEAVE),
    ] {
        let socket_path = std::env::temp_dir().join(format!(
            "sophia-x-setup-containment-{}-{}.sock",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let namespace = NamespaceContext::new(
            NamespaceId::from_raw(837),
            NamespaceProfile::ClassicShared,
            NamespaceCapabilities::NONE,
        )
        .unwrap();
        let policy = Arc::new(TestXAdmissionPolicy::new(namespace, false));
        let config = XServerFrontendConfig::new_with_namespace_context(&socket_path, namespace)
            .unwrap()
            .with_admission_policy(policy.clone());

        let server = thread::spawn(move || -> Result<(), X11SetupSocketError> {
            let mut frontend = XServerFrontend::bind(config).unwrap();
            frontend.serve_next_concurrently()?;
            frontend.serve_next_concurrently()?;
            frontend.wait_for_clients()
        });

        wait_for_socket(&socket_path);
        let mut abandoned = connect_x_socket(&socket_path);
        if prelude == COMPLETE_SETUP_THEN_LEAVE {
            // A complete, valid setup, abandoned before its answer is read.
            // The server writes into a closed peer, which fails on the write
            // rather than the read: the read classification alone misses it.
            let _ = abandoned.write_all(&setup_request(
                XByteOrder::LittleEndian,
                11,
                0,
                b"",
                b"",
            ));
            let _ = abandoned.flush();
        } else if !prelude.is_empty() {
            let _ = abandoned.write_all(prelude);
            let _ = abandoned.flush();
        }
        drop(abandoned);

        // A healthy client, after the abandoned one, on the same frontend.
        let mut healthy = connect_x_socket(&socket_path);
        healthy
            .write_all(&setup_request(XByteOrder::LittleEndian, 11, 0, b"", b""))
            .unwrap();
        // Reaching a resource-id base at all means the setup completed, which
        // is what proves the service survived. The exact base is not pinned:
        // a peer that completed setup before leaving consumed the first range
        // legitimately, so the next client is handed the one after it.
        let base = read_setup_resource_id_base(&mut healthy, XByteOrder::LittleEndian);
        assert!(
            base >= X_SETUP_DEFAULT_RESOURCE_ID_BASE,
            "{label}: the server did not survive to serve a healthy client"
        );

        drop(healthy);
        let outcome = server.join().expect("the frontend thread must not panic");
        assert!(
            outcome.is_ok(),
            "{label}: a peer that never handshook ended the whole service: {:?}",
            outcome.err()
        );
        let _ = std::fs::remove_file(&socket_path);
    }
}

/// The same containment, one stage later: a client that completes setup, then
/// announces a request length and leaves without sending the body.
///
/// The request *header* read already treated a vanished peer as an ordinary
/// end of stream. The payload read did not, so abandoning a request part way
/// through ended the service exactly as an abandoned handshake did.
#[cfg(unix)]
#[test]
fn a_client_that_abandons_a_request_body_does_not_end_the_service() {
    use std::io::Write;
    use std::sync::Arc;
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    let socket_path = std::env::temp_dir().join(format!(
        "sophia-x-truncated-request-{}-{}.sock",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let namespace = NamespaceContext::new(
        NamespaceId::from_raw(839),
        NamespaceProfile::ClassicShared,
        NamespaceCapabilities::NONE,
    )
    .unwrap();
    let policy = Arc::new(TestXAdmissionPolicy::new(namespace, false));
    let config = XServerFrontendConfig::new_with_namespace_context(&socket_path, namespace)
        .unwrap()
        .with_admission_policy(policy.clone());

    let server = thread::spawn(move || -> Result<(), X11SetupSocketError> {
        let mut frontend = XServerFrontend::bind(config).unwrap();
        frontend.serve_next_concurrently()?;
        frontend.serve_next_concurrently()?;
        frontend.wait_for_clients()
    });

    wait_for_socket(&socket_path);
    let mut abandoned = connect_x_socket(&socket_path);
    abandoned
        .write_all(&setup_request(XByteOrder::LittleEndian, 11, 0, b"", b""))
        .unwrap();
    read_setup_resource_id_base(&mut abandoned, XByteOrder::LittleEndian);
    // A CreateWindow header claiming eight units, with none of the body.
    let mut truncated = vec![1, 0];
    push_u16(&mut truncated, XByteOrder::LittleEndian, 8);
    abandoned.write_all(&truncated).unwrap();
    abandoned.flush().unwrap();
    drop(abandoned);

    let mut healthy = connect_x_socket(&socket_path);
    healthy
        .write_all(&setup_request(XByteOrder::LittleEndian, 11, 0, b"", b""))
        .unwrap();
    assert!(
        read_setup_resource_id_base(&mut healthy, XByteOrder::LittleEndian)
            >= X_SETUP_DEFAULT_RESOURCE_ID_BASE,
        "the server did not survive a client that abandoned a request body"
    );

    drop(healthy);
    let outcome = server.join().expect("the frontend thread must not panic");
    assert!(
        outcome.is_ok(),
        "an abandoned request body ended the whole service: {:?}",
        outcome.err()
    );
    let _ = std::fs::remove_file(&socket_path);
}
