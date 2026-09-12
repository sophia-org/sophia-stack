use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use sophia_protocol::*;
use sophia_runtime::*;
use sophia_shell_client::{ShellClientOptions, ShellConnection};

fn directory() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "sophia-content-transport-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn evidence() -> ProtectionDomainEvidence {
    ProtectionDomainEvidence {
        backend: ProtectionBackendKind::Bubblewrap,
        supervisor_pid: std::process::id(),
        peer_pid: std::process::id(),
        roles: [ProtectionDomainRole::MetadataShell].into_iter().collect(),
    }
}

fn next_content(client: &mut ShellConnection) -> ShellContentRecord {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        if let Some((_, record)) = client.poll_content().unwrap() {
            return record;
        }
        assert!(Instant::now() < deadline, "content response timed out");
        std::thread::yield_now();
    }
}

#[test]
fn admitted_resource_transfer_settles_and_releases_over_the_real_socket() {
    let mut session = ShellSessionTransport::bind_for_supervised_uid(
        directory(),
        rustix::process::geteuid().as_raw(),
    )
    .unwrap();
    session.authorize_protected_peer(&evidence()).unwrap();
    let socket = session.socket_path().to_path_buf();
    let (done_tx, done_rx) = std::sync::mpsc::channel();
    let client = std::thread::spawn(move || {
        let capabilities =
            SOPHIA_SHELL_CAPABILITY_DESCRIPTOR_SWITCHER | SOPHIA_SHELL_CAPABILITY_CONTENT_SURFACE;
        let mut client = ShellConnection::connect(
            socket,
            ShellClientOptions {
                minimum_revision: 5,
                maximum_revision: 6,
                required_capabilities: capabilities,
                handshake_timeout: Duration::from_secs(2),
            },
        )
        .unwrap();
        let ShellContentRecord::Limits(limits) = next_content(&mut client) else {
            panic!("expected limits");
        };
        let transaction = TransactionId::from_raw(7);
        let resource = ContentResourceId {
            id: 1,
            generation: 1,
        };
        client
            .send_content(
                transaction,
                &ShellContentRecord::ResourceBegin(ContentResourceBegin {
                    grant: limits.grant,
                    resource,
                    width_px: 2,
                    height_px: 1,
                    rendered_scale_numerator: 1,
                    rendered_scale_denominator: 1,
                    pixel_format: 1,
                    chunk_count: 1,
                    total_bytes: 8,
                }),
            )
            .unwrap();
        client
            .send_content(
                transaction,
                &ShellContentRecord::ResourceChunk(ContentResourceChunk {
                    grant: limits.grant,
                    resource,
                    ordinal: 0,
                    offset: 0,
                    bytes: vec![0, 0, 255, 255, 0, 128, 0, 128],
                }),
            )
            .unwrap();
        client
            .send_content(
                transaction,
                &ShellContentRecord::ResourceEnd(ContentResourceEnd {
                    grant: limits.grant,
                    resource,
                    total_bytes: 8,
                    chunk_count: 1,
                }),
            )
            .unwrap();
        let mut statuses = Vec::new();
        while statuses.len() < 2 {
            if let ShellContentRecord::ResourceStatus(status) = next_content(&mut client) {
                statuses.push(status.status);
            }
        }
        assert_eq!(statuses, [1, 2]);
        client
            .send_content(
                TransactionId::from_raw(8),
                &ShellContentRecord::ResourceRetire(ContentResourceRetire {
                    grant: limits.grant,
                    resource,
                }),
            )
            .unwrap();
        let ShellContentRecord::ResourceReleased(released) = next_content(&mut client) else {
            panic!("expected resource release");
        };
        assert_eq!(released.resource, resource);
        assert_eq!(released.reason, ContentReason::None as u16);
        done_tx.send(()).unwrap();
    });

    session
        .accept_and_negotiate_with_content_policy(
            1,
            Duration::from_secs(2),
            ShellContentAdmissionPolicy::Granted {
                discrete_input: false,
            },
        )
        .unwrap();
    let start = Instant::now();
    while done_rx.try_recv().is_err() {
        session
            .service_content_resources(start.elapsed().as_millis() as u64)
            .unwrap();
        assert!(start.elapsed() < Duration::from_secs(2));
        std::thread::yield_now();
    }
    session.disconnect().unwrap();
    client.join().unwrap();
}
