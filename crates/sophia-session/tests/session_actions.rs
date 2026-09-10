use sophia_protocol::{SessionApplicationId, SurfaceId, TransactionId};
use sophia_session::session_actions::{
    SESSION_ACTION_APPLICATION_CAPACITY, SessionLaunchCommand, SessionLaunchIntent,
    SessionLaunchQueue, SessionLaunchQueueOutcome,
};
use std::sync::Arc;

fn command(id: &str, placement: Option<u64>) -> Arc<SessionLaunchCommand> {
    Arc::new(SessionLaunchCommand {
        id: id.to_owned(),
        executable: format!("/usr/bin/{id}").into(),
        arguments: vec!["--original".to_owned()],
        placement_classification: placement,
    })
}

#[test]
fn queued_commands_retain_complete_specs_and_are_taken_only_by_exact_admission() {
    let mut queue = SessionLaunchQueue::default();
    let mut selected = command("original", Some(91));
    let retained = Arc::downgrade(&selected);
    queue.enqueue(intent(1), 0);
    queue.enqueue_command(intent(2), selected.clone(), 0);
    Arc::make_mut(&mut selected).executable = "/usr/bin/replacement".into();
    Arc::make_mut(&mut selected).arguments = vec!["--replacement".to_owned()];
    Arc::make_mut(&mut selected).placement_classification = Some(92);
    queue.begin_next(true).unwrap();
    assert!(queue.take_admitted_command(intent(2).transaction).is_none());
    queue.fail_current().unwrap();
    let admitted = queue.begin_next(true).unwrap();
    assert_eq!(admitted.placement_classification, Some(91));
    assert!(queue.take_admitted_command(intent(1).transaction).is_none());
    let captured = queue.take_admitted_command(admitted.transaction).unwrap();
    assert_eq!(captured.executable.to_str(), Some("/usr/bin/original"));
    assert_eq!(captured.arguments, ["--original"]);
    assert_eq!(captured.placement_classification, Some(91));
    assert!(queue.take_admitted_command(admitted.transaction).is_none());
    assert!(queue.complete_spawn(admitted.transaction).is_none());
    assert!(
        queue
            .complete_successful_exit(intent(1).transaction, false)
            .is_none()
    );
    assert!(
        queue
            .complete_successful_exit(admitted.transaction, true)
            .is_none()
    );
    assert!(
        queue
            .complete_successful_exit(admitted.transaction, false)
            .is_some()
    );
    drop(captured);
    assert!(retained.upgrade().is_none());
}

#[test]
fn process_only_spawn_does_not_wait_for_a_window_or_steal_the_next_admission() {
    let mut queue = SessionLaunchQueue::default();
    queue.enqueue_command(intent(1), command("headless", None), 0);
    queue.enqueue_command(intent(2), command("placed", Some(7)), 0);
    let first = queue.begin_next(true).unwrap();
    queue.take_admitted_command(first.transaction).unwrap();
    assert!(queue.complete_spawn(intent(2).transaction).is_none());
    assert!(queue.complete_spawn(first.transaction).is_some());
    let second = queue.begin_next(true).unwrap();
    queue.take_admitted_command(second.transaction).unwrap();
    assert!(
        queue
            .complete_successful_exit(first.transaction, false)
            .is_none()
    );
    assert!(queue.complete_spawn(second.transaction).is_none());
    assert!(
        queue
            .complete_successful_exit(second.transaction, false)
            .is_some()
    );
    assert_eq!(queue.timed_out(), 0);
    assert!(queue.admission().is_none());
}

#[test]
fn command_capacity_cancellation_and_catalog_identity_remain_independent() {
    let mut queue = SessionLaunchQueue::default();
    let command = command("bounded", None);
    let retained = Arc::downgrade(&command);
    for raw in 1..=SESSION_ACTION_APPLICATION_CAPACITY as u64 {
        assert!(matches!(
            queue.enqueue_command(intent(raw), command.clone(), 0),
            SessionLaunchQueueOutcome::Queued { .. }
        ));
    }
    assert_eq!(
        queue.enqueue_command(intent(99), command.clone(), 0),
        SessionLaunchQueueOutcome::RejectedCapacity
    );
    drop(command);
    queue.begin_next(true).unwrap();
    assert_eq!(
        queue.cancel_pending(),
        SESSION_ACTION_APPLICATION_CAPACITY - 1
    );
    assert!(retained.upgrade().is_some());
    queue.fail_current().unwrap();
    assert!(retained.upgrade().is_none());

    queue.enqueue_catalog(intent(100), 0);
    queue.begin_next(true).unwrap();
    assert!(
        queue
            .take_admitted_command(intent(100).transaction)
            .is_none()
    );
    assert!(queue.complete_spawn(intent(100).transaction).is_none());
    assert!(
        queue
            .complete_successful_exit(intent(100).transaction, false)
            .is_none()
    );
    assert!(
        queue
            .complete_successful_exit(intent(100).transaction, true)
            .is_none()
    );
    assert!(queue.dispatch_catalog(intent(100).transaction));
    queue.observe_surface(SurfaceId::new(11, 1)).unwrap();
    assert!(
        queue
            .complete_successful_exit(intent(100).transaction, true)
            .is_some()
    );
}

fn intent(raw: u64) -> SessionLaunchIntent {
    SessionLaunchIntent {
        transaction: TransactionId::from_raw(raw),
        application: SessionApplicationId::from_raw(1),
        placement_classification: None,
    }
}

#[test]
fn launch_burst_is_bounded_without_becoming_an_error() {
    let mut queue = SessionLaunchQueue::default();
    for raw in 1..=32 {
        let outcome = queue.enqueue(intent(raw), 0);
        if raw <= SESSION_ACTION_APPLICATION_CAPACITY as u64 {
            assert!(matches!(outcome, SessionLaunchQueueOutcome::Queued { .. }));
        } else {
            assert_eq!(outcome, SessionLaunchQueueOutcome::RejectedCapacity);
        }
    }

    assert_eq!(queue.pending_len(), SESSION_ACTION_APPLICATION_CAPACITY);
    assert_eq!(queue.rejected(), 16);
}

#[test]
fn launches_advance_only_after_the_observed_surface_is_stable() {
    let mut queue = SessionLaunchQueue::default();
    queue.enqueue(intent(1), 0);
    queue.enqueue(intent(2), 0);

    assert_eq!(queue.begin_next(false), None);
    assert_eq!(queue.begin_next(true), Some(intent(1)));
    assert_eq!(queue.begin_next(true), None);

    let surface = SurfaceId::new(7, 1);
    assert!(queue.observe_surface(surface).is_some());
    assert_eq!(queue.complete_if_stable(false, Some(surface)), None);
    assert_eq!(
        queue
            .complete_if_stable(true, Some(surface))
            .map(|admission| admission.intent),
        Some(intent(1))
    );
    assert_eq!(queue.begin_next(true), Some(intent(2)));
}

#[test]
fn an_observed_application_can_exit_before_the_admission_poll_settles() {
    let mut queue = SessionLaunchQueue::default();
    queue.enqueue(intent(1), 0);
    assert_eq!(queue.begin_next(true), Some(intent(1)));
    assert!(queue.complete_observed_exit().is_none());

    assert!(queue.observe_surface(SurfaceId::new(9, 1)).is_some());
    assert_eq!(
        queue
            .complete_observed_exit()
            .map(|admission| admission.intent),
        Some(intent(1))
    );
    assert!(queue.admission().is_none());
}

#[test]
fn a_multi_toplevel_launch_settles_on_any_presented_observed_surface() {
    let mut queue = SessionLaunchQueue::default();
    queue.enqueue(intent(1), 0);
    assert_eq!(queue.begin_next(true), Some(intent(1)));

    let transient = SurfaceId::new(10, 1);
    let stable = SurfaceId::new(11, 1);
    assert!(queue.observe_surface(transient).is_some());
    assert!(queue.observe_surface(stable).is_some());
    assert!(queue.observe_surface(stable).is_none());
    assert_eq!(queue.complete_if_stable(true, None), None);
    assert_eq!(
        queue
            .complete_if_stable(true, Some(stable))
            .map(|admission| admission.intent),
        Some(intent(1))
    );
}

#[test]
fn timeout_and_logout_release_bounded_work() {
    let mut queue = SessionLaunchQueue::default();
    queue.enqueue(intent(1), 0);
    queue.enqueue(intent(2), 0);
    queue.enqueue(intent(3), 0);
    queue.begin_next(true);

    assert_eq!(
        queue.timeout_current().map(|admission| admission.intent),
        Some(intent(1))
    );
    assert_eq!(queue.timed_out(), 1);
    assert_eq!(queue.cancel_pending(), 2);
    assert_eq!(queue.pending_len(), 0);
}

#[test]
fn a_withdrawn_surface_releases_the_launch_that_was_waiting_for_it() {
    // When the layout coordinator gives up on a surface, the launch that owned
    // it cannot ever settle. Waiting out the remaining admission budget would
    // hold the queue shut behind it, so every later press queues in silence.
    let mut queue = SessionLaunchQueue::default();
    queue.enqueue(intent(1), 0);
    queue.enqueue(intent(2), 0);
    assert_eq!(queue.begin_next(true), Some(intent(1)));

    let surface = SurfaceId::new(7, 1);
    let untouched = SurfaceId::new(8, 1);
    assert!(queue.observe_surface(surface).is_some());

    // A withdrawal naming some other surface is not this launch's business.
    assert!(queue.withdraw_current(&[untouched]).is_none());
    assert_eq!(queue.withdrawn(), 0);

    assert_eq!(
        queue
            .withdraw_current(&[untouched, surface])
            .map(|admission| admission.intent),
        Some(intent(1))
    );
    // Counted apart from a timeout: the deadline was never reached.
    assert_eq!(queue.withdrawn(), 1);
    assert_eq!(queue.timed_out(), 0);

    // The queue moves on rather than staying shut.
    assert_eq!(queue.begin_next(true), Some(intent(2)));
}

#[test]
fn a_trusted_placement_class_is_issued_for_only_the_first_surface() {
    let mut queue = SessionLaunchQueue::default();
    let classified = SessionLaunchIntent {
        placement_classification: Some(7),
        ..intent(1)
    };
    queue.enqueue(classified, 0);
    assert_eq!(queue.begin_next(true), Some(classified));

    let first = queue.observe_surface(SurfaceId::new(7, 1)).unwrap();
    let second = queue.observe_surface(SurfaceId::new(8, 1)).unwrap();
    assert_eq!(first.placement_classification, Some(7));
    assert_eq!(second.placement_classification, None);
}

#[test]
fn a_withdrawal_without_an_outstanding_launch_is_not_an_outcome() {
    let mut queue = SessionLaunchQueue::default();
    assert!(queue.withdraw_current(&[SurfaceId::new(7, 1)]).is_none());
    assert_eq!(queue.withdrawn(), 0);
}

#[test]
fn catalog_authority_is_scoped_to_origin_even_when_transactions_collide() {
    let mut queue = SessionLaunchQueue::default();
    let same = intent(7);
    queue.enqueue(same, 0);
    queue.enqueue_catalog(same, 0);
    queue.begin_next(true).unwrap();
    assert!(!queue.dispatch_catalog(same.transaction));
    queue.cancel_catalog(same.transaction);
    assert_eq!(queue.admission().unwrap().intent, same);
    assert_eq!(queue.pending_len(), 0);
    queue.fail_current();
    queue.enqueue_catalog(same, 0);
    queue.begin_next(true).unwrap();
    assert!(queue.dispatch_catalog(same.transaction));
    assert!(!queue.dispatch_catalog(same.transaction));
    assert_eq!(queue.take_catalog_dispatch(), Some(same.transaction));
    queue.cancel_catalog(same.transaction);
    assert!(queue.admission().is_none());
}
