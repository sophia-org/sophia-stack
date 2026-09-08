#![cfg(target_os = "linux")]

use sophia_x_authority::{
    X_DRI3_MAJOR_OPCODE, X_DRI3_OPEN_MINOR_OPCODE, X_SETUP_DEFAULT_ROOT, XByteOrder, XClientReply,
    XSetupFailure, XSetupSuccess, encode_x_client_reply, encode_x11_setup_failure,
    encode_x11_setup_success,
};
use std::{
    fs::{self, File},
    io::{self, IoSlice, Read, Write},
    mem::MaybeUninit,
    os::{
        fd::AsFd,
        unix::{
            fs::PermissionsExt,
            net::{UnixListener, UnixStream},
        },
    },
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
    thread::JoinHandle,
    time::{Duration, Instant},
};

const COOKIE: [u8; 16] = [0x51; 16];
const AUTH_NAME: &[u8] = b"MIT-MAGIC-COOKIE-1";
static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
enum Scenario {
    MissingExtension,
    MissingFd,
    WrongFd,
    WrongFdCount,
    WrongCookie,
    MissingCookie,
    Timeout,
    Device(PathBuf),
}

#[derive(Default, Debug)]
struct Observed {
    cookie_rejected: bool,
    closed_without_setup: bool,
    authenticated: bool,
    queried_dri3: bool,
    opened_root: bool,
}

struct Fixture {
    root: PathBuf,
    socket: PathBuf,
    stop: mpsc::Sender<()>,
    worker: Option<JoinHandle<Result<(), String>>>,
    observed: Arc<Mutex<Observed>>,
}

impl Fixture {
    fn new(scenario: Scenario) -> Self {
        let root = std::env::temp_dir().join(format!(
            "sophia-client-launch-{}-{}",
            std::process::id(),
            NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
        let socket = root.join("X0");
        let listener = UnixListener::bind(&socket).unwrap();
        listener.set_nonblocking(true).unwrap();
        let mut authority = u16::MAX.to_be_bytes().to_vec();
        let cookie = if matches!(scenario, Scenario::WrongCookie) {
            [0x73; 16]
        } else {
            COOKIE
        };
        for field in [
            b"".as_slice(),
            b"0".as_slice(),
            AUTH_NAME,
            cookie.as_slice(),
        ] {
            authority.extend_from_slice(&u16::try_from(field.len()).unwrap().to_be_bytes());
            authority.extend_from_slice(field);
        }
        if matches!(scenario, Scenario::MissingCookie) {
            authority.clear();
        }
        fs::write(root.join("authority"), authority).unwrap();
        fs::set_permissions(root.join("authority"), fs::Permissions::from_mode(0o600)).unwrap();
        fs::write(
            root.join("target"),
            "#!/bin/sh\nprintf executed > \"$CLIENT_LAUNCH_CAPTURE\"\n",
        )
        .unwrap();
        fs::set_permissions(root.join("target"), fs::Permissions::from_mode(0o700)).unwrap();
        let observed = Arc::new(Mutex::new(Observed::default()));
        let worker_observed = observed.clone();
        let (stop, stopped) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        match stopped.recv_timeout(Duration::from_millis(5)) {
                            Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(()),
                            Err(mpsc::RecvTimeoutError::Timeout) => {}
                        }
                    }
                    Err(error) => return Err(error.to_string()),
                }
            };
            serve(stream, scenario, &worker_observed).map_err(|error| error.to_string())
        });
        Self {
            root,
            socket,
            stop,
            worker: Some(worker),
            observed,
        }
    }

    fn command(&self, check_only: bool) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_sophia"));
        command.args(["client-launch", "--adapter=chromium", "--argv-style=direct"]);
        if check_only {
            command.arg("--check-only");
        }
        command.arg("--").arg(self.root.join("target"));
        command
            .env("DISPLAY", format!("unix:{}", self.socket.display()))
            .env("XAUTHORITY", self.root.join("authority"))
            .env("CLIENT_LAUNCH_CAPTURE", self.root.join("executed"));
        command
    }

    fn run(&self, mut command: Command) -> Run {
        let stdout = self.root.join("stdout");
        let stderr = self.root.join("stderr");
        command
            .stdout(Stdio::from(File::create(&stdout).unwrap()))
            .stderr(Stdio::from(File::create(&stderr).unwrap()));
        let started = Instant::now();
        let mut child = command.spawn().unwrap();
        let pid = child.id();
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break status;
            }
            if started.elapsed() > Duration::from_secs(3) {
                child.kill().unwrap();
                child.wait().unwrap();
                panic!("client-launch exceeded the independent three-second watchdog");
            }
            std::thread::sleep(Duration::from_millis(5));
        };
        Run {
            status,
            pid,
            elapsed: started.elapsed(),
            stdout: fs::read_to_string(stdout).unwrap(),
            stderr: fs::read_to_string(stderr).unwrap(),
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(worker) = self.worker.take() {
            let result = worker.join();
            if !std::thread::panicking() {
                result.unwrap().unwrap();
            }
        }
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[derive(Debug)]
struct Run {
    status: ExitStatus,
    pid: u32,
    elapsed: Duration,
    stdout: String,
    stderr: String,
}

fn read16(order: XByteOrder, bytes: &[u8]) -> u16 {
    let bytes = bytes.try_into().unwrap();
    match order {
        XByteOrder::LittleEndian => u16::from_le_bytes(bytes),
        XByteOrder::BigEndian => u16::from_be_bytes(bytes),
    }
}

fn read32(order: XByteOrder, bytes: &[u8]) -> u32 {
    let bytes = bytes.try_into().unwrap();
    match order {
        XByteOrder::LittleEndian => u32::from_le_bytes(bytes),
        XByteOrder::BigEndian => u32::from_be_bytes(bytes),
    }
}

fn request(stream: &mut UnixStream, order: XByteOrder) -> io::Result<Vec<u8>> {
    let mut bytes = vec![0; 4];
    stream.read_exact(&mut bytes)?;
    let length = usize::from(read16(order, &bytes[2..4])) * 4;
    assert!((4..=4096).contains(&length));
    bytes.resize(length, 0);
    stream.read_exact(&mut bytes[4..])?;
    Ok(bytes)
}

fn wait_for_disconnect(stream: &mut UnixStream) -> io::Result<()> {
    let mut byte = [0];
    assert_eq!(
        stream.read(&mut byte)?,
        0,
        "discovery must end without additional requests"
    );
    Ok(())
}

fn serve(mut stream: UnixStream, scenario: Scenario, observed: &Mutex<Observed>) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    if matches!(scenario, Scenario::MissingCookie) {
        wait_for_disconnect(&mut stream)?;
        observed.lock().unwrap().closed_without_setup = true;
        return Ok(());
    }
    let mut setup = [0; 12];
    stream.read_exact(&mut setup)?;
    let order = match setup[0] {
        b'l' => XByteOrder::LittleEndian,
        b'B' => XByteOrder::BigEndian,
        other => panic!("unexpected byte order {other}"),
    };
    assert_eq!(read16(order, &setup[2..4]), 11);
    let name_len = usize::from(read16(order, &setup[6..8]));
    let cookie_len = usize::from(read16(order, &setup[8..10]));
    assert!(name_len <= 256 && cookie_len <= 256);
    let padded_name = (name_len + 3) & !3;
    let mut auth = vec![0; padded_name + ((cookie_len + 3) & !3)];
    stream.read_exact(&mut auth)?;
    assert_eq!(
        &auth[..name_len],
        AUTH_NAME,
        "the helper must authenticate rather than falling back to an empty setup"
    );
    if auth[padded_name..padded_name + cookie_len] != COOKIE {
        assert!(matches!(scenario, Scenario::WrongCookie));
        observed.lock().unwrap().cookie_rejected = true;
        stream.write_all(
            &encode_x11_setup_failure(
                order,
                &XSetupFailure::new(b"authentication failed".to_vec()),
            )
            .unwrap(),
        )?;
        return Ok(());
    }
    observed.lock().unwrap().authenticated = true;
    stream.write_all(
        &encode_x11_setup_success(order, &XSetupSuccess::client_compatible()).unwrap(),
    )?;
    let query = request(&mut stream, order)?;
    assert_eq!(query[0], 98);
    let name_len = usize::from(read16(order, &query[4..6]));
    assert_eq!(&query[8..8 + name_len], b"DRI3");
    observed.lock().unwrap().queried_dri3 = true;
    stream.write_all(&encode_x_client_reply(
        order,
        XClientReply::QueryExtension {
            sequence: 1,
            present: !matches!(scenario, Scenario::MissingExtension),
            major_opcode: X_DRI3_MAJOR_OPCODE,
            first_event: 0,
            first_error: 0,
        },
    ))?;
    if matches!(scenario, Scenario::MissingExtension) {
        return wait_for_disconnect(&mut stream);
    }
    let open = request(&mut stream, order)?;
    assert_eq!(
        (open[0], open[1], open.len()),
        (X_DRI3_MAJOR_OPCODE, X_DRI3_OPEN_MINOR_OPCODE, 12)
    );
    assert_eq!(read32(order, &open[4..8]), X_SETUP_DEFAULT_ROOT);
    assert_eq!(read32(order, &open[8..12]), 0);
    observed.lock().unwrap().opened_root = true;
    if matches!(scenario, Scenario::Timeout) {
        return wait_for_disconnect(&mut stream);
    }
    let mut reply = encode_x_client_reply(order, XClientReply::Dri3Open { sequence: 2 });
    if matches!(scenario, Scenario::MissingFd) {
        stream.write_all(&reply)?;
    } else {
        let path = match &scenario {
            Scenario::Device(path) => path.as_path(),
            _ => Path::new("/dev/null"),
        };
        let file = File::open(path)?;
        let duplicate = file.try_clone()?;
        let mut fds = vec![file.as_fd()];
        if matches!(scenario, Scenario::WrongFdCount) {
            reply[1] = 2;
            fds.push(duplicate.as_fd());
        }
        let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(2))];
        let mut ancillary = rustix::net::SendAncillaryBuffer::new(&mut space);
        assert!(ancillary.push(rustix::net::SendAncillaryMessage::ScmRights(&fds)));
        let sent = rustix::net::sendmsg(
            &stream,
            &[IoSlice::new(&reply)],
            &mut ancillary,
            rustix::net::SendFlags::empty(),
        )?;
        stream.write_all(&reply[sent..])?;
    }
    wait_for_disconnect(&mut stream)
}

#[test]
fn discovery_authenticates_and_refuses_missing_or_invalid_device_evidence() {
    for scenario in [
        Scenario::MissingExtension,
        Scenario::MissingFd,
        Scenario::WrongFd,
        Scenario::WrongFdCount,
    ] {
        let fixture = Fixture::new(scenario.clone());
        let output = fixture.run(fixture.command(false));
        assert!(!output.status.success(), "{output:?}");
        assert!(
            !fixture.root.join("executed").exists(),
            "the target ran after discovery failed"
        );
        let observed = fixture.observed.lock().unwrap();
        assert!(
            observed.authenticated && observed.queried_dri3,
            "{observed:?}: {output:?}"
        );
        assert_eq!(
            observed.opened_root,
            !matches!(scenario, Scenario::MissingExtension)
        );
    }
}

#[test]
fn a_rejected_cookie_never_executes_the_target() {
    let fixture = Fixture::new(Scenario::WrongCookie);
    let output = fixture.run(fixture.command(false));
    assert!(!output.status.success(), "{output:?}");
    assert!(!fixture.root.join("executed").exists());
    let observed = fixture.observed.lock().unwrap();
    assert!(
        observed.cookie_rejected,
        "the server must reject the presented cookie"
    );
    assert!(!observed.queried_dri3);
    assert!(
        !output.stderr.trim().is_empty(),
        "discovery refusal must report a reason"
    );
}

#[test]
fn a_stalled_authenticated_open_expires_without_exec() {
    for check_only in [false, true] {
        let fixture = Fixture::new(Scenario::Timeout);
        let output = fixture.run(fixture.command(check_only));
        assert!(!output.status.success(), "{output:?}");
        assert!(
            fixture.observed.lock().unwrap().opened_root,
            "the timeout must exercise DRI3 Open"
        );
        assert!(
            output.elapsed < Duration::from_secs(2),
            "the one-second discovery budget was not bounded: {output:?}"
        );
        assert!(!fixture.root.join("executed").exists());
    }
}

#[test]
fn missing_cookie_never_falls_back_to_unauthenticated_setup() {
    let fixture = Fixture::new(Scenario::MissingCookie);
    let output = fixture.run(fixture.command(false));
    assert!(!output.status.success(), "{output:?}");
    assert!(!fixture.root.join("executed").exists());
    // Join the fixture's server before reading its EOF observation.
    let mut fixture = fixture;
    fixture.worker.take().unwrap().join().unwrap().unwrap();
    let observed = fixture.observed.lock().unwrap();
    assert!(observed.closed_without_setup, "{observed:?}");
    assert!(!observed.authenticated && !observed.queried_dri3);
}

fn authority_record(family: u16, address: &[u8], display: &[u8], cookie: &[u8]) -> Vec<u8> {
    let mut record = family.to_be_bytes().to_vec();
    for field in [address, display, AUTH_NAME, cookie] {
        record.extend_from_slice(&u16::try_from(field.len()).unwrap().to_be_bytes());
        record.extend_from_slice(field);
    }
    record
}

#[test]
fn authorization_selects_the_matching_local_hostname_and_display() {
    let fixture = Fixture::new(Scenario::WrongFd);
    let system = rustix::system::uname();
    let hostname = system.nodename().to_bytes();
    let wrong_cookie = [0x29; 16];
    let authority = [
        authority_record(0, &[127, 0, 0, 1], b"0", &wrong_cookie),
        authority_record(256, b"different-test-host", b"0", &wrong_cookie),
        authority_record(256, hostname, b"31337", &wrong_cookie),
        authority_record(256, hostname, b"0", &COOKIE),
        authority_record(u16::MAX, b"", b"999", &wrong_cookie),
    ]
    .concat();
    fs::write(fixture.root.join("authority"), authority).unwrap();
    let output = fixture.run(fixture.command(false));
    assert!(
        !output.status.success(),
        "the deliberately non-DRM fd must be refused: {output:?}"
    );
    assert!(!fixture.root.join("executed").exists());
    let observed = fixture.observed.lock().unwrap();
    assert!(
        observed.authenticated && observed.opened_root,
        "wrong authority record selected: {observed:?}; {output:?}"
    );
}

fn hardware_node() -> PathBuf {
    let path = PathBuf::from(
        std::env::var_os("SOPHIA_CLIENT_LAUNCH_TEST_DEVICE")
            .expect("set SOPHIA_CLIENT_LAUNCH_TEST_DEVICE to a DRM render node"),
    );
    fs::canonicalize(path).unwrap()
}

#[test]
#[ignore = "requires an explicitly selected DRM render node"]
fn check_only_reports_the_authenticated_advertised_render_device() {
    let device = hardware_node();
    let fixture = Fixture::new(Scenario::Device(device.clone()));
    let output = fixture.run(fixture.command(true));
    assert!(output.status.success(), "{output:?}");
    assert!(
        output
            .stdout
            .contains("sophia_client_launch schema=1 adapter=chromium status=adapted"),
        "{output:?}"
    );
    assert!(
        output
            .stdout
            .contains(&format!("render_node={}", device.display())),
        "{output:?}"
    );
    assert!(!fixture.root.join("executed").exists());
    assert!(fixture.observed.lock().unwrap().opened_root);
}

fn install_capture_target(fixture: &Fixture) {
    fs::write(fixture.root.join("target"), r#"#!/usr/bin/python3
import os
with open(os.environ["CLIENT_LAUNCH_CAPTURE"], "w") as result:
    result.write("pid=" + str(os.getpid()) + "\n")
    result.write("argv=" + ",".join(os.fsencode(arg).hex() for arg in __import__("sys").argv[1:]) + "\n")
    for fd in os.listdir("/proc/self/fd"):
        try:
            target = os.readlink("/proc/self/fd/" + fd)
        except FileNotFoundError:
            continue
        if target.startswith("socket:") or target.startswith("/dev/dri/") or target.endswith("/authority"):
            result.write("leaked=" + target + "\n")
"#).unwrap();
}

#[test]
#[ignore = "requires an explicitly selected DRM render node and Python 3"]
fn exec_preserves_pid_and_arguments_and_closes_discovery_descriptors() {
    let device = hardware_node();
    let fixture = Fixture::new(Scenario::Device(device.clone()));
    install_capture_target(&fixture);
    let arguments = ["literal space", "", "--", "dollar$and`backtick"];
    let mut command = fixture.command(false);
    command.args(arguments);
    let output = fixture.run(command);
    assert!(output.status.success(), "{output:?}");
    let capture = fs::read_to_string(fixture.root.join("executed")).unwrap();
    assert!(
        capture
            .lines()
            .any(|line| line == format!("pid={}", output.pid)),
        "{capture}"
    );
    let injected = format!("--render-node-override={}", device.display());
    let expected = [
        arguments[0],
        arguments[1],
        injected.as_str(),
        arguments[2],
        arguments[3],
    ]
    .into_iter()
    .map(|argument| {
        argument
            .as_bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    })
    .collect::<Vec<_>>()
    .join(",");
    assert!(
        capture
            .lines()
            .any(|line| line == format!("argv={expected}")),
        "{capture}"
    );
    assert!(!capture.contains("leaked="), "{capture}");
}

#[test]
#[ignore = "requires an explicitly selected DRM render node and Python 3"]
fn an_explicit_matching_device_is_preserved_without_an_extra_switch() {
    let device = hardware_node();
    let fixture = Fixture::new(Scenario::Device(device.clone()));
    install_capture_target(&fixture);
    let explicit = format!("--render-node-override={}", device.display());
    let arguments = [explicit.as_str(), "literal space", "--", "unchanged"];
    let mut command = fixture.command(false);
    command.args(arguments);
    let output = fixture.run(command);
    assert!(output.status.success(), "{output:?}");
    let capture = fs::read_to_string(fixture.root.join("executed")).unwrap();
    let expected = arguments
        .into_iter()
        .map(|argument| {
            argument
                .as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(",");
    assert!(
        capture
            .lines()
            .any(|line| line == format!("argv={expected}")),
        "{capture}"
    );
    assert!(
        capture
            .lines()
            .any(|line| line == format!("pid={}", output.pid)),
        "{capture}"
    );
    assert!(!capture.contains("leaked="), "{capture}");
}

#[test]
#[ignore = "requires two explicitly selected DRM render nodes on different GPUs"]
fn an_explicit_other_gpu_is_refused_before_exec() {
    let device = hardware_node();
    let other = fs::canonicalize(
        std::env::var_os("SOPHIA_CLIENT_LAUNCH_OTHER_DEVICE")
            .expect("set SOPHIA_CLIENT_LAUNCH_OTHER_DEVICE to a render node on a different GPU"),
    )
    .unwrap();
    let device_parent = |node: &Path| {
        fs::canonicalize(
            Path::new("/sys/class/drm")
                .join(node.file_name().unwrap())
                .join("device"),
        )
        .unwrap()
    };
    assert_ne!(
        device_parent(&device),
        device_parent(&other),
        "the conflict probe needs distinct GPUs"
    );
    for check_only in [false, true] {
        let fixture = Fixture::new(Scenario::Device(device.clone()));
        let mut command = fixture.command(check_only);
        command.arg(format!("--render-node-override={}", other.display()));
        let output = fixture.run(command);
        assert!(!output.status.success(), "{output:?}");
        assert!(fixture.observed.lock().unwrap().opened_root);
        assert!(
            !fixture.root.join("executed").exists(),
            "a conflicting explicit device must not execute the target"
        );
    }
}
