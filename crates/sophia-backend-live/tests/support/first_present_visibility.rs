use super::*;

#[test]
fn first_frame_waits_outside_the_view_without_blocking_another_window() {
    let mut resources = LivePresentationResourceSession::default();
    let mut scheduler = LiveProductionPresentScheduler::default();
    let now = Instant::now();
    let first = SurfaceId::new(201, 1);
    let second = SurfaceId::new(202, 1);
    for (surface, id) in [(first, 201), (second, 202)] {
        let handle = BufferHandle::from_raw(id);
        resources
            .register_source(descriptor(handle), vec![fd()])
            .unwrap();
        let batch = scheduler_batch(TransactionId::from_raw(id), surface, handle);
        scheduler
            .enqueue_group(&batch.groups[0], &[], &mut resources, now)
            .unwrap();
    }
    let first_transaction = TransactionId::from_raw(201);
    assert_eq!(
        scheduler.poll_gate(&mut resources, now).unwrap(),
        LiveProductionPresentGate::Ready(first_transaction)
    );
    let candidate = scheduler.front().unwrap().candidate.key();
    assert!(scheduler.defer_first_visibility(candidate));
    assert_eq!(scheduler.release_first_visibility(&[]), 0);
    // Ordinary layout visibility is insufficient: the animated position can
    // still lie beyond the head even though the settled placement is inside it.
    scheduler.release_layout_deferred_for_surfaces(&[first, second], &[]);
    assert_eq!(scheduler.awaiting_first_visibility().count(), 1);
    assert_eq!(
        scheduler.poll_gate(&mut resources, now).unwrap(),
        LiveProductionPresentGate::Ready(TransactionId::from_raw(202))
    );
    scheduler.pop_front();
    assert_eq!(
        scheduler.poll_gate(&mut resources, now).unwrap(),
        LiveProductionPresentGate::Idle
    );
    let entered = Rect {
        x: 8,
        y: 40,
        width: 64,
        height: 48,
    };
    scheduler.reproject_surface(first, entered);
    assert_eq!(
        scheduler.release_first_visibility(&[SurfaceId::new(201, 2)]),
        0
    );
    assert_eq!(scheduler.release_first_visibility(&[first]), 1);
    assert_eq!(
        scheduler
            .poll_gate(&mut resources, now + Duration::from_secs(3))
            .unwrap(),
        LiveProductionPresentGate::Ready(first_transaction)
    );
    let ready = scheduler.front().unwrap();
    assert_eq!(ready.candidate.key(), candidate);
    assert_eq!(ready.target, entered);
    assert_eq!(ready.candidate.previous_committed_generation, 0);
    assert_eq!(scheduler.controlled_rejections(), 0);
    // Closing the window or ending the session still drains the parked debt.
    assert!(scheduler.defer_first_visibility(candidate));
    assert_eq!(scheduler.drain_transactions(), vec![first_transaction]);
    assert_eq!(scheduler.awaiting_first_visibility().count(), 0);
}
