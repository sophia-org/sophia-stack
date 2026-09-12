use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use sophia_protocol::{ContentResourceId, TransactionId};
use sophia_runtime::{
    ProcessLaunchSpec, ProcessSupervisor, ProtectionDomainRole, ProtectionDomainSpec,
    ProtectionPath, ShellContentAdmissionPolicy, ShellSessionTransport, SupervisedProcessKind,
    SupervisorCommand, SupervisorEvent,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("shell-content-conformance-host: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: shell_content_conformance_host CLIENT")?;
    if !client.is_absolute() || !client.is_file() {
        return Err("shell client must be an absolute executable path".into());
    }
    let directory = std::env::temp_dir().join(format!(
        "sophia-shell-content-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    ));
    let mut transport = ShellSessionTransport::bind_for_supervised_uid(
        &directory,
        rustix::process::geteuid().as_raw(),
    )?;
    let socket = transport.socket_path().to_path_buf();
    let domain = ProtectionDomainSpec::bubblewrap([ProtectionDomainRole::MetadataShell])?.path(
        ProtectionPath::read_only(socket.parent().ok_or("shell socket lacks a parent")?),
    )?;
    let spec = ProcessLaunchSpec::new(client)
        .arg("content-proof")
        .arg("--socket")
        .arg(&socket)
        .process_group()
        .protection_domain(domain);
    let mut supervisor = ProcessSupervisor::new(SupervisedProcessKind::Shell, spec);
    supervisor.apply(SupervisorCommand::StartProcess {
        process: SupervisedProcessKind::Shell,
        delay: Duration::ZERO,
    })?;
    let evidence = supervisor
        .protection_evidence()
        .ok_or("shell process has no protection evidence")?
        .clone();
    transport.authorize_protected_peer(&evidence)?;
    transport.accept_and_negotiate_with_content_policy(
        1,
        Duration::from_secs(5),
        ShellContentAdmissionPolicy::Granted {
            discrete_input: false,
        },
    )?;
    let grant = transport.content_grant().ok_or("content was not granted")?;
    let resource = ContentResourceId {
        id: 1,
        generation: 1,
    };
    let deadline = Instant::now() + Duration::from_secs(5);
    let started = Instant::now();
    let mut lease = None;
    let mut verified = false;
    loop {
        if Instant::now() >= deadline {
            return Err("content client did not settle before the host deadline".into());
        }
        match transport.service_content_resources(started.elapsed().as_millis() as u64) {
            Ok(_) => {}
            Err(sophia_runtime::ShellTransportError::NotConnected)
                if verified
                    && transport
                        .content_usage()
                        .is_some_and(|usage| usage == Default::default()) => {}
            Err(error) => return Err(error.into()),
        }
        if lease.is_none()
            && let Ok(candidate) = transport.lease_content_resource(grant, resource)
        {
            if candidate.bytes() != [0, 0, 255, 255, 0, 128, 0, 128] {
                return Err("independent client uploaded different canonical pixels".into());
            }
            lease = Some(candidate);
        }
        if lease.is_some()
            && transport
                .content_usage()
                .is_some_and(|usage| usage.retiring == 8)
        {
            drop(lease.take());
            verified = true;
        }
        if supervisor.poll()? == Some(SupervisorEvent::ProcessExited) {
            if !verified
                || transport
                    .content_usage()
                    .is_none_or(|usage| usage != Default::default())
            {
                return Err("client exited before exact lease-backed release".into());
            }
            break;
        }
        std::thread::yield_now();
    }
    transport.disconnect()?;
    println!(
        "sophia_shell_content_transport schema=1 status=complete protected=true bytes=8 accepted=true lease_retained=true released=true native_presentation=false transaction={}",
        TransactionId::from_raw(1).raw()
    );
    Ok(())
}
