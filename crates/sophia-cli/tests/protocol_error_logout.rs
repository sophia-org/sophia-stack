#![cfg(feature = "native-session")]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "sophia-protocol-error-logout-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
        Self(root)
    }

    fn write(&self, name: &str, contents: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, contents).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn rejected_x_request_does_not_fail_normal_session_logout() {
    let fixture = Fixture::new();
    let core = fixture.write("core.kdl", "schema 2\n");
    let profile = fixture.write(
        "desktop.kdl",
        "schema 1\nshell { enabled #false; }\nsession { startup \"wire\"; }\n",
    );
    // The child receives this test session's display and authorization. Checking
    // both replies on one connection proves rejection did not terminate intake.
    let client = fixture.write(
        "wire.py",
        r#"import os
import socket
import struct

with open(os.environ["XAUTHORITY"], "rb") as authority:
    authority.read(2)  # FamilyLocal
    fields = []
    for _ in range(4):
        length = struct.unpack(">H", authority.read(2))[0]
        fields.append(authority.read(length))
name, cookie = fields[2:]
assert name == b"MIT-MAGIC-COOKIE-1"

def padded(data):
    return data + b"\0" * (-len(data) % 4)

with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as connection:
    connection.settimeout(2)
    connection.connect("/tmp/.X11-unix/X" + os.environ["DISPLAY"][1:])

    def receive(length):
        data = b""
        while len(data) < length:
            chunk = connection.recv(length - len(data))
            assert chunk, "X connection closed before the reply"
            data += chunk
        return data

    connection.sendall(struct.pack("<BBHHHHH", ord("l"), 0, 11, 0,
                                   len(name), len(cookie), 0)
                       + padded(name) + padded(cookie))
    setup = receive(8)
    assert setup[0] == 1, "X setup rejected"
    receive(struct.unpack_from("<H", setup, 6)[0] * 4)

    connection.sendall(struct.pack("<BBH", 255, 0, 1))
    rejected = receive(32)
    assert rejected[0:2] == bytes([0, 1]), "expected BadRequest"
    assert struct.unpack_from("<H", rejected, 2)[0] == 1
    assert rejected[10] == 255

    connection.sendall(struct.pack("<BBH", 43, 0, 1))  # GetInputFocus
    reply = receive(32)
    assert reply[0] == 1, "valid request did not receive a reply"
    assert struct.unpack_from("<H", reply, 2)[0] == 2
    print("wire_client_verified_rejection_and_next_reply", flush=True)
"#,
    );
    let output = Command::new(env!("CARGO_BIN_EXE_sophia"))
        .args([
            "session",
            "run",
            "--session-mode=normal",
            "--no-input",
            "--session-app=wire=/usr/bin/python3",
            "--session-action-app=terminal=wire",
            // A finite harness lifetime does not impose a daily-session deadline.
            "--max-runtime-ms=3000",
        ])
        .arg(format!("--session-app-arg=wire={}", client.display()))
        .arg(format!("--config={}", core.display()))
        .arg(format!("--desktop-profile={}", profile.display()))
        .arg(format!("--display=:{}", 40000 + std::process::id() % 10000))
        .env("XDG_CONFIG_HOME", &fixture.0)
        .env("XDG_RUNTIME_DIR", &fixture.0)
        .env_remove("SOPHIA_DIAGNOSTIC_DIR")
        .env_remove("SOPHIA_DIAGNOSTIC_SESSION")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stdout.contains("wire_client_verified_rejection_and_next_reply"),
        "{stdout}\n{stderr}"
    );
    assert!(
        stdout.contains("major=255 minor=0 code=1 count=1"),
        "{stdout}\n{stderr}"
    );
    assert!(output.status.success(), "{stdout}\n{stderr}");
    assert!(
        stdout.contains("sophia_live_session_cleanup schema=1 status=clean"),
        "{stdout}\n{stderr}"
    );
}
