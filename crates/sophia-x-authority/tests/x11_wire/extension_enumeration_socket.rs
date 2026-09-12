#[cfg(unix)]
fn list_extensions_request(byte_order: XByteOrder) -> Vec<u8> {
    let mut out = vec![99, 0];
    push_u16(&mut out, byte_order, 1);
    out
}

/// Enumerating the extensions and then asking about each one must not produce
/// two different answers. The two replies are built by separate code, and DRI3
/// presence depends on a render-device provider that the pure dispatch cannot
/// see, so agreement has to be checked against a served connection rather than
/// against the dispatch table.
#[cfg(unix)]
#[test]
fn frontend_extension_enumeration_agrees_with_query_in_both_byte_orders() {
    use std::io::Write;
    use std::sync::Arc;
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    for byte_order in [XByteOrder::LittleEndian, XByteOrder::BigEndian] {
        let socket_path = std::env::temp_dir().join(format!(
            "sophia-x-extension-enumeration-{}-{}.sock",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let namespace = NamespaceContext::new(
            NamespaceId::from_raw(831),
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

        client.write_all(&list_extensions_request(byte_order)).unwrap();
        let mut header = [0; 32];
        fill_from_socket(&mut client, &mut header);
        assert_eq!(header[0], 1, "{byte_order:?}: a reply, not an error");
        let count = usize::from(header[1]);
        assert!(
            count > 0,
            "{byte_order:?}: the frontend advertises extensions through \
             QueryExtension and must enumerate them here; reporting none told \
             every client that asked there were no extensions at all"
        );
        let payload_len = read_u32(byte_order, &header[4..8]) as usize * 4;
        let mut payload = vec![0; payload_len];
        fill_from_socket(&mut client, &mut payload);

        let mut at = 0;
        let mut listed = Vec::new();
        for _ in 0..count {
            let len = usize::from(payload[at]);
            at += 1;
            listed.push(String::from_utf8(payload[at..at + len].to_vec()).expect("utf8 name"));
            at += len;
        }

        for name in &listed {
            client
                .write_all(&query_extension_request(byte_order, name))
                .unwrap();
            let mut reply = [0; 32];
            fill_from_socket(&mut client, &mut reply);
            assert_eq!(reply[0], 1, "{byte_order:?}: {name} query is a reply");
            assert_eq!(
                reply[8], 1,
                "{byte_order:?}: {name} is enumerated but QueryExtension reports it absent"
            );
        }

        // This fixture has no render-device provider, so DRI3 is genuinely
        // absent here and the enumeration must say so. Listing it would be a
        // disagreement the loop above cannot catch: it only checks the names
        // that were listed.
        assert!(
            !listed.iter().any(|name| name == X_DRI3_EXTENSION_NAME),
            "{byte_order:?}: DRI3 needs a render-device provider and this \
             connection has none, but the enumeration offered it anyway"
        );

        drop(client);
        server.join().unwrap();
        let _ = std::fs::remove_file(&socket_path);
    }
}
