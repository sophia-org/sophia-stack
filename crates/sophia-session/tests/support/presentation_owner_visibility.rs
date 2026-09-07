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
