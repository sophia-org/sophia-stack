use super::*;

fn runtime() -> LiveProductionVisualRuntime {
    LiveProductionVisualRuntime::new(
        &[
            HeadlessOutput {
                id: OutputId::from_raw(1),
                size: Size {
                    width: 2560,
                    height: 1440,
                },
                scale: 1,
            },
            HeadlessOutput {
                id: OutputId::from_raw(2),
                size: Size {
                    width: 1920,
                    height: 1080,
                },
                scale: 1,
            },
        ],
        None,
    )
    .unwrap()
}

fn layer(surface: SurfaceId) -> LayerSnapshot {
    LayerSnapshot {
        input_region: None,
        surface,
        authority_local_id: None,
        output: None,
        namespace: None,
        stack_rank: 0,
        geometry: Rect {
            x: 0,
            y: 0,
            width: 2560,
            height: 32,
        },
        source_size: Size {
            width: 2560,
            height: 32,
        },
        source: BufferSource::CpuBuffer { handle: 1 },
        damage: Region::empty(),
        opacity: 1.0,
        crop: None,
        transform: Transform::IDENTITY,
        generation: 1,
        resize_sync: ResizeSyncCapability::ImplicitOnly,
        translation: None,
    }
}

#[test]
fn panel_route_reaches_display_list_and_is_withdrawn_with_session_visibility() {
    let panel = SurfaceId::new(1, 1);
    let unplaced = SurfaceId::new(2, 1);
    let mut runtime = runtime();
    let layers = [layer(panel), layer(unplaced)];
    let committed = layers
        .iter()
        .map(|l| {
            CommittedSurfaceState::with_source(
                l.surface,
                1,
                l.geometry,
                l.source,
                l.source_size,
                Region::empty(),
            )
        })
        .collect::<Vec<_>>();
    assert!(runtime.apply_presentation_layout(&layers, &[panel]));
    for output in [OutputId::from_raw(1), OutputId::from_raw(2)] {
        let list = runtime
            .display_list_for_output(
                output,
                runtime.outputs.logical_viewport(output).unwrap(),
                &committed,
                &[panel, unplaced],
            )
            .unwrap();
        assert!(
            list.commands
                .contains(&CompositorDisplayCommand::Surface { surface: panel })
        );
        assert!(
            !list
                .commands
                .contains(&CompositorDisplayCommand::Surface { surface: unplaced })
        );
    }
    assert!(
        runtime.apply_presentation_layout(&layers, &[]),
        "routing withdrawal invalidates the projection even with the same layer order"
    );
    assert!(runtime.geometry_routed_surfaces.is_empty());
    assert!(!runtime.apply_presentation_layout(&layers, &[]));
    runtime.apply_presentation_layout(&[], &[panel]);
    assert!(
        runtime.geometry_routed_surfaces.is_empty(),
        "stale route cannot resurrect an unmapped panel"
    );
}

#[test]
fn retained_repaints_wait_for_the_exact_first_present_to_retire() {
    let mut runtime = runtime();
    let surface = SurfaceId::new(3, 1);
    let transaction = TransactionId::from_raw(23649);
    let output = OutputId::from_raw(1);
    let geometry = Rect {
        x: 1919,
        y: 41,
        width: 1258,
        height: 1390,
    };
    let size = Size {
        width: geometry.width,
        height: geometry.height,
    };
    let candidate = SurfaceTransaction {
        input_region: None,
        transaction,
        surface,
        authority: AuthorityKind::SophiaX,
        namespace: None,
        target_geometry: geometry,
        presentation_extent: size,
        content: SurfaceContentSet::singleton(BufferSource::DmaBuf { handle: 28 }, size),
        damage: Region::empty(),
        readiness: SurfaceTransactionReadiness::Ready,
        timeout_msec: 250,
        previous_committed_generation: 0,
    };
    let prepared = runtime.production.prepare_present_transaction(&candidate);
    assert!(prepared.is_ready());
    let present = LiveProductionSubmittedPresent::new(
        BTreeMap::from([(output, LiveProductionNativeFrameId::from_raw(3045))]),
        output,
        candidate.key(),
        transaction,
        surface,
        prepared,
        LiveRetainedRendererImageLayer {
            image_id: LiveRendererImageId::from_raw(28),
            size,
            format: LIVE_RENDERER_SCANOUT_FORMAT_XRGB8888,
            placement: LiveCompositionPlacement {
                target: geometry,
                clip: None,
                transform: Transform::IDENTITY,
                alpha: 1.0,
                sampling: HeadSamplingClass::Exact,
            },
        },
    )
    .unwrap();
    assert!(!runtime.native_publication_blocked());
    runtime.present_scheduler.mark_rendering(present);
    assert!(
        runtime.native_publication_blocked(),
        "first-frame render owns its retirement proof before KMS submission"
    );
    runtime
        .present_scheduler
        .mark_output_submitted(output)
        .unwrap();
    assert!(runtime.native_publication_blocked());
    runtime
        .present_scheduler
        .mark_output_retired(LiveProductionPageFlipRetirement {
            output,
            ust: 1000,
            msc: 1,
        })
        .unwrap();
    assert!(
        runtime.native_publication_blocked(),
        "retirement must be settled before repainting"
    );
    runtime.present_scheduler.take_submitted().unwrap();
    assert!(!runtime.native_publication_blocked());
}

/// The other two clauses of the same guard, which the Present path above does
/// not reach. Both are states in which the native path already owns the head a
/// composed frame would be queued into.
#[test]
fn a_suspended_native_path_blocks_publication() {
    let mut runtime = runtime();
    assert!(!runtime.native_publication_blocked());

    runtime.native_suspended = true;
    assert!(
        runtime.native_publication_blocked(),
        "a suspended native path has no head to accept a composed frame"
    );

    runtime.native_suspended = false;
    assert!(!runtime.native_publication_blocked());
}

#[test]
fn a_bound_software_frame_blocks_publication() {
    let mut runtime = runtime();
    let output = OutputId::from_raw(1);
    let frame = LiveProductionNativeFrameId::from_raw(4096);
    assert!(!runtime.native_publication_blocked());

    runtime.software_present_frames_bound.insert(
        frame,
        software_present::LiveProductionSoftwarePresentBinding {
            frames: BTreeMap::from([(output, frame)]),
            clock_output: output,
            output_cohort: TransactionPresentationCohort::new(
                TransactionId::from_raw(23650),
                [output],
            )
            .unwrap(),
            retirements: BTreeMap::new(),
            submissions: Vec::new(),
            phase: LiveProductionSoftwarePresentFramePhase::Pending,
        },
    );
    assert!(
        runtime.native_publication_blocked(),
        "a bound software frame already owns the head it will present into"
    );

    runtime.software_present_frames_bound.clear();
    assert!(!runtime.native_publication_blocked());
}

#[test]
fn policy_status_does_not_create_a_builtin_bar_or_capture_panel_input() {
    let mut runtime = runtime();
    let publication = PolicyIndicatorPublication {
        generation: 1,
        connection_epoch: Some(1),
        indicators: vec![PolicyProjectionIndicator {
            output: OutputId::from_raw(1),
            slot: 0,
            indicator: 1,
            action: Some(WmActionId::from_raw(1)),
            state_bits: 1,
            label: "dev".into(),
        }],
        output_statuses: Vec::new(),
        tab_groups: Vec::new(),
    };
    runtime.set_indicator_publication(Some(publication.clone()));
    for output in [OutputId::from_raw(1), OutputId::from_raw(2)] {
        let list = runtime
            .display_list_for_output(
                output,
                runtime.outputs.logical_viewport(output).unwrap(),
                &[],
                &[],
            )
            .unwrap();
        assert!(
            !list
                .commands
                .iter()
                .any(|command| matches!(command, CompositorDisplayCommand::IndicatorStrip(_)))
        );
    }
    runtime.publish_committed_input_layers();
    assert!(
        runtime
            .input_projections
            .iter()
            .all(|projection| projection.chrome_targets.is_empty()
                && projection.chrome_occlusion.is_none())
    );
    assert_eq!(
        runtime.indicator_publication,
        Some(publication),
        "committed policy descriptors remain available to shell projections"
    );
}

fn window_layer(
    surface: SurfaceId,
    rank: u32,
    geometry: Rect,
    output: Option<OutputId>,
) -> LayerSnapshot {
    LayerSnapshot {
        stack_rank: rank,
        geometry,
        output,
        source_size: Size {
            width: geometry.width,
            height: geometry.height,
        },
        ..layer(surface)
    }
}

/// A repaint's raise orders the stack and never becomes the chrome focus.
#[test]
fn a_repaint_raise_never_becomes_the_chrome_focus() {
    let focused = SurfaceId::new(1, 1);
    let other = SurfaceId::new(2, 1);
    let popup = SurfaceId::new(3, 1);
    let mut runtime = runtime();
    let layers = [
        // Managed windows are placed on an output; the popup is client-positioned
        // and routes by geometry instead, exactly as the session distinguishes them.
        window_layer(
            focused,
            0,
            Rect {
                x: 100,
                y: 100,
                width: 800,
                height: 600,
            },
            Some(OutputId::from_raw(1)),
        ),
        window_layer(
            other,
            1,
            Rect {
                x: 300,
                y: 150,
                width: 640,
                height: 480,
            },
            Some(OutputId::from_raw(1)),
        ),
        window_layer(
            popup,
            2,
            Rect {
                x: 200,
                y: 200,
                width: 120,
                height: 80,
            },
            None,
        ),
    ];
    let committed = layers
        .iter()
        .map(|l| {
            CommittedSurfaceState::with_source(
                l.surface,
                1,
                l.geometry,
                l.source,
                l.source_size,
                Region::empty(),
            )
        })
        .collect::<Vec<_>>();
    runtime.set_surface_chrome_style(SurfaceChromeStyle {
        frame: SurfaceFrameStyle {
            width: 4,
            ..SurfaceFrameStyle::default()
        },
        ..SurfaceChromeStyle::default()
    });
    // The popup is client-positioned, so the session leaves it out of the chrome set.
    assert!(runtime.apply_presentation_layout(&layers, &[popup]));
    assert!(runtime.set_chrome_surfaces(&[focused, other]));

    let repaint = |runtime: &mut LiveProductionVisualRuntime, raised, focus| {
        let list = runtime.prepare_repaint(&committed, raised, focus).unwrap();
        let resolved = runtime.focused_surface();
        let summary = compositor_chrome_summary(&list, resolved);
        assert!(
            summary.frames > 0,
            "the windows must be framed for this regression to mean anything"
        );
        (resolved, summary.focused_frames)
    };

    assert_eq!(
        repaint(&mut runtime, None, Some(focused)),
        (Some(focused), 1)
    );
    // A frameless popup raised over the focused window.
    assert_eq!(
        repaint(&mut runtime, Some(popup), Some(focused)),
        (Some(focused), 1),
    );
    // A different FRAMED window raised: still not a focus change.
    assert_eq!(
        repaint(&mut runtime, Some(other), Some(focused)),
        (Some(focused), 1),
    );
    // No raise at all.
    assert_eq!(
        repaint(&mut runtime, None, Some(focused)),
        (Some(focused), 1)
    );

    // Separation, not suppression: the raise must still order the stack while
    // the focus stays put. Without it the top of the stack is the popup.
    let stack = |runtime: &mut LiveProductionVisualRuntime, raised| {
        let list = runtime
            .prepare_repaint(&committed, raised, Some(focused))
            .unwrap();
        list.commands
            .iter()
            .filter_map(|command| match command {
                CompositorDisplayCommand::Surface { surface } => Some(*surface),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(stack(&mut runtime, None).last(), Some(&popup));
    assert_eq!(
        stack(&mut runtime, Some(other)).last(),
        Some(&other),
        "a raised framed window must still reach the top of the stack"
    );
    assert_eq!(
        runtime.focused_surface(),
        Some(focused),
        "and raising it must not have moved the focus"
    );
    // An explicit focus that owns no chrome leaves the held focus standing.
    assert_eq!(
        repaint(&mut runtime, None, Some(popup)),
        (Some(focused), 1),
        "an explicit focus is still normalized against the chrome set"
    );
    // An explicit absence of focus clears it, even while something is raised.
    assert_eq!(repaint(&mut runtime, Some(focused), None), (None, 0));
}
