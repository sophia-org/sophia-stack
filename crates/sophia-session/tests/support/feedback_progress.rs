use crate::live_session::{
    XPresentSessionObserver, authority_batch_has_engine_work,
    native_frame_service_should_preempt_authority, wm_update_coordinator_batch,
};
use sophia_backend_live::{
    LivePresentBufferDisposition, LivePresentFeedbackOutcome, LivePresentProtocolFeedback,
    LiveProductionVisualRuntime,
};
use sophia_engine::{
    HeadlessOutput, OutputFrameServiceObservation, OutputFrameServiceRequest,
    OutputNativeFramePhase,
};
use sophia_protocol::{NamespaceId, OutputId, Size, TransactionId};
use sophia_x_authority::{
    X_PRESENT_MAJOR_OPCODE, XServerFrontendConfig, XServerFrontendRouteBroker,
    XServerFrontendServiceCommand, run_x_server_frontend_routed_until_stopped,
};
use std::io::{Read, Write};
use std::num::NonZeroUsize;
use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

struct FeedbackFrontend {
    socket: std::path::PathBuf,
    stop: mpsc::SyncSender<XServerFrontendServiceCommand>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Drop for FeedbackFrontend {
    fn drop(&mut self) {
        let _ = self
            .stop
            .try_send(XServerFrontendServiceCommand::StopAccepting);
        if let Some(thread) = self.thread.take() {
            let result = thread.join();
            if !std::thread::panicking() {
                result.expect("feedback frontend exits cleanly");
            }
        }
        let _ = std::fs::remove_file(&self.socket);
    }
}

fn request(opcode: u8, minor: u8, size: usize) -> Vec<u8> {
    let mut bytes = vec![0; size];
    bytes[0] = opcode;
    bytes[1] = minor;
    bytes[2..4].copy_from_slice(&u16::try_from(size / 4).unwrap().to_le_bytes());
    bytes
}

fn put(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn packet(stream: &mut UnixStream) -> Vec<u8> {
    let mut bytes = vec![0; 32];
    stream.read_exact(&mut bytes).expect("frontend packet");
    assert_ne!(bytes[0], 0, "unexpected X error: {bytes:?}");
    if matches!(bytes[0], 1 | 35) {
        bytes.resize(32 + usize::try_from(u32_at(&bytes, 4)).unwrap() * 4, 0);
        stream.read_exact(&mut bytes[32..]).unwrap();
    }
    bytes
}

#[test]
fn feedback_progress_precedes_no_engine_work_with_a_disarmed_native_deadline() {
    let socket = std::env::temp_dir().join(format!(
        "sophia-feedback-{}-{}.sock",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
    ));
    let broker = XServerFrontendRouteBroker::new(NonZeroUsize::new(32).unwrap());
    let mut observer = XPresentSessionObserver::new(broker.protocol_router());
    let (observations, observed) = mpsc::sync_channel(128);
    let (stop, stopped) = mpsc::sync_channel(1);
    let config = XServerFrontendConfig::new(&socket, NamespaceId::from_raw(997)).unwrap();
    let thread = std::thread::spawn(move || {
        run_x_server_frontend_routed_until_stopped(config, observations, broker, stopped).unwrap();
    });
    // Declared before the client so panic unwinding disconnects it before join.
    let _frontend = FeedbackFrontend {
        socket: socket.clone(),
        stop,
        thread: Some(thread),
    };
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut stream = loop {
        match UnixStream::connect(&socket) {
            Ok(stream) => break stream,
            Err(_) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(1)),
            Err(error) => panic!("feedback frontend did not bind: {error}"),
        }
    };
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    stream
        .write_all(&[b'l', 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let mut prefix = [0; 8];
    stream.read_exact(&mut prefix).unwrap();
    assert_eq!(prefix[0], 1, "X setup succeeds");
    let mut setup = vec![0; usize::from(u16::from_le_bytes([prefix[6], prefix[7]])) * 4];
    stream.read_exact(&mut setup).unwrap();
    let base = u32_at(&setup, 4);
    let vendor_len = usize::from(u16::from_le_bytes([setup[16], setup[17]]));
    let root = u32_at(
        &setup,
        32 + vendor_len.next_multiple_of(4) + usize::from(setup[21]) * 8,
    );
    let window = base | 1;
    let pixmap = base | 2;
    let event_id = base | 3;
    let mut create = request(1, 0, 36);
    put(&mut create, 4, window);
    put(&mut create, 8, root);
    create[16..18].copy_from_slice(&2_u16.to_le_bytes());
    create[18..20].copy_from_slice(&2_u16.to_le_bytes());
    create[22..24].copy_from_slice(&1_u16.to_le_bytes());
    put(&mut create, 28, 1 << 9);
    put(&mut create, 32, 1);
    stream.write_all(&create).unwrap();
    let mut map = request(8, 0, 8);
    put(&mut map, 4, window);
    stream.write_all(&map).unwrap();
    let mut create_pixmap = request(53, 24, 16);
    put(&mut create_pixmap, 4, pixmap);
    put(&mut create_pixmap, 8, window);
    create_pixmap[12..14].copy_from_slice(&2_u16.to_le_bytes());
    create_pixmap[14..16].copy_from_slice(&2_u16.to_le_bytes());
    stream.write_all(&create_pixmap).unwrap();
    let gc = base | 4;
    let mut create_gc = request(55, 0, 20);
    put(&mut create_gc, 4, gc);
    put(&mut create_gc, 8, pixmap);
    put(&mut create_gc, 12, 1 << 2);
    put(&mut create_gc, 16, 0x214365);
    stream.write_all(&create_gc).unwrap();
    let mut image = request(72, 2, 40);
    put(&mut image, 4, pixmap);
    put(&mut image, 8, gc);
    image[12..14].copy_from_slice(&2_u16.to_le_bytes());
    image[14..16].copy_from_slice(&2_u16.to_le_bytes());
    image[21] = 24;
    image[24..].copy_from_slice(&[0x21, 0x43, 0x65, 0xff].repeat(4));
    stream.write_all(&image).unwrap();
    let mut select = request(X_PRESENT_MAJOR_OPCODE, 3, 16);
    put(&mut select, 4, event_id);
    put(&mut select, 8, window);
    put(&mut select, 12, 6);
    stream.write_all(&select).unwrap();
    let mut present = request(X_PRESENT_MAJOR_OPCODE, 1, 72);
    put(&mut present, 4, window);
    put(&mut present, 8, pixmap);
    put(&mut present, 12, 71);
    stream.write_all(&present).unwrap();
    stream.write_all(&request(43, 0, 4)).unwrap();
    assert_eq!(packet(&mut stream)[0], 1, "Present request was accepted");
    let transaction = loop {
        let batch = observed.recv_timeout(Duration::from_secs(2)).unwrap();
        if !batch.software_present_submissions.is_empty() {
            break batch.transaction;
        }
    };
    let output = OutputId::from_raw(1);
    let mut runtime = LiveProductionVisualRuntime::new(
        &[HeadlessOutput {
            id: output,
            size: Size {
                width: 2,
                height: 2,
            },
            scale: 1,
        }],
        None,
    )
    .unwrap();
    let native_request = OutputFrameServiceRequest {
        outputs: vec![OutputFrameServiceObservation {
            output,
            primary: true,
            native_phase: OutputNativeFramePhase::Idle,
            pending_frame: false,
        }],
        ..Default::default()
    };
    let deadline_armed = false;
    assert!(!native_frame_service_should_preempt_authority(
        &native_request,
        false,
        false,
        0,
        deadline_armed,
    ));
    // Stand in for an already settled retirement, not a frame submission.
    // The real router owns the transaction from the client's Present request.
    runtime.route_present_feedback(LivePresentFeedbackOutcome {
        feedback: vec![
            LivePresentProtocolFeedback::Idle { transaction },
            LivePresentProtocolFeedback::Complete {
                transaction,
                ust: 100,
                msc: 4,
                disposition: LivePresentBufferDisposition::Copied,
            },
        ],
        idle_fence_triggered: false,
    });
    let mut pending = Vec::new();
    let mut skipped = 0;
    for ticket in 100..108 {
        // This is the lifecycle observation boundary used by the owner. The
        // native retirement itself and the include call site remain outside
        // this headless test; neither a timeout nor a WM update rescues it.
        observer
            .drain_pending_feedback(&mut runtime, &mut pending)
            .unwrap();
        let batch = wm_update_coordinator_batch(TransactionId::from_raw(ticket));
        if !authority_batch_has_engine_work(&batch) {
            skipped += 1;
            continue;
        }
        panic!("the fixture must take the no-engine-work path");
    }
    assert_eq!(skipped, 8);
    assert_eq!((observer.idle_routed, observer.complete_routed), (1, 1));
    let idle = packet(&mut stream);
    let complete = packet(&mut stream);
    assert_eq!(
        (idle[0], idle[1], u16::from_le_bytes([idle[8], idle[9]])),
        (35, X_PRESENT_MAJOR_OPCODE, 2)
    );
    assert_eq!(
        (
            complete[0],
            complete[1],
            u16::from_le_bytes([complete[8], complete[9]])
        ),
        (35, X_PRESENT_MAJOR_OPCODE, 1)
    );
    assert_eq!(u32_at(&idle, 20), 71);
    assert_eq!(u32_at(&complete, 20), 71);
    // A round trip after both events makes duplicates visible without a sleep.
    stream.write_all(&request(43, 0, 4)).unwrap();
    assert_eq!(
        packet(&mut stream)[0],
        1,
        "no duplicate feedback precedes the reply"
    );
}
