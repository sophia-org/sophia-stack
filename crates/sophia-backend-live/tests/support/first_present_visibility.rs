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
    assert!(scheduler.defer_first_visibility(
        candidate,
        LiveProductionFirstVisibilityReason::OutsideHeadFrames,
        now,
    ));
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
    assert!(scheduler.defer_first_visibility(
        candidate,
        LiveProductionFirstVisibilityReason::OutsideHeadFrames,
        now,
    ));
    assert_eq!(scheduler.drain_transactions(), vec![first_transaction]);
    assert_eq!(scheduler.awaiting_first_visibility().count(), 0);
}

#[test]
fn a_parked_first_candidate_gives_up_its_budget_instead_of_waiting_forever() {
    // The wait had no bound and only one way out, so a candidate whose
    // release condition never came true sat in the queue until admission
    // withdrew the client with nothing said about why.
    let mut resources = LivePresentationResourceSession::default();
    let mut scheduler = LiveProductionPresentScheduler::default();
    let now = Instant::now();
    let surface = SurfaceId::new(301, 1);
    let handle = BufferHandle::from_raw(301);
    resources
        .register_source(descriptor(handle), vec![fd()])
        .unwrap();
    let batch = scheduler_batch(TransactionId::from_raw(301), surface, handle);
    scheduler
        .enqueue_group(&batch.groups[0], &[], &mut resources, now)
        .unwrap();
    let transaction = TransactionId::from_raw(301);
    assert_eq!(
        scheduler.poll_gate(&mut resources, now).unwrap(),
        LiveProductionPresentGate::Ready(transaction)
    );
    let candidate = scheduler.front().unwrap().candidate.key();
    assert!(scheduler.defer_first_visibility(
        candidate,
        LiveProductionFirstVisibilityReason::OutsidePresentationOrder,
        now,
    ));
    assert_eq!(scheduler.awaiting_first_visibility().count(), 1);
    assert!(!scheduler.front_first_visibility_exhausted());

    // Nothing releases it, and inside the budget it keeps waiting.
    assert!(
        scheduler
            .expire_first_visibility(now + Duration::from_millis(500))
            .is_empty()
    );
    assert_eq!(scheduler.awaiting_first_visibility().count(), 1);
    assert_eq!(
        scheduler.poll_gate(&mut resources, now).unwrap(),
        LiveProductionPresentGate::Idle
    );

    // Past the budget it returns to the runnable queue, marked, so the
    // ordinary Present path rejects it the way it rejects any candidate that
    // cannot be shown. The reason it was parked for travels with it.
    let expired = scheduler.expire_first_visibility(now + Duration::from_secs(3));
    assert_eq!(
        expired,
        vec![(
            surface,
            LiveProductionFirstVisibilityReason::OutsidePresentationOrder
        )]
    );
    assert_eq!(scheduler.awaiting_first_visibility().count(), 0);
    assert_eq!(
        scheduler
            .poll_gate(&mut resources, now + Duration::from_secs(3))
            .unwrap(),
        LiveProductionPresentGate::Ready(transaction)
    );
    assert!(scheduler.front_first_visibility_exhausted());
    // Expiring twice reports once; the candidate is no longer parked.
    assert!(
        scheduler
            .expire_first_visibility(now + Duration::from_secs(9))
            .is_empty()
    );
}

#[test]
fn each_parking_reason_travels_with_its_candidate() {
    // Release is evaluated against the condition that parked a candidate, so
    // the reason has to survive parking. Deferring for absence from the
    // presentation order and then demanding visibility to leave is the wait
    // that stranded a client.
    let mut resources = LivePresentationResourceSession::default();
    let mut scheduler = LiveProductionPresentScheduler::default();
    let now = Instant::now();
    for (index, reason) in [
        LiveProductionFirstVisibilityReason::OutsidePresentationOrder,
        LiveProductionFirstVisibilityReason::NoApplicableOutput,
        LiveProductionFirstVisibilityReason::OutsideHeadFrames,
    ]
    .into_iter()
    .enumerate()
    {
        let id = 401 + u64::try_from(index).unwrap();
        let surface = SurfaceId::new(u32::try_from(id).unwrap(), 1);
        let handle = BufferHandle::from_raw(id);
        resources
            .register_source(descriptor(handle), vec![fd()])
            .unwrap();
        let batch = scheduler_batch(TransactionId::from_raw(id), surface, handle);
        scheduler
            .enqueue_group(&batch.groups[0], &[], &mut resources, now)
            .unwrap();
        assert_eq!(
            scheduler.poll_gate(&mut resources, now).unwrap(),
            LiveProductionPresentGate::Ready(TransactionId::from_raw(id))
        );
        let candidate = scheduler.front().unwrap().candidate.key();
        assert!(scheduler.defer_first_visibility(candidate, reason, now));
        assert_eq!(
            scheduler
                .awaiting_first_visibility()
                .find(|(parked, _, _)| *parked == surface)
                .map(|(_, _, parked_reason)| parked_reason),
            Some(reason)
        );
    }
    assert_eq!(scheduler.awaiting_first_visibility().count(), 3);
    // All three are bounded by the same budget.
    assert_eq!(
        scheduler
            .expire_first_visibility(now + Duration::from_secs(3))
            .len(),
        3
    );
}

#[test]
fn an_escaped_present_is_removed_by_exact_identity_even_once_parked() {
    // A frame that presented before its window mapped can already be parked by
    // the time the map is acknowledged, so the prompt skip has to be able to
    // take a parked entry as well as a runnable one. It must take only the
    // exact frame: a transaction and surface pair can name more than one
    // source, and the buffer is what separates them.
    let mut resources = LivePresentationResourceSession::default();
    let mut scheduler = LiveProductionPresentScheduler::default();
    let now = Instant::now();
    let surface = SurfaceId::new(501, 1);
    let handle = BufferHandle::from_raw(501);
    resources
        .register_source(descriptor(handle), vec![fd()])
        .unwrap();
    let batch = scheduler_batch(TransactionId::from_raw(501), surface, handle);
    scheduler
        .enqueue_group(&batch.groups[0], &[], &mut resources, now)
        .unwrap();
    let transaction = TransactionId::from_raw(501);
    assert_eq!(
        scheduler.poll_gate(&mut resources, now).unwrap(),
        LiveProductionPresentGate::Ready(transaction)
    );
    let candidate = scheduler.front().unwrap().candidate.key();
    assert!(scheduler.defer_first_visibility(
        candidate,
        LiveProductionFirstVisibilityReason::OutsidePresentationOrder,
        now,
    ));
    assert_eq!(scheduler.awaiting_first_visibility().count(), 1);

    // Another buffer for the same transaction and surface is a different frame
    // and must not be taken.
    assert!(
        scheduler
            .remove_queued_dma_candidate(sophia_protocol::DmaBufPresentKey {
                transaction,
                surface,
                buffer: BufferHandle::from_raw(502),
            })
            .is_none()
    );
    assert_eq!(scheduler.awaiting_first_visibility().count(), 1);

    // The exact frame is taken, parked or not, and leaves nothing behind.
    assert_eq!(
        scheduler.remove_queued_dma_candidate(sophia_protocol::DmaBufPresentKey {
            transaction,
            surface,
            buffer: handle,
        }),
        Some(transaction)
    );
    assert_eq!(scheduler.awaiting_first_visibility().count(), 0);
    assert_eq!(
        scheduler.poll_gate(&mut resources, now).unwrap(),
        LiveProductionPresentGate::Idle
    );
    // Asking twice is a miss, not a second removal.
    assert!(
        scheduler
            .remove_queued_dma_candidate(sophia_protocol::DmaBufPresentKey {
                transaction,
                surface,
                buffer: handle,
            })
            .is_none()
    );
}
