#![cfg(all(target_os = "linux", feature = "libdrm-events", feature = "gbm-probe"))]

use sophia_backend_live::{
    LiveRendererWorkerOutputKey, NativeGbmRenderedScanoutBufferDiscoveryExporter,
    NativeGbmRendererWorkerCore, RenderDeviceDiscoveryBackend,
};
use std::{
    fs::File,
    io,
    os::fd::{AsFd, BorrowedFd},
    process::{Child, Command, Stdio},
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

const DEADLINE: Duration = Duration::from_secs(5);
const CHILD: &str = "SOPHIA_TEST_RENDERER_SHUTDOWN_CHILD";
const DROP_THREAD: &str = "renderer-drop";

struct GatedDevice {
    file: File,
    first_access: AtomicBool,
    entered: mpsc::SyncSender<()>,
    resume: Mutex<mpsc::Receiver<()>>,
}

impl AsFd for GatedDevice {
    fn as_fd(&self) -> BorrowedFd<'_> {
        if !self.first_access.swap(true, Ordering::SeqCst) {
            self.entered.send(()).unwrap();
            self.resume.lock().unwrap().recv_timeout(DEADLINE).unwrap();
        }
        self.file.as_fd()
    }
}

struct UnusedDiscovery;

impl RenderDeviceDiscoveryBackend for UnusedDiscovery {
    type Device = File;

    fn open_render_device(&self) -> io::Result<File> {
        panic!("attaching a shared worker must not open another device")
    }
}

fn wait_for_drop_to_block() {
    let deadline = Instant::now() + DEADLINE;
    loop {
        for entry in std::fs::read_dir("/proc/self/task").unwrap() {
            let path = entry.unwrap().path();
            if std::fs::read_to_string(path.join("comm"))
                .is_ok_and(|name| name.trim() == DROP_THREAD)
                && std::fs::read_to_string(path.join("wchan"))
                    .is_ok_and(|wait| wait.contains("futex"))
            {
                return;
            }
        }
        assert!(Instant::now() < deadline, "core destruction never blocked");
        thread::yield_now();
    }
}

fn exercise_full_queue_shutdown() {
    let (entered, init_started) = mpsc::sync_channel(1);
    let (resume, init_resume) = mpsc::sync_channel(1);
    let core = NativeGbmRendererWorkerCore::spawn(Ok(GatedDevice {
        file: File::open("/dev/null").unwrap(),
        first_access: AtomicBool::new(false),
        entered,
        resume: Mutex::new(init_resume),
    }))
    .unwrap();
    init_started.recv_timeout(DEADLINE).unwrap();

    // Startup is held before command service; each pair queues Register + Deregister.
    for key in 1..=16 {
        let mut exporter = NativeGbmRenderedScanoutBufferDiscoveryExporter::new(UnusedDiscovery);
        exporter.set_output(LiveRendererWorkerOutputKey::from_raw(key));
        exporter.attach_shared_worker(&core);
        drop(exporter);
    }

    let (finished, dropped) = mpsc::sync_channel(1);
    let destruction = thread::Builder::new()
        .name(DROP_THREAD.to_owned())
        .spawn(move || {
            drop(core);
            finished.send(()).unwrap();
        })
        .unwrap();
    // The destructor's send/join is its first blocking operation. Observing it
    // prevents startup from draining the queue before Shutdown is attempted.
    wait_for_drop_to_block();
    resume.send(()).unwrap();
    dropped
        .recv_timeout(DEADLINE)
        .expect("full queue lost Shutdown");
    destruction.join().unwrap();
}

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if !matches!(self.0.try_wait(), Ok(Some(_))) {
            let _ = self.0.kill();
        }
        let _ = self.0.wait();
    }
}

#[test]
fn a_full_command_queue_cannot_discard_worker_shutdown() {
    if std::env::var_os(CHILD).is_some() {
        exercise_full_queue_shutdown();
        return;
    }
    // Isolate the failure: a lost shutdown must not strand the test harness.
    let mut child = ChildGuard(
        Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "a_full_command_queue_cannot_discard_worker_shutdown",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + DEADLINE * 3;
    loop {
        if let Some(status) = child.0.try_wait().unwrap() {
            assert!(status.success(), "shutdown child failed: {status}");
            return;
        }
        if Instant::now() >= deadline {
            panic!("shutdown child exceeded its deadline");
        }
        thread::sleep(Duration::from_millis(5));
    }
}
