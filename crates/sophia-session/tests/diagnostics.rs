use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::PathBuf;
use std::sync::Arc;

use sophia_session::diagnostics::{Capture, Retention, Store, capture_line, reduced_record};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self(
            std::env::temp_dir().join(format!("sophia-diagnostics-{}-{nonce}", std::process::id())),
        )
    }
    fn store(&self) -> Store {
        Store::open(&self.0, Retention::default()).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn live_owner_selection_failure_and_preservation_survive_new_sessions() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let first = store
        .begin("hagia", std::process::id(), "release_commit=test\n")
        .unwrap();
    assert!(store.begin("hagia", std::process::id(), "").is_err());
    let marker = store.mark(None, "window stopped responding").unwrap();
    assert_eq!(marker.session, first.id);
    store.finish(&first.id, Some(17)).unwrap();
    assert!(store.mark(None, "must select the ended session").is_err());
    assert_eq!(store.select(Some("latest")).unwrap().status, "failed");
    let kept = store.keep(&first.id).unwrap();
    let second = store
        .begin("hagia", std::process::id(), "release_commit=next\n")
        .unwrap();
    assert_eq!(store.select(None).unwrap().id, second.id);
    let result = store.inspect(&first.id, Some(&marker.id)).unwrap();
    assert!(result.manifest.contains("release_commit=test"));
    assert!(result.markers.contains("window stopped responding"));
    assert!(
        fs::read_to_string(kept.join("snapshot"))
            .unwrap()
            .contains("complete=true")
    );
    assert!(kept.join("SHA256SUMS").exists());
}

#[test]
fn dead_owner_and_pid_reuse_are_never_reported_as_live() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let mut child = std::process::Command::new("sleep")
        .arg("60")
        .spawn()
        .unwrap();
    let record = store.begin("test", child.id(), "").unwrap();
    rustix::process::kill_process(
        rustix::process::Pid::from_raw(child.id() as i32).unwrap(),
        rustix::process::Signal::STOP,
    )
    .unwrap();
    assert!(store.mark(None, "owner is stopped").is_ok());
    child.kill().unwrap();
    child.wait().unwrap();
    assert_eq!(
        store.select(Some(&record.id)).unwrap().status,
        "interrupted"
    );
    let marker = store.mark(Some("latest"), "reported after crash").unwrap();
    assert_eq!(marker.session, record.id);
    let manifest = record.path.join("manifest");
    let content = fs::read_to_string(&manifest).unwrap();
    let content = content
        .lines()
        .filter(|l| !l.starts_with("owner_pid=") && !l.starts_with("owner_start_ticks="))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        manifest,
        format!(
            "{content}\nowner_pid={}\nowner_start_ticks=0\n",
            std::process::id()
        ),
    )
    .unwrap();
    assert_eq!(
        store.select(Some(&record.id)).unwrap().status,
        "interrupted"
    );
}

#[test]
fn retention_keeps_active_and_preserved_evidence_but_not_unbounded_daily_copies() {
    let fixture = Fixture::new();
    let store = Store::open(
        &fixture.0,
        Retention {
            finished_sessions: 1,
            bytes: 1024 * 1024,
        },
    )
    .unwrap();
    let first = store.begin("hagia", std::process::id(), "").unwrap();
    store.finish(&first.id, Some(0)).unwrap();
    let kept = store.keep(&first.id).unwrap();
    let second = store.begin("hagia", std::process::id(), "").unwrap();
    store.finish(&second.id, Some(0)).unwrap();
    assert!(!first.path.exists());
    assert!(kept.exists());
    let live = store.begin("hagia", std::process::id(), "").unwrap();
    let tiny = Store::open(
        &fixture.0,
        Retention {
            finished_sessions: 0,
            bytes: 1,
        },
    )
    .unwrap();
    tiny.prune().unwrap();
    assert!(live.path.exists());
    assert!(!second.path.exists());
    assert!(tiny.preserved_bytes().unwrap() > 0);
}

#[test]
fn labels_and_files_do_not_allow_injection_or_follow_links() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let record = store.begin("hagia", std::process::id(), "").unwrap();
    assert!(store.mark(None, "injected\nstatus=passed").is_err());
    assert!(store.mark(None, &"é".repeat(129)).is_err());
    assert!(store.inspect("../outside", None).is_err());
    let outside = fixture.0.join("outside");
    fs::write(&outside, "unchanged").unwrap();
    fs::remove_file(record.path.join("markers.log")).unwrap();
    symlink(&outside, record.path.join("markers.log")).unwrap();
    assert!(store.mark(None, "must fail").is_err());
    assert_eq!(fs::read_to_string(&outside).unwrap(), "unchanged");
    fs::remove_file(record.path.join("markers.log")).unwrap();
    fs::hard_link(&outside, record.path.join("markers.log")).unwrap();
    assert!(store.mark(None, "must also fail").is_err());
    fs::set_permissions(&fixture.0, fs::Permissions::from_mode(0o777)).unwrap();
    assert!(Store::open(&fixture.0, Retention::default()).is_err());
}

#[test]
fn concurrent_markers_remain_distinct_and_ambiguous_sessions_need_selection() {
    let fixture = Fixture::new();
    let store = Arc::new(fixture.store());
    let first = store.begin("one", std::process::id(), "").unwrap();
    let second = store.begin("two", std::process::id(), "").unwrap();
    assert!(store.mark(None, "ambiguous").is_err());
    let threads = (0..8)
        .map(|index| {
            let store = store.clone();
            let id = first.id.clone();
            std::thread::spawn(move || store.mark(Some(&id), &format!("marker {index}")).unwrap())
        })
        .collect::<Vec<_>>();
    let ids = threads
        .into_iter()
        .map(|thread| thread.join().unwrap().id)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ids.len(), 8);
    assert!(
        store
            .inspect(&second.id, Some(ids.first().unwrap()))
            .is_err()
    );
    assert_eq!(
        store
            .inspect(&first.id, None)
            .unwrap()
            .markers
            .lines()
            .count(),
        8
    );
}

#[test]
fn capture_is_bounded_redacted_and_survives_without_completion_records() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let record = store.begin("capture", std::process::id(), "").unwrap();
    // Simulate a full rolling history without allocating a huge test buffer.
    for segment in 0..4 {
        let path = record.path.join(format!("events.{segment}.log"));
        let file = fs::File::create(&path).unwrap();
        file.set_len(15 * 1024 * 1024).unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
    }
    let capture = Capture::start(&record.path).unwrap();
    for sequence in 0..1700 {
        capture_line(&format!(
            "sophia_live_resource_sample schema=1 seq={sequence} rss_kib=123"
        ));
        if sequence % 100 == 0 {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    capture_line(
        "sophia_live_desktop_profile schema=1 status=loaded generation=4 digest=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa title=private namespace=42 peer_pid=99",
    );
    capture_line(
        "sophia_session_profile schema=1 status=activated role=wm generation=5 digest=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    );
    capture_line(
        "sophia_config_reload schema=2 status=applied generation=6 digest=cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
    );
    // Oversized input is counted instead of being allowed to allocate an unbounded queue entry.
    capture_line(&"x".repeat(5000));
    drop(capture);
    let inspection = store.inspect(&record.id, None).unwrap();
    assert!(inspection.identities.contains("generation=4"));
    assert!(inspection.identities.contains("generation=5"));
    assert!(inspection.identities.contains("generation=6"));
    assert!(!inspection.identities.contains("private"));
    assert!(!inspection.identities.contains("peer_pid"));
    let resources = inspection
        .events
        .iter()
        .filter(|line| line.contains("sophia_live_resource_sample "))
        .count();
    let discarded = inspection
        .health
        .lines()
        .find_map(|line| line.strip_prefix("discarded="))
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert!(resources > 0);
    assert_eq!(resources + discarded, 1701); // resources plus the oversized record
    assert!(inspection.health.contains("recording=stopped"));
    assert!(inspection.health.contains("rotated_bytes=15728640"));
    assert!(inspection.record.bytes <= 64 * 1024 * 1024);
    assert!(!inspection.health.contains("discarded=0\n"));
    assert!(!record.path.join("outcome").exists());
}

#[test]
fn payload_values_are_not_copied_from_session_records() {
    let record = reduced_record("sophia_event schema=1 title=123 namespace_id=456 xid=789 error=secret uri=https://example.invalid text=hello count=9 status=failed").unwrap();
    assert_eq!(record, "sophia_event schema=1 count=9 status=failed");
    assert!(reduced_record("arbitrary client stdout").is_none());
}

#[test]
fn protocol_tally_retains_refusal_classification_without_resource_payloads() {
    for status in ["clean", "compatibility_refusals", "degraded"] {
        let safe = format!(
            "sophia_live_session_protocol_error_tally schema=3 status={status} major=144 minor=30 code=1 count=5 distinct=1 discarded=64 total=69"
        );
        assert_eq!(
            reduced_record(&format!(
                "{safe} xid=123 resource_id=456 namespace_id=789 title=private error=secret payload=hidden"
            ))
            .unwrap(),
            safe
        );
    }
    assert_eq!(
        reduced_record("sophia_other_event major=144 minor=30 code=1 distinct=1 discarded=64 total=69 status=compatibility_refusals").unwrap(),
        "sophia_other_event"
    );
}

#[test]
fn protocol_tally_classification_rejects_out_of_range_and_nonnumeric_fields() {
    let prefix = "sophia_live_session_protocol_error_tally";
    let maximums = format!(
        "{prefix} major=255 minor=65535 code=255 distinct=64 discarded={} total={}",
        u64::MAX,
        u64::MAX,
    );
    assert_eq!(reduced_record(&maximums).unwrap(), maximums);
    for field in [
        "major=256",
        "minor=65536",
        "code=256",
        "distinct=65",
        "discarded=18446744073709551616",
        "total=18446744073709551616",
        "major=-1",
        "minor=+1",
        "code=0xff",
        "distinct=1.0",
        "discarded=true",
        "total=none",
        "major=",
        "minor=private",
    ] {
        assert_eq!(
            reduced_record(&format!("{prefix} {field}")).unwrap(),
            prefix,
            "rejected field: {field}"
        );
    }
}

#[test]
fn session_failure_records_preserve_safe_phase_and_cause_without_error_payloads() {
    use sophia_session::diagnostics::{SessionFailurePhase, session_failure_record};

    let error: Box<dyn std::error::Error> =
        "persistent session controls did not drain cleanly".into();
    let record = session_failure_record(SessionFailurePhase::ControlDrain, error.as_ref());
    assert_eq!(
        record,
        "sophia_session_failure schema=1 status=failed phase=control_drain failure_code=session_control_drain"
    );
    assert_eq!(reduced_record(&record).unwrap(), record);

    let private = std::io::Error::other("/private/application/document");
    let record = session_failure_record(SessionFailurePhase::Authority, &private);
    assert_eq!(
        record,
        "sophia_session_failure schema=1 status=failed phase=authority failure_code=unclassified"
    );
    assert_eq!(reduced_record(&record).unwrap(), record);
    assert_eq!(
        reduced_record("sophia_session_failure phase=private_document failure_code=private_document error=private_document").unwrap(),
        "sophia_session_failure"
    );
}

#[test]
fn vt_failure_records_retain_the_boundary_and_safe_cause() {
    use sophia_renderer_live::LiveRendererScanoutBufferExportDetail as Detail;
    use sophia_session::diagnostics::failure_code;

    let error = Detail::WorkerDisconnected;
    let record = format!(
        "sophia_live_renderer_handoff schema=1 status=failed phase=export_images failure_code={} retained_count=5 error=private",
        failure_code(&error),
    );
    assert_eq!(
        reduced_record(&record).unwrap(),
        "sophia_live_renderer_handoff schema=1 status=failed phase=export_images failure_code=renderer_worker_disconnected retained_count=5"
    );
    let missing: Box<dyn std::error::Error> =
        "retained scene refers to an unavailable promoted renderer image".into();
    assert_eq!(failure_code(missing.as_ref()), "handoff_missing_image");
    let private = std::io::Error::other("/private/application/document");
    assert_eq!(failure_code(&private), "unclassified");
    assert_eq!(
        reduced_record("sophia_live_renderer_handoff failure_code=private_document").unwrap(),
        "sophia_live_renderer_handoff"
    );
    for record in [
        "sophia_live_session_vt schema=4 status=preparing",
        "sophia_live_input_epoch schema=1 reason=virtual_terminal epoch=3",
        "sophia_live_session_runtime_fatal schema=1 status=detected source=owner_loop action=bounded_cleanup",
        "sophia_live_session_vt schema=6 status=quiesced outcome=forced_detach_timeout",
    ] {
        assert_eq!(reduced_record(record).unwrap(), record);
    }
}

#[test]
fn marker_windows_follow_the_boot_clock_across_wall_clock_changes() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let record = store.begin("clock", std::process::id(), "").unwrap();
    let boot = fs::read_to_string("/proc/sys/kernel/random/boot_id").unwrap();
    fs::write(
        record.path.join("markers.log"),
        format!("incident\t1\t200000\t{}\tclock changed\n", boot.trim()),
    )
    .unwrap();
    let event_path = record.path.join("events.0.log");
    fs::write(&event_path, "1\t999999999\t200001\tsophia_test schema=1 count=1\n2\t2\t300000\tsophia_test schema=1 count=2\n").unwrap();
    fs::set_permissions(event_path, fs::Permissions::from_mode(0o600)).unwrap();
    let inspection = store.inspect(&record.id, Some("incident")).unwrap();
    assert_eq!(inspection.events.len(), 1);
    assert!(inspection.events[0].contains("count=1"));
    assert_eq!(inspection.window_start_msec, Some(140000));
    store.finish(&record.id, Some(1)).unwrap();
    let marker = store.mark(Some(&record.id), "reported much later").unwrap();
    fs::write(
        record.path.join("markers.log"),
        format!(
            "{}\t1\t400000\t{}\treported later\n",
            marker.id,
            boot.trim()
        ),
    )
    .unwrap();
    let inspection = store.inspect(&record.id, Some(&marker.id)).unwrap();
    assert_eq!(inspection.events.len(), 1);
    assert!(inspection.events[0].contains("count=2"));
}
