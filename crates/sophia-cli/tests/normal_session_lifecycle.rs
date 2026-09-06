#![cfg(feature = "native-session")]

use std::os::unix::fs::PermissionsExt;
use std::process::Command;

#[test]
fn normal_sessions_do_not_need_an_application_frame_to_run_or_finish() {
    let root = std::env::temp_dir().join(format!("sophia-no-startup-proof-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
    let core = root.join("core.kdl");
    std::fs::write(&core, "schema 2\n").unwrap();
    std::fs::set_permissions(&core, std::fs::Permissions::from_mode(0o600)).unwrap();
    let profile = root.join("desktop.kdl");
    for (index, startup) in [
        "startup;",
        "startup \"background\";",
        "startup \"failed\" \"background\";",
        "startup \"missing\" \"background\";",
        "startup \"background\";",
    ]
    .iter()
    .enumerate()
    {
        std::fs::write(
            &profile,
            format!("schema 1\nshell {{ enabled #false; }}\nsession {{ {startup} }}\n"),
        )
        .unwrap();
        std::fs::set_permissions(&profile, std::fs::Permissions::from_mode(0o600)).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_sophia"))
            .args([
                "session",
                "run",
                "--session-mode=normal",
                "--no-input",
                "--session-app=background=/usr/bin/sleep",
                "--session-app-arg=background=20",
                "--session-app=failed=/usr/bin/false",
                "--session-app=missing=/nonexistent/sophia-test-app",
                "--session-action-app=terminal=background",
            ])
            .arg(format!("--config={}", core.display()))
            .arg(format!("--desktop-profile={}", profile.display()))
            .arg(format!(
                "--display=:{}",
                20000 + std::process::id() % 10000 + index as u32
            ))
            .arg(if index == 1 {
                "--max-runtime-ms=8500"
            } else if index == 4 {
                "--max-runtime-ms=500"
            } else {
                "--max-runtime-ms=100"
            })
            .args(if index == 4 {
                vec!["--startup-ready-timeout-ms=100"]
            } else {
                Vec::new()
            })
            .env("XDG_CONFIG_HOME", &root)
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if index == 4 {
            assert!(!output.status.success());
            assert!(
                stderr.contains("startup application was not visibly presented"),
                "{stdout}\n{stderr}"
            );
            assert!(!stdout.contains("status=not_requested"));
            continue;
        }
        assert!(output.status.success(), "{startup}: {stdout}\n{stderr}");
        assert!(stdout.contains("sophia_live_session_startup_proof schema=1 status=not_requested"));
        assert!(stdout.contains("startup_ready_msec=not_requested"));
        assert!(!stdout.contains("sophia_live_session_startup schema=2 status=ready"));
        assert!(stdout.contains("sophia_live_session_cleanup schema=1 status=clean"));
        if index >= 2 {
            assert!(stdout.contains("status=started id=background source=startup"));
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
