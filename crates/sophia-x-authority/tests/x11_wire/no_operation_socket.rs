/// NoOperation with `padding_units` extra 4-byte units beyond its header.
/// Padding is the request's whole purpose: clients use it to align what
/// follows, and the server must ignore the filler and stay framed.
#[cfg(unix)]
fn no_operation_request(byte_order: XByteOrder, padding_units: u16) -> Vec<u8> {
    let mut out = vec![127, 0];
    push_u16(&mut out, byte_order, 1 + padding_units);
    out.resize(4 + usize::from(padding_units) * 4, 0xab);
    out
}

/// A client that sends NoOperation must get nothing back, and the request
/// after it must still complete against the sequence the client expects.
/// Answering an error here does not merely add noise: it desynchronises every
/// completion that follows.
#[cfg(unix)]
#[test]
fn frontend_no_operation_answers_nothing_and_keeps_sequences_aligned() {
    use std::io::Write;
    use std::sync::Arc;
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    for byte_order in [XByteOrder::LittleEndian, XByteOrder::BigEndian] {
        let socket_path = std::env::temp_dir().join(format!(
            "sophia-x-no-operation-{}-{}.sock",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let namespace = NamespaceContext::new(
            NamespaceId::from_raw(833),
            NamespaceProfile::ClassicShared,
            NamespaceCapabilities::NONE,
        )
        .unwrap();
        let policy = Arc::new(TestXAdmissionPolicy::new(namespace, false));
        let config = XServerFrontendConfig::new_with_namespace_context(&socket_path, namespace)
            .unwrap()
            .with_admission_policy(policy.clone());
        let server = thread::spawn(move || {
            let mut frontend = XServerFrontend::bind(config).unwrap();
            frontend.serve_next().unwrap();
        });

        wait_for_socket(&socket_path);
        let mut client = connect_x_socket(&socket_path);
        client
            .write_all(&setup_request(byte_order, 11, 0, b"", b""))
            .unwrap();
        read_setup_success(&mut client, byte_order);

        // Sequence 1.
        let window = X_SETUP_DEFAULT_RESOURCE_ID_BASE + 1;
        client
            .write_all(&create_window_request(byte_order, window, 0, 0, 64, 48))
            .unwrap();
        // Sequences 2 and 3: one bare, one carrying padding, so the reply
        // below also proves the filler was consumed rather than parsed as a
        // following request.
        client
            .write_all(&no_operation_request(byte_order, 0))
            .unwrap();
        client
            .write_all(&no_operation_request(byte_order, 5))
            .unwrap();
        // Sequence 4.
        client
            .write_all(&resource_request(byte_order, 14, window))
            .unwrap();

        let record = read_x_reply(&mut client, byte_order);
        assert_eq!(
            record[0], 1,
            "{byte_order:?}: NoOperation answered something; the first record \
             back should be the GetGeometry reply"
        );
        assert_eq!(
            read_u16(byte_order, &record[2..4]),
            4,
            "{byte_order:?}: the two NoOperations must each consume a sequence \
             number, or every completion after them is misattributed"
        );

        drop(client);
        server.join().unwrap();
        let _ = std::fs::remove_file(&socket_path);
    }
}
