//! Dummy-probe telemetry only: atomic, nonblocking pipe writes below PIPE_BUF.
use std::fs::File;
use std::io::Write;
use std::os::fd::FromRawFd;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{mpsc, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const LIMIT: u64 = 65_536;
struct Trace {
    file: File,
    start: Instant,
    next: AtomicU64,
    dropped: AtomicU64,
    stopped: AtomicBool,
}
static TRACE: OnceLock<Trace> = OnceLock::new();

pub fn mark(stage: &'static str, value: u64) {
    let Some(trace) = TRACE.get() else { return };
    let seq = trace.next.fetch_add(1, Ordering::Relaxed);
    if seq >= LIMIT && stage != "heartbeat" && stage != "process_exit" {
        trace.dropped.fetch_add(1, Ordering::Relaxed);
        return;
    }
    let wall = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let line = format!(
        "t082 {seq} {} {:?} {} {wall} {stage} {value} {}\n",
        std::process::id(),
        std::thread::current().id(),
        trace.start.elapsed().as_nanos(),
        trace.dropped.load(Ordering::Relaxed)
    );
    // All callers use fixed stage names and numeric metadata. A single write
    // smaller than PIPE_BUF either commits the entire record or returns EAGAIN.
    if line.len() > 512 || (&trace.file).write(line.as_bytes()).ok() != Some(line.len()) {
        trace.dropped.fetch_add(1, Ordering::Relaxed);
    }
}

pub struct Guard {
    stop: mpsc::Sender<()>,
    worker: Option<std::thread::JoinHandle<()>>,
}
pub fn start() -> Guard {
    assert_eq!(
        std::env::var("T082_DUMMY_PROBE").as_deref(),
        Ok("1"),
        "dummy harness required"
    );
    let fd: i32 = std::env::var("T082_TRACE_FD")
        .expect("trace pipe required")
        .parse()
        .expect("trace fd");
    assert!(fd >= 3, "trace descriptor must be separate from stdio");
    // The harness owns creation and sets O_NONBLOCK before passing this fd.
    let file = unsafe { File::from_raw_fd(fd) };
    use std::os::unix::fs::FileTypeExt;
    assert!(file
        .metadata()
        .expect("trace metadata")
        .file_type()
        .is_fifo());
    assert!(TRACE
        .set(Trace {
            file,
            start: Instant::now(),
            next: AtomicU64::new(0),
            dropped: AtomicU64::new(0),
            stopped: AtomicBool::new(false)
        })
        .is_ok());
    mark("process_start", 0);
    let (stop, receiver) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        while receiver.recv_timeout(Duration::from_millis(100)).is_err() {
            if TRACE.get().unwrap().stopped.load(Ordering::Relaxed) {
                break;
            }
            mark("heartbeat", 0);
        }
    });
    Guard {
        stop,
        worker: Some(worker),
    }
}
impl Drop for Guard {
    fn drop(&mut self) {
        TRACE.get().unwrap().stopped.store(true, Ordering::Relaxed);
        let _ = self.stop.send(());
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
        mark("process_exit", 0);
    }
}

// Opaque per-thread connection ordinals, never native pointers in the trace.
// Fixed storage keeps diagnostics bounded; zero denotes identity overflow.
pub fn connection(pointer: usize) {
    thread_local! {
        static CONNECTIONS: std::cell::RefCell<[usize; 16]> = const { std::cell::RefCell::new([0; 16]) };
    }
    let id = CONNECTIONS.with(|slots| {
        let mut slots = slots.borrow_mut();
        if let Some(index) = slots.iter().position(|p| *p == pointer) {
            return index as u64 + 1;
        }
        if let Some(index) = slots.iter().position(|p| *p == 0) {
            slots[index] = pointer;
            return index as u64 + 1;
        }
        0
    });
    mark("xcb_connection", id);
}
