use std::fs;
use std::path::PathBuf;
use std::process::Command;

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        Self(std::env::temp_dir().join(format!(
                "sophia-cli-diagnostics-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            )))
    }
    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_sophia"));
        command
            .env("XDG_STATE_HOME", &self.0)
            .env_remove("SOPHIA_DIAGNOSTIC_DIR")
            .env_remove("SOPHIA_DIAGNOSTIC_SESSION");
        command
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn failed_wrapper_exit_is_retained_and_marked_without_a_running_desktop() {
    let fixture = Fixture::new();
    let output = fixture
        .command()
        .args([
            "session",
            "_supervise",
            "--profile=test",
            "--",
            "/bin/sh",
            "-c",
            "exit 23",
        ])
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(23),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = fixture
        .command()
        .args([
            "session",
            "mark",
            "--session=latest",
            "previous session crashed",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("marker="));
    let output = fixture
        .command()
        .args(["session", "inspect", "latest"])
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("status=failed"));
    assert!(text.contains("exit_status=23"));
    assert!(text.contains("previous session crashed"));
    assert!(!text.contains("owner_pid="));
    assert!(
        fixture
            .command()
            .args(["session", "keep", "latest"])
            .status()
            .unwrap()
            .success()
    );
}

#[test]
fn diagnostics_failure_does_not_replace_the_child_exit_status() {
    let fixture = Fixture::new();
    fs::create_dir_all(&fixture.0).unwrap();
    fs::write(fixture.0.join("sophia"), "not a directory").unwrap();
    let output = fixture
        .command()
        .args([
            "session",
            "_supervise",
            "--profile=test",
            "--",
            "/bin/sh",
            "-c",
            "exit 19",
        ])
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(19),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("status=unavailable"));
}

#[test]
fn malformed_incident_commands_fail_without_starting_a_session() {
    let fixture = Fixture::new();
    for args in [
        vec!["session", "mark"],
        vec!["session", "mark", "--session=latest", "--session=other"],
        vec!["session", "inspect"],
        vec!["session", "keep"],
    ] {
        let output = fixture.command().args(args).output().unwrap();
        assert!(!output.status.success());
    }
}
