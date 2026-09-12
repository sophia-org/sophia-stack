#[test]
fn pre_admission_pixels_are_quarantined_from_layout_and_runtime() {
    let surface = SurfaceId::new(5, 1);
    let geometry = Rect {
        x: 20,
        y: 30,
        width: 640,
        height: 480,
    };
    let constraints = SurfaceConstraints {
        min_size: None,
        max_size: None,
    };
    let transaction = SurfaceTransaction {
        input_region: None,
        transaction: TransactionId::from_raw(11),
        authority: sophia_protocol::AuthorityKind::SophiaX,
        surface,
        namespace: None,
        target_geometry: geometry,
        presentation_extent: Size {
            width: (geometry).width,
            height: (geometry).height,
        },
        content: sophia_protocol::SurfaceContentSet::singleton(BufferSource::DmaBuf { handle: 44 }, sophia_protocol::Size {
            width: geometry.width,
            height: geometry.height,
        }),

        damage: Region::single(Rect {
            x: 0,
            y: 0,
            width: geometry.width,
            height: geometry.height,
        }),
        readiness: sophia_protocol::SurfaceTransactionReadiness::Ready,
        timeout_msec: 250,
        previous_committed_generation: 0,
    };
    let mut batch =
        crate::live_session::wm_update_coordinator_batch(TransactionId::from_raw(11));
    let client = sophia_x_authority::XServerFrontendClientId::from_raw(1);
    batch.client = Some(client);
    add_test_surface_route(&mut batch, surface, client);
    batch.surface_presentations.push(
        sophia_x_authority::XAuthoritySurfacePresentationObservation {
            surface,
            role: sophia_protocol::SurfacePresentationRole::PolicyManaged,
            kind: sophia_protocol::LayoutNodeKind::Toplevel,
            placement_preference: sophia_protocol::SurfacePlacementPreference::Default,
            stack_rank: 0,
            owner: None,
            mapped: false,
            geometry,
            constraints,
            generation: 1,
        },
    );
    batch
        .presentation_intents
        .push(sophia_protocol::SurfacePresentationIntent {
            surface,
            kind: sophia_protocol::SurfacePresentationIntentKind::Request,
            role: sophia_protocol::SurfacePresentationRole::PolicyManaged,
            surface_kind: sophia_protocol::LayoutNodeKind::Toplevel,
            placement_preference: sophia_protocol::SurfacePlacementPreference::Default,
            presentation_owner: None,
            stack_rank: 0,
            geometry,
            constraints,
            generation: 1,
        });
    batch.transactions.push(transaction.clone());
    batch
        .present_submissions
        .push(sophia_x_authority::XAuthorityPresentSubmission {
            transaction: TransactionId::from_raw(11),
            surface,
            buffer: sophia_protocol::BufferHandle::from_raw(44),
            x_offset: 0,
            y_offset: 0,
            acquire_fence: None,
            idle_fence: None,
        });
    let mut layout = PersistentLiveLayout::default();

    let observation = layout.observe_authority_batch(&batch);
    let (projected, released) = layout.projected_batch(&batch);

    assert_eq!(observation.new_surfaces, vec![surface]);
    assert!(layout.layers.is_empty());
    assert_eq!(
        layout.selected_pre_admission_transaction(
            surface,
            Size {
                width: geometry.width,
                height: geometry.height,
            },
        ),
        Some(&transaction)
    );
    assert!(projected.transactions.is_empty());
    assert!(projected.present_submissions.is_empty());
    assert!(released.is_empty());
    assert_eq!(layout.pre_admission_groups.len(), 1);
    assert!(!observation.admission_group_overflowed);
    assert_eq!(layout.next_unmanaged_surface(), Some(surface));
    // Request and Present arrived in one batch, and intents are applied before
    // the quarantine decision, so the surface was already pending by the time
    // the Present was classified. Nothing escaped, so nothing may be skipped.
    assert!(layout.escaped_pre_admission.is_empty());
}

/// Build a batch carrying one surface's transaction and its Present, with the
/// admission Request included or withheld.
///
/// Withholding it is what a client that presents before mapping produces: the
/// surface is still inactive when the Present is classified.
fn present_batch(
    surface: SurfaceId,
    transaction_id: u64,
    buffer: u64,
    geometry: Rect,
    request_admission: bool,
) -> sophia_x_authority::XAuthorityObservedTransactionBatch {
    let constraints = SurfaceConstraints {
        min_size: None,
        max_size: None,
    };
    let transaction = SurfaceTransaction {
        input_region: None,
        transaction: TransactionId::from_raw(transaction_id),
        authority: sophia_protocol::AuthorityKind::SophiaX,
        surface,
        namespace: None,
        target_geometry: geometry,
        presentation_extent: Size {
            width: geometry.width,
            height: geometry.height,
        },
        content: sophia_protocol::SurfaceContentSet::singleton(
            BufferSource::DmaBuf { handle: buffer },
            sophia_protocol::Size {
                width: geometry.width,
                height: geometry.height,
            },
        ),
        damage: Region::single(Rect {
            x: 0,
            y: 0,
            width: geometry.width,
            height: geometry.height,
        }),
        readiness: sophia_protocol::SurfaceTransactionReadiness::Ready,
        timeout_msec: 250,
        previous_committed_generation: 0,
    };
    let mut batch = crate::live_session::wm_update_coordinator_batch(TransactionId::from_raw(
        transaction_id,
    ));
    let client = sophia_x_authority::XServerFrontendClientId::from_raw(1);
    batch.client = Some(client);
    add_test_surface_route(&mut batch, surface, client);
    batch.surface_presentations.push(
        sophia_x_authority::XAuthoritySurfacePresentationObservation {
            surface,
            role: sophia_protocol::SurfacePresentationRole::PolicyManaged,
            kind: sophia_protocol::LayoutNodeKind::Toplevel,
            placement_preference: sophia_protocol::SurfacePlacementPreference::Default,
            stack_rank: 0,
            owner: None,
            mapped: false,
            geometry,
            constraints,
            generation: 1,
        },
    );
    if request_admission {
        batch
            .presentation_intents
            .push(sophia_protocol::SurfacePresentationIntent {
                surface,
                kind: sophia_protocol::SurfacePresentationIntentKind::Request,
                role: sophia_protocol::SurfacePresentationRole::PolicyManaged,
                surface_kind: sophia_protocol::LayoutNodeKind::Toplevel,
                placement_preference: sophia_protocol::SurfacePlacementPreference::Default,
                presentation_owner: None,
                stack_rank: 0,
                geometry,
                constraints,
                generation: 1,
            });
    }
    batch.transactions.push(transaction);
    batch
        .present_submissions
        .push(sophia_x_authority::XAuthorityPresentSubmission {
            transaction: TransactionId::from_raw(transaction_id),
            surface,
            buffer: sophia_protocol::BufferHandle::from_raw(buffer),
            x_offset: 0,
            y_offset: 0,
            acquire_fence: None,
            idle_fence: None,
        });
    batch
}

#[test]
fn a_present_arriving_before_its_window_maps_is_recorded_as_escaped() {
    // Kitty presents its first frame microseconds before it maps the window.
    // The surface is still inactive, so the quarantine does not hold the frame
    // and production receives one admission has no claim on. A later map does
    // not replay the batch that carried it, so nothing admission does will ever
    // settle it.
    let surface = SurfaceId::new(5, 1);
    let geometry = Rect {
        x: 20,
        y: 30,
        width: 640,
        height: 480,
    };
    let batch = present_batch(surface, 11, 44, geometry, false);
    let mut layout = PersistentLiveLayout::default();

    layout.observe_authority_batch(&batch);
    let (projected, _) = layout.projected_batch(&batch);

    // It escaped: production sees the Present, the quarantine does not hold it.
    assert_eq!(projected.present_submissions.len(), 1);
    assert!(layout.pre_admission_groups.is_empty());

    // Recorded by exact identity, and not yet actionable: the map has not been
    // confirmed, so the client cannot be expected to draw again.
    assert_eq!(layout.escaped_pre_admission.len(), 1);
    assert_eq!(
        layout.escaped_pre_admission[0].key,
        sophia_protocol::DmaBufPresentKey {
            transaction: TransactionId::from_raw(11),
            surface,
            buffer: sophia_protocol::BufferHandle::from_raw(44),
        }
    );
    assert!(!layout.escaped_pre_admission[0].ready);
    assert!(layout.skippable_escaped_presents().is_empty());
}

#[test]
fn an_escaped_present_is_identified_by_its_buffer_not_its_transaction() {
    // One transaction and surface can name more than one source. Skipping on
    // the pair alone would let a backing snapshot stand in for the client's
    // Present, so the buffer is part of the identity.
    let surface = SurfaceId::new(5, 1);
    let geometry = Rect {
        x: 20,
        y: 30,
        width: 640,
        height: 480,
    };
    let batch = present_batch(surface, 11, 44, geometry, false);
    let mut layout = PersistentLiveLayout::default();
    layout.observe_authority_batch(&batch);

    let recorded = layout.escaped_pre_admission[0].key;
    let other_buffer = sophia_protocol::DmaBufPresentKey {
        buffer: sophia_protocol::BufferHandle::from_raw(45),
        ..recorded
    };
    assert_ne!(recorded, other_buffer);
    assert!(!crate::live_session::escaped_key_names_candidate(
        other_buffer,
        sophia_protocol::SurfaceTransactionKey {
            transaction: TransactionId::from_raw(11),
            surface,
            target_buffer: BufferSource::DmaBuf { handle: 44 },
        }
    ));
    assert!(crate::live_session::escaped_key_names_candidate(
        recorded,
        sophia_protocol::SurfaceTransactionKey {
            transaction: TransactionId::from_raw(11),
            surface,
            target_buffer: BufferSource::DmaBuf { handle: 44 },
        }
    ));
}

#[test]
fn a_surface_that_becomes_client_positioned_cannot_skip_its_escaped_present() {
    // A role change removes admission state without clearing the record, so
    // eligibility is rechecked when the skip is asked for rather than trusted
    // from when it was recorded.
    let surface = SurfaceId::new(5, 1);
    let geometry = Rect {
        x: 20,
        y: 30,
        width: 640,
        height: 480,
    };
    let batch = present_batch(surface, 11, 44, geometry, false);
    let mut layout = PersistentLiveLayout::default();
    layout.observe_authority_batch(&batch);
    for escaped in &mut layout.escaped_pre_admission {
        escaped.ready = true;
    }
    assert_eq!(layout.skippable_escaped_presents().len(), 1);

    layout.presentation_roles.insert(
        surface,
        sophia_protocol::SurfacePresentationRole::ClientPositioned,
    );
    // Ineligible, and dropped rather than left to evict an actionable record.
    assert!(layout.skippable_escaped_presents().is_empty());
    assert!(layout.escaped_pre_admission.is_empty());
}


#[test]
fn an_escaped_present_becomes_skippable_only_once_the_map_is_acknowledged() {
    // The map is what makes the client able to draw again after a skip, and the
    // acknowledgement is the first point it is a confirmed fact rather than a
    // request. Acting at the request instead would skip the frame while the
    // window still cannot be shown.
    let surface = SurfaceId::new(5, 1);
    let geometry = Rect {
        x: 20,
        y: 30,
        width: 640,
        height: 480,
    };
    let present = present_batch(surface, 11, 44, geometry, false);
    let mut layout = PersistentLiveLayout::default();
    layout.observe_authority_batch(&present);
    assert_eq!(layout.escaped_pre_admission.len(), 1);

    // The map arrives in a later batch, as it must for the frame to have
    // escaped at all. Requesting admission is not yet acknowledgement.
    let map = present_batch(surface, 12, 45, geometry, true);
    layout.observe_authority_batch(&map);
    assert!(layout.skippable_escaped_presents().is_empty());

    let transaction = TransactionId::from_raw(13);
    let proposal = LiveWmProposal {
        transaction,
        layers: planning_layers_for(&layout, [surface]),
        requested_sizes: BTreeMap::from([(
            surface,
            Size {
                width: geometry.width,
                height: geometry.height,
            },
        )]),
        presentation_states: BTreeMap::new(),
        configure_deliveries: 0,
        focus: Some(surface),
        timeout: Duration::from_secs(1),
        update: sophia_engine::WmTransactionUpdate {
            commit: TransactionCommit {
                transaction,
                outcome: TransactionOutcome::Committed,
                applied_surfaces: vec![surface],
            },
        },
        moved_surfaces: 0,
        source: None,
        policy_settlement: None,
    };
    let mut controls = crate::session_control::SessionControlQueue::default();
    assert!(layout.stage(proposal, &mut controls).unwrap().is_none());
    // Staged and awaiting the authority's answer: still not skippable.
    assert!(layout.skippable_escaped_presents().is_empty());

    // An acknowledgement naming a different transaction is refused, and changes
    // nothing. A stale or forged answer cannot authorize the skip.
    assert!(!layout.acknowledge_admission_control(TransactionId::from_raw(99), surface));
    assert!(layout.skippable_escaped_presents().is_empty());

    assert!(layout.acknowledge_admission_control(transaction, surface));
    assert_eq!(
        layout.skippable_escaped_presents(),
        vec![sophia_protocol::DmaBufPresentKey {
            transaction: TransactionId::from_raw(11),
            surface,
            buffer: sophia_protocol::BufferHandle::from_raw(44),
        }]
    );
    // The same acknowledgement replayed is refused; admission has moved on.
    assert!(!layout.acknowledge_admission_control(transaction, surface));

    // Once production has taken it, the request is finished and not repeated.
    layout.consume_escaped_present(sophia_protocol::DmaBufPresentKey {
        transaction: TransactionId::from_raw(11),
        surface,
        buffer: sophia_protocol::BufferHandle::from_raw(44),
    });
    assert!(layout.skippable_escaped_presents().is_empty());
    assert!(layout.escaped_pre_admission.is_empty());
}


#[test]
fn a_ready_escaped_present_that_admission_has_staged_is_not_skippable() {
    // The guardrail that stops this optimization recreating the retirement debt
    // it was meant to avoid. Once admission stages a candidate it arms
    // retirement from exactly that key when the layout commits, so skipping it
    // here would settle a frame that is no longer ours.
    let surface = SurfaceId::new(5, 1);
    let geometry = Rect {
        x: 20,
        y: 30,
        width: 640,
        height: 480,
    };
    let present = present_batch(surface, 11, 44, geometry, false);
    let mut layout = PersistentLiveLayout::default();
    layout.observe_authority_batch(&present);
    let escaped = layout.escaped_pre_admission[0].key;
    for record in &mut layout.escaped_pre_admission {
        record.ready = true;
    }
    assert_eq!(layout.skippable_escaped_presents(), vec![escaped]);

    // Put the escaped frame in the pending layout as admission's own staged
    // candidate, which is the state commit_pending arms retirement from.
    let transaction = TransactionId::from_raw(13);
    let mut pending = PendingLiveWmLayout {
        transaction,
        layers: Vec::new(),
        requested_sizes: BTreeMap::new(),
        presentation_states: BTreeMap::new(),
        presentation_settlements: BTreeSet::new(),
        configure_deliveries: 0,
        focus: None,
        deadline: Instant::now() + Duration::from_secs(1),
        update: sophia_engine::WmTransactionUpdate {
            commit: TransactionCommit {
                transaction,
                outcome: TransactionOutcome::Committed,
                applied_surfaces: vec![surface],
            },
        },
        moved_surfaces: 0,
        staged_transactions: BTreeMap::new(),
        admission_surfaces: BTreeSet::from([surface]),
        source: None,
        policy_settlement: None,
    };
    pending
        .staged_transactions
        .insert(surface, present.transactions[0].clone());
    layout.pending = Some(pending);

    // Owned by admission now, so the exemption is refused.
    assert!(layout.skippable_escaped_presents().is_empty());

    // A different buffer on the same surface is a different frame, and staging
    // one does not surrender the other. Membership alone is not the test.
    let other = present_batch(surface, 11, 45, geometry, false);
    layout
        .pending
        .as_mut()
        .unwrap()
        .staged_transactions
        .insert(surface, other.transactions[0].clone());
    layout
        .escaped_pre_admission
        .push_back(crate::live_session::EscapedPreAdmissionPresent {
            key: escaped,
            ready: true,
        });
    assert_eq!(layout.skippable_escaped_presents(), vec![escaped]);
}

#[test]
fn a_ready_escaped_present_awaiting_retirement_is_not_skippable() {
    // Admission selected this exact frame after it escaped. Observed pixels are
    // not a claim, but a selected visual candidate is, and settling it here
    // would take a retirement that belongs to admission.
    let surface = SurfaceId::new(5, 1);
    let geometry = Rect {
        x: 20,
        y: 30,
        width: 640,
        height: 480,
    };
    let present = present_batch(surface, 11, 44, geometry, false);
    let mut layout = PersistentLiveLayout::default();
    layout.observe_authority_batch(&present);
    let escaped = layout.escaped_pre_admission[0].key;
    for record in &mut layout.escaped_pre_admission {
        record.ready = true;
    }
    assert_eq!(layout.skippable_escaped_presents(), vec![escaped]);

    // Drive admission to AwaitingRetirement the way it actually gets there:
    // request, stage a control, acknowledge the map, then arm retirement from
    // this very frame.
    let map = present_batch(surface, 12, 45, geometry, true);
    layout.observe_authority_batch(&map);
    let transaction = TransactionId::from_raw(13);
    let proposal = LiveWmProposal {
        transaction,
        layers: planning_layers_for(&layout, [surface]),
        requested_sizes: BTreeMap::from([(
            surface,
            Size {
                width: geometry.width,
                height: geometry.height,
            },
        )]),
        presentation_states: BTreeMap::new(),
        configure_deliveries: 0,
        focus: Some(surface),
        timeout: Duration::from_secs(1),
        update: sophia_engine::WmTransactionUpdate {
            commit: TransactionCommit {
                transaction,
                outcome: TransactionOutcome::Committed,
                applied_surfaces: vec![surface],
            },
        },
        moved_surfaces: 0,
        source: None,
        policy_settlement: None,
    };
    let mut controls = crate::session_control::SessionControlQueue::default();
    assert!(layout.stage(proposal, &mut controls).unwrap().is_none());
    assert!(layout.acknowledge_admission_control(transaction, surface));
    assert!(
        layout
            .admissions
            .begin_retirement(surface, present.transactions[0].key())
    );

    // Selected, so the exemption is refused and the record is dropped rather
    // than left to evict an actionable one.
    assert!(layout.skippable_escaped_presents().is_empty());
}

#[test]
fn a_withdrawn_surface_cannot_authorize_a_skip_with_a_late_acknowledgement() {
    // Withdrawal ends the admission lifecycle that would have authorized the
    // skip. An acknowledgement arriving afterwards names a lifecycle that no
    // longer exists and must not revive the record.
    let surface = SurfaceId::new(5, 1);
    let geometry = Rect {
        x: 20,
        y: 30,
        width: 640,
        height: 480,
    };
    let present = present_batch(surface, 11, 44, geometry, false);
    let mut layout = PersistentLiveLayout::default();
    layout.observe_authority_batch(&present);
    assert_eq!(layout.escaped_pre_admission.len(), 1);

    let mut withdrawal = crate::live_session::wm_update_coordinator_batch(
        TransactionId::from_raw(14),
    );
    let client = sophia_x_authority::XServerFrontendClientId::from_raw(1);
    withdrawal.client = Some(client);
    add_test_surface_route(&mut withdrawal, surface, client);
    withdrawal
        .presentation_intents
        .push(sophia_protocol::SurfacePresentationIntent {
            surface,
            kind: sophia_protocol::SurfacePresentationIntentKind::Withdraw,
            role: sophia_protocol::SurfacePresentationRole::PolicyManaged,
            surface_kind: sophia_protocol::LayoutNodeKind::Toplevel,
            placement_preference: sophia_protocol::SurfacePlacementPreference::Default,
            presentation_owner: None,
            stack_rank: 0,
            geometry,
            constraints: SurfaceConstraints {
                min_size: None,
                max_size: None,
            },
            generation: 1,
        });
    layout.observe_authority_batch(&withdrawal);
    assert!(layout.escaped_pre_admission.is_empty());

    // A late acknowledgement finds no admission state to accept it, and there
    // is no record left for it to authorize either.
    assert!(!layout.acknowledge_admission_control(TransactionId::from_raw(13), surface));
    assert!(layout.skippable_escaped_presents().is_empty());
}
