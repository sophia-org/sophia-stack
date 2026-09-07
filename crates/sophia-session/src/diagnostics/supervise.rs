//! The installed wrapper's diagnostic lifetime, without owning graphics or TTYs.
use std::io;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;

use super::Store;

pub fn supervise(
    profile: &str,
    identity: &str,
    program: &Path,
    args: &[String],
) -> io::Result<i32> {
    let (store, record) = match Store::from_environment().and_then(|store| {
        let record = store.begin(profile, std::process::id(), identity)?;
        Ok((store, record))
    }) {
        Ok(pair) => pair,
        Err(error) if error.kind() == io::ErrorKind::WouldBlock => return Err(error),
        Err(_) => {
            crate::session_eprintln!("sophia_session_diagnostics schema=1 status=unavailable");
            let status = std::process::Command::new(program)
                .args(args)
                .env("SOPHIA_DIAGNOSTICS_DISABLED", "true")
                .env_remove("SOPHIA_DIAGNOSTIC_DIR")
                .env_remove("SOPHIA_DIAGNOSTIC_SESSION")
                .status()?;
            return Ok(status
                .code()
                .or_else(|| status.signal().map(|signal| 128 + signal))
                .unwrap_or(1));
        }
    };
    let child = std::process::Command::new(program)
        .args(args)
        .env("SOPHIA_DIAGNOSTIC_SESSION", &record.id)
        .env("SOPHIA_DIAGNOSTIC_DIR", &record.path)
        .spawn();
    let mut child = match child {
        Ok(child) => child,
        Err(error) => {
            let _ = store.finish(&record.id, Some(127));
            return Err(error);
        }
    };
    // The TTY wrapper, rather than this waiting parent, is the live owner. Its
    // guard remains independent if this process is killed during the session.
    let _ = store.transfer_owner(&record.id, child.id());
    let status = child.wait()?;
    let code = status
        .code()
        .or_else(|| status.signal().map(|signal| 128 + signal))
        .unwrap_or(1);
    // Recording trouble must not replace the desktop's real exit status.
    if store.finish(&record.id, Some(code)).is_err() {
        crate::session_eprintln!("sophia_session_diagnostics schema=1 status=unavailable");
    }
    Ok(code)
}
