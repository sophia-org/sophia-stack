//! Publish revision 6 indicators to an independently implemented client.
//!
//! The point of this host is what it refuses to do itself. It launches the
//! client through the real protection domain, negotiates through the real
//! transport, projects a policy publication through the **production** mapping,
//! and encodes with the **production** encoder. Nothing about the bytes
//! originates here, so a client agreeing with them is agreeing with the
//! session rather than with a second encoder written to match it.
//!
//! What it does not establish, and what must be pinned elsewhere: live service
//! scheduling, unchanged-snapshot suppression, reconnect refresh, and topology
//! sourcing of the active output. Those live in the owner loop and the session's
//! publication path, not in the projection this exercises.

use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sophia_protocol::{
    OutputId, PolicyProjectionIndicator, PolicyProjectionOutputStatus, TransactionId,
    encode_shell_indicator_snapshot,
};
use sophia_runtime::{
    ProcessLaunchSpec, ProcessSupervisor, ProtectionDomainRole, ProtectionDomainSpec,
    ProtectionPath, ShellSessionTransport, SupervisedProcessKind, SupervisorCommand,
};
use sophia_session::shell_indicator_publication::indicator_snapshot;

fn main() {
    if let Err(error) = run() {
        eprintln!("shell-indicator-conformance-host: {error}");
        std::process::exit(1);
    }
}

/// One output holds a view; the other is active and holds nothing. That second
/// output is the whole reason the vocabulary carries a separate active-output
/// identity, so it is the case this host publishes.
fn publication() -> sophia_engine::PolicyIndicatorPublication {
    sophia_engine::PolicyIndicatorPublication {
        tab_groups: Vec::new(),
        generation: 6,
        connection_epoch: Some(1),
        indicators: vec![
            PolicyProjectionIndicator {
                output: OutputId::from_raw(1),
                slot: 0,
                indicator: 11,
                action: Some(sophia_protocol::WmActionId::from_raw(41)),
                state_bits: 1,
                label: "web".to_owned(),
            },
            PolicyProjectionIndicator {
                output: OutputId::from_raw(1),
                slot: 1,
                indicator: 12,
                // Published without an action: not activatable, and the
                // projection must spell that as the zero sentinel.
                action: None,
                state_bits: 0,
                label: "code".to_owned(),
            },
        ],
        output_statuses: vec![
            PolicyProjectionOutputStatus {
                output: OutputId::from_raw(1),
                focus_bits: 0,
                layout: "Scroller".to_owned(),
            },
            PolicyProjectionOutputStatus {
                output: OutputId::from_raw(2),
                focus_bits: 1,
                layout: "Scroller".to_owned(),
            },
        ],
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: shell_indicator_conformance_host CLIENT")?;
    if !client.is_absolute() || !client.is_file() {
        return Err("shell client must be an absolute executable path".into());
    }

    let directory = std::env::temp_dir().join(format!(
        "sophia-shell-indicator-conformance-{}-{}",
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
    let spec = ProcessLaunchSpec::new(&client)
        .arg("--serve")
        .env(sophia_runtime::SOPHIA_SHELL_SOCKET_ENV, &socket)
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

    let connection_epoch = 1;
    let welcome = transport.accept_and_negotiate(connection_epoch, Duration::from_secs(5))?;

    // The client must have opted in; the gate is production's, not this host's.
    if !transport.supports_indicators() {
        return Err("client did not negotiate view indicators".into());
    }
    println!(
        "sophia_shell_indicator_host schema=1 status=negotiated revision={} capabilities=0x{:x}",
        welcome.selected_revision, welcome.capabilities
    );

    let snapshot = indicator_snapshot(
        &publication(),
        Some(OutputId::from_raw(2)),
        connection_epoch,
    );
    let transaction = TransactionId::from_raw(1);
    let frames = encode_shell_indicator_snapshot(transaction, &snapshot)
        .map_err(|error| format!("indicator encode failed: {error:?}"))?;
    for frame in frames {
        transport.send_async(frame)?;
    }
    // Closing the stream is how the client learns the set is complete; it exits
    // on EOF, which is the same shape a session teardown presents.
    transport.disconnect()?;
    supervisor.terminate()?;
    println!(
        "sophia_shell_indicator_corpus schema=1 status=complete protected=true indicators={} statuses={} active_output={} absent_action=1",
        snapshot.indicators.len(),
        snapshot.statuses.len(),
        snapshot.active_output.map_or(0, OutputId::raw)
    );
    Ok(())
}
