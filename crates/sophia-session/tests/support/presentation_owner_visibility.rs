use super::*;
use sophia_protocol::{LayoutNodeKind, SurfacePlacementPreference, SurfacePresentationRole};

fn observe(
    layout: &mut PersistentLiveLayout,
    surface: SurfaceId,
    role: SurfacePresentationRole,
    owner: Option<SurfaceId>,
    mapped: bool,
) {
    let mut batch = crate::live_session::wm_update_coordinator_batch(TransactionId::from_raw(70));
    batch.surface_presentations.push(
        sophia_x_authority::XAuthoritySurfacePresentationObservation {
            surface,
            role,
            kind: LayoutNodeKind::Toplevel,
            placement_preference: SurfacePlacementPreference::Default,
            owner,
            stack_rank: 0,
            mapped,
            geometry: Rect {
                x: 0,
                y: 0,
                width: 240,
                height: 112,
            },
            constraints: SurfaceConstraints {
                min_size: None,
                max_size: None,
            },
            generation: 1,
        },
    );
    layout.observe_authority_batch(&batch);
}

#[test]
fn panel_popups_inherit_mapping_without_requesting_wm_placement() {
    let panel = SurfaceId::new(70, 1);
    let popup = SurfaceId::new(71, 1);
    let nested = SurfaceId::new(72, 1);
    let mut layout = PersistentLiveLayout::default();
    for (surface, owner) in [(panel, None), (popup, Some(panel)), (nested, Some(popup))] {
        observe(
            &mut layout,
            surface,
            SurfacePresentationRole::ClientPositioned,
            owner,
            true,
        );
    }
    let visible = |layout: &PersistentLiveLayout, surface| {
        layout
            .client_positioned_visible::<()>(surface, |_| panic!("panel has no WM placement"))
            .unwrap()
    };
    assert!(visible(&layout, nested));
    observe(
        &mut layout,
        panel,
        SurfacePresentationRole::ClientPositioned,
        None,
        false,
    );
    assert!(!visible(&layout, nested));
    observe(
        &mut layout,
        panel,
        SurfacePresentationRole::ClientPositioned,
        None,
        true,
    );
    assert!(visible(&layout, popup));

    let mut removal = crate::live_session::wm_update_coordinator_batch(TransactionId::from_raw(71));
    removal.removed_surfaces.push(panel);
    layout.observe_authority_batch(&removal);
    observe(
        &mut layout,
        SurfaceId::new(70, 2),
        SurfacePresentationRole::ClientPositioned,
        None,
        true,
    );
    assert!(!visible(&layout, popup));
}

#[test]
fn nested_popup_visibility_reaches_managed_owner_and_propagates_errors() {
    let managed = SurfaceId::new(80, 1);
    let popup = SurfaceId::new(81, 1);
    let nested = SurfaceId::new(82, 1);
    let mut layout = PersistentLiveLayout::default();
    observe(
        &mut layout,
        managed,
        SurfacePresentationRole::PolicyManaged,
        None,
        true,
    );
    observe(
        &mut layout,
        popup,
        SurfacePresentationRole::ClientPositioned,
        Some(managed),
        true,
    );
    observe(
        &mut layout,
        nested,
        SurfacePresentationRole::ClientPositioned,
        Some(popup),
        true,
    );
    for visible in [false, true] {
        assert_eq!(
            layout.client_positioned_visible::<()>(nested, |owner| {
                assert_eq!(owner, managed);
                Ok(visible)
            }),
            Ok(visible)
        );
    }
    assert_eq!(
        layout.client_positioned_visible(nested, |_| Err("policy unavailable")),
        Err("policy unavailable")
    );
}

#[test]
fn cyclic_popup_ownership_is_not_visible() {
    let first = SurfaceId::new(90, 1);
    let second = SurfaceId::new(91, 1);
    let mut layout = PersistentLiveLayout::default();
    for (surface, owner) in [(first, second), (second, first)] {
        observe(
            &mut layout,
            surface,
            SurfacePresentationRole::ClientPositioned,
            Some(owner),
            true,
        );
    }
    assert!(
        !layout
            .client_positioned_visible::<()>(first, |_| panic!("cycle has no managed ancestor"))
            .unwrap()
    );
}

/// An unmapped managed window leaves the scene before policy agrees.
///
/// Policy projects a layout that is not refreshed in the step carrying an
/// unmap, so a window torn down this cycle is still projected as visible. A
/// captured session showed the consequence: six Thunar menu windows reported
/// `mapped=0` by `GetWindowAttributes` while still present in the scene. Being
/// listed as a child of the root is not being mapped.
#[test]
fn an_unmapped_managed_surface_leaves_the_scene_though_policy_still_projects_it() {
    let managed = SurfaceId::new(100, 1);
    let mut layout = PersistentLiveLayout::default();
    observe(
        &mut layout,
        managed,
        SurfacePresentationRole::PolicyManaged,
        None,
        true,
    );
    // Policy is asked while the window is mapped, and its answer is honoured.
    assert_eq!(
        layout.managed_scene_visible::<()>(managed, |surface| {
            assert_eq!(surface, managed);
            Ok(true)
        }),
        Ok(true)
    );

    observe(
        &mut layout,
        managed,
        SurfacePresentationRole::PolicyManaged,
        None,
        false,
    );
    // Now policy is stale. It must not be consulted at all: mapping already
    // settles the question, and a projection that says otherwise is a view of
    // a layout one step behind.
    assert_eq!(
        layout.managed_scene_visible::<()>(managed, |_| {
            panic!("an unmapped surface must not consult a stale projection")
        }),
        Ok(false)
    );
}

/// Unmapping a window takes its popups with it, whatever policy still says.
#[test]
fn unmapping_a_managed_owner_hides_the_popups_it_owns() {
    let managed = SurfaceId::new(110, 1);
    let popup = SurfaceId::new(111, 1);
    let nested = SurfaceId::new(112, 1);
    let mut layout = PersistentLiveLayout::default();
    observe(
        &mut layout,
        managed,
        SurfacePresentationRole::PolicyManaged,
        None,
        true,
    );
    observe(
        &mut layout,
        popup,
        SurfacePresentationRole::ClientPositioned,
        Some(managed),
        true,
    );
    observe(
        &mut layout,
        nested,
        SurfacePresentationRole::ClientPositioned,
        Some(popup),
        true,
    );
    // Policy answers "visible" throughout, which is exactly the stale
    // projection the repair has to survive.
    let visible = |layout: &PersistentLiveLayout, surface| {
        layout
            .client_positioned_visible::<()>(surface, |_| Ok(true))
            .unwrap()
    };
    assert!(visible(&layout, nested));

    observe(
        &mut layout,
        managed,
        SurfacePresentationRole::PolicyManaged,
        None,
        false,
    );
    assert!(
        !visible(&layout, popup),
        "a popup whose window is unmapped is not visible"
    );
    assert!(
        !visible(&layout, nested),
        "and neither is a popup of that popup"
    );

    // Remapping the window restores both, so this hides rather than forgets.
    observe(
        &mut layout,
        managed,
        SurfacePresentationRole::PolicyManaged,
        None,
        true,
    );
    assert!(visible(&layout, popup));
    assert!(visible(&layout, nested));
}

/// A popup that unmaps and remaps returns, and a destroyed one does not come
/// back when its identifier is reused.
#[test]
fn a_popup_returns_on_remap_but_not_through_a_reused_identifier() {
    let managed = SurfaceId::new(120, 1);
    let popup = SurfaceId::new(121, 1);
    let mut layout = PersistentLiveLayout::default();
    observe(
        &mut layout,
        managed,
        SurfacePresentationRole::PolicyManaged,
        None,
        true,
    );
    observe(
        &mut layout,
        popup,
        SurfacePresentationRole::ClientPositioned,
        Some(managed),
        true,
    );
    let visible = |layout: &PersistentLiveLayout, surface| {
        layout
            .client_positioned_visible::<()>(surface, |_| Ok(true))
            .unwrap()
    };
    assert!(visible(&layout, popup));

    // A menu closing is an unmap, not a destroy.
    observe(
        &mut layout,
        popup,
        SurfacePresentationRole::ClientPositioned,
        Some(managed),
        false,
    );
    assert!(!visible(&layout, popup));
    observe(
        &mut layout,
        popup,
        SurfacePresentationRole::ClientPositioned,
        Some(managed),
        true,
    );
    assert!(visible(&layout, popup), "reopening the menu shows it again");

    // Destroying it drops the surface entirely. The generation makes a reused
    // X identifier a different surface, so the destroyed popup cannot be
    // revived by whatever the client creates next at the same id.
    let mut removal = crate::live_session::wm_update_coordinator_batch(TransactionId::from_raw(72));
    removal.removed_surfaces.push(popup);
    layout.observe_authority_batch(&removal);
    assert!(!visible(&layout, popup));

    let reused = SurfaceId::new(121, 2);
    assert!(
        !visible(&layout, reused),
        "a reused identifier is not the destroyed popup and has no standing of its own"
    );
}

fn admission_geometry() -> Rect {
    Rect {
        x: 0,
        y: 0,
        width: 240,
        height: 112,
    }
}

/// Drives a managed surface to the point where the frontend has been asked to
/// admit it and has not yet answered.
fn awaiting_admission(surface: SurfaceId, transaction: TransactionId) -> PersistentLiveLayout {
    let mut layout = PersistentLiveLayout::default();
    // The window exists and the authority reports it not yet mapped, which is
    // what a policy-managed window looks like before placement.
    observe(
        &mut layout,
        surface,
        SurfacePresentationRole::PolicyManaged,
        None,
        false,
    );
    assert!(
        layout
            .admissions
            .observe_intent(sophia_protocol::SurfacePresentationIntent {
                surface,
                kind: sophia_protocol::SurfacePresentationIntentKind::Request,
                role: SurfacePresentationRole::PolicyManaged,
                surface_kind: LayoutNodeKind::Toplevel,
                placement_preference: SurfacePlacementPreference::Default,
                presentation_owner: None,
                stack_rank: 0,
                geometry: admission_geometry(),
                constraints: SurfaceConstraints {
                    min_size: None,
                    max_size: None,
                },
                generation: 1,
            })
    );
    assert!(
        layout
            .admissions
            .begin_control(surface, transaction, admission_geometry())
    );
    layout
}

/// An acknowledged admission makes the window visible immediately, with no
/// further traffic of any kind.
///
/// The authority marks a policy-managed window viewable when it admits it, and
/// that transition rides in no presentation of its own, because the engine
/// asked for the map rather than a client. A live session showed the cost: two
/// Kitty windows viewable on the X server, drawing, and an entirely empty
/// scene. Nothing here observes another batch after the acknowledgement, which
/// is the point -- a client that admits and then goes idle must still appear.
#[test]
fn an_acknowledged_admission_shows_the_window_without_any_later_traffic() {
    let surface = SurfaceId::new(160, 1);
    let transaction = TransactionId::from_raw(90);
    let mut layout = awaiting_admission(surface, transaction);

    assert_eq!(
        layout.managed_scene_visible::<()>(surface, |_| Ok(true)),
        Ok(false),
        "before the frontend answers, the window is not in the scene"
    );

    assert!(layout.acknowledge_admission_control(transaction, surface));

    assert_eq!(
        layout.managed_scene_visible::<()>(surface, |_| Ok(true)),
        Ok(true),
        "the acknowledgement alone makes it eligible"
    );
}

/// Only a correlated acknowledgement counts, and it counts once.
#[test]
fn an_uncorrelated_admission_acknowledgement_does_not_map_anything() {
    let surface = SurfaceId::new(161, 1);
    let transaction = TransactionId::from_raw(91);
    let visible = |layout: &PersistentLiveLayout, surface| {
        layout.managed_scene_visible::<()>(surface, |_| Ok(true)) == Ok(true)
    };

    // A different transaction is not this admission.
    let mut layout = awaiting_admission(surface, transaction);
    assert!(!layout.acknowledge_admission_control(TransactionId::from_raw(92), surface));
    assert!(!visible(&layout, surface));
    // ...and the real one still works afterwards.
    assert!(layout.acknowledge_admission_control(transaction, surface));
    assert!(visible(&layout, surface));

    // A repeat is refused: the surface has left ControlPending.
    assert!(!layout.acknowledge_admission_control(transaction, surface));

    // A surface the authority never asked to admit cannot be mapped by an
    // acknowledgement naming it.
    let mut fresh = awaiting_admission(surface, transaction);
    let stranger = SurfaceId::new(162, 1);
    assert!(!fresh.acknowledge_admission_control(transaction, stranger));
    assert!(!visible(&fresh, stranger));

    // A reused identifier is a different surface, so an acknowledgement for the
    // old generation does not map the new one.
    let reused = SurfaceId::new(161, 2);
    assert!(!fresh.acknowledge_admission_control(transaction, reused));
    assert!(!visible(&fresh, reused));
}

/// An acknowledgement that arrives after the window is gone maps nothing.
#[test]
fn a_late_admission_acknowledgement_cannot_revive_a_removed_window() {
    let surface = SurfaceId::new(163, 1);
    let transaction = TransactionId::from_raw(93);
    let mut layout = awaiting_admission(surface, transaction);

    // The client destroys the window while the admission is in flight.
    let mut removal = crate::live_session::wm_update_coordinator_batch(TransactionId::from_raw(94));
    removal.removed_surfaces.push(surface);
    layout.observe_authority_batch(&removal);

    assert!(
        !layout.acknowledge_admission_control(transaction, surface),
        "a destroyed window has no admission left to acknowledge"
    );
    assert_eq!(
        layout.managed_scene_visible::<()>(surface, |_| Ok(true)),
        Ok(false)
    );

    // And an unmap published after a good acknowledgement still wins.
    let mut unmapped = awaiting_admission(SurfaceId::new(164, 1), transaction);
    let live = SurfaceId::new(164, 1);
    assert!(unmapped.acknowledge_admission_control(transaction, live));
    observe(
        &mut unmapped,
        live,
        SurfacePresentationRole::PolicyManaged,
        None,
        false,
    );
    assert_eq!(
        unmapped.managed_scene_visible::<()>(live, |_| Ok(true)),
        Ok(false),
        "an unmap after admission removes it again"
    );
}

/// The admitted window's pixels actually reach a composed frame, and only
/// after the acknowledgement.
///
/// Every other test here reasons about eligibility. This one carries it
/// through to bytes, because eligibility is not what the user sees: the live
/// regression had two viewable Kitty windows drawing into retained buffers and
/// a scene that composed nothing, and an offline suite that asserted only
/// eligibility passed throughout. The scene is composed from exactly the
/// surfaces the visibility rule selects, which is what
/// `authority_production.rs` does when it builds `presentation_layout`.
#[test]
fn an_admitted_window_composes_its_pixels_and_an_unmapped_one_does_not() {
    let surface = SurfaceId::new(165, 1);
    let transaction = TransactionId::from_raw(95);
    let geometry = Rect {
        x: 0,
        y: 0,
        width: 2,
        height: 1,
    };
    // A distinctive marker so a composed frame cannot be confused with a
    // cleared one, in either direction.
    let marker = [0x21, 0x43, 0x65, 0xff, 0x21, 0x43, 0x65, 0xff];
    let empty = [0u8; 8];

    let mut scene = LiveProductionCpuScene::new(Size {
        width: 2,
        height: 1,
    });
    let committed = CommittedSurfaceState {
        surface,
        committed_generation: 1,
        geometry,
        content: sophia_protocol::SurfaceContentSet::singleton(
            BufferSource::CpuBuffer { handle: 11 },
            Size {
                width: geometry.width,
                height: geometry.height,
            },
        ),
        damage: Region::single(geometry),
    };
    scene
        .apply_updates([sophia_backend_live::LiveCpuBufferUpdate::Replace(
            sophia_backend_live::LiveCpuBufferSource {
                handle: 11,
                size: Size {
                    width: 2,
                    height: 1,
                },
                stride: 8,
                format: X_AUTHORITY_CPU_BUFFER_FORMAT_XRGB8888,
                generation: 1,
                bytes: std::sync::Arc::new(marker.to_vec()),
            },
        )])
        .unwrap();
    scene.reconcile_buffer_residency(&[11]);

    let mut layout = awaiting_admission(surface, transaction);

    // The scene is composed from exactly what the visibility rule admits,
    // the way the production cycle builds its presentation layout.
    let selected = |layout: &PersistentLiveLayout| -> Vec<CommittedSurfaceState> {
        [committed.clone()]
            .into_iter()
            .filter(|state| {
                layout
                    .managed_scene_visible::<()>(state.surface, |_| Ok(true))
                    .unwrap()
            })
            .collect()
    };

    // The client has retained pixels and the frontend has not answered yet.
    // Nothing composes, and the marker is nowhere in the frame.
    let before = selected(&layout);
    assert!(before.is_empty());
    assert_eq!(
        scene.compose(&before, None, None).unwrap().frame.bytes,
        empty.to_vec().into(),
        "a window awaiting admission contributes no pixels"
    );

    // The frontend acknowledges. No further client traffic of any kind.
    assert!(layout.acknowledge_admission_control(transaction, surface));

    let after = selected(&layout);
    assert_eq!(after.len(), 1);
    assert_eq!(
        scene.compose(&after, None, None).unwrap().frame.bytes,
        marker.to_vec().into(),
        "the acknowledged window's retained pixels reach the frame"
    );

    // Unmapping takes them out again.
    observe(
        &mut layout,
        surface,
        SurfacePresentationRole::PolicyManaged,
        None,
        false,
    );
    let unmapped = selected(&layout);
    assert!(unmapped.is_empty());
    assert_eq!(
        scene.compose(&unmapped, None, None).unwrap().frame.bytes,
        empty.to_vec().into(),
        "an unmapped window stops contributing pixels"
    );
}
