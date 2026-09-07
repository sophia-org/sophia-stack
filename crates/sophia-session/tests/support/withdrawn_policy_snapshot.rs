use super::*;

#[test]
fn retained_pixels_cannot_return_a_withdrawn_dialog_to_the_wm_snapshot() {
    let surface = SurfaceId::new(190, 3);
    let client = sophia_x_authority::XServerFrontendClientId::from_raw(1);
    let geometry = Rect {
        x: 0,
        y: 0,
        width: 320,
        height: 200,
    };
    let intent = sophia_protocol::SurfacePresentationIntent {
        surface,
        kind: sophia_protocol::SurfacePresentationIntentKind::Request,
        role: sophia_protocol::SurfacePresentationRole::PolicyManaged,
        surface_kind: sophia_protocol::LayoutNodeKind::Dialog,
        placement_preference: sophia_protocol::SurfacePlacementPreference::Default,
        presentation_owner: None,
        stack_rank: 1,
        geometry,
        constraints: SurfaceConstraints {
            min_size: None,
            max_size: None,
        },
        generation: 1,
    };
    let snapshots = |layout: &PersistentLiveLayout| {
        public_policy_surface_snapshots(
            layout,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            sophia_engine::SurfaceChromeStyle::default(),
        )
        .unwrap()
    };
    let mut layout = PersistentLiveLayout::default();
    let mut request = crate::live_session::wm_update_coordinator_batch(TransactionId::from_raw(1));
    request.client = Some(client);
    add_test_surface_route(&mut request, surface, client);
    request.presentation_intents.push(intent);
    layout.observe_authority_batch(&request);
    assert_eq!(
        snapshots(&layout)[0].surface,
        surface,
        "pending admission is visible to policy"
    );

    let pixels = test_layer(surface, geometry);
    layout.layers.insert(surface, pixels.clone());
    let mut withdrawal =
        crate::live_session::wm_update_coordinator_batch(TransactionId::from_raw(2));
    withdrawal
        .presentation_intents
        .push(sophia_protocol::SurfacePresentationIntent {
            kind: sophia_protocol::SurfacePresentationIntentKind::Withdraw,
            ..intent
        });
    layout.observe_authority_batch(&withdrawal);
    assert_eq!(layout.layers.get(&surface), Some(&pixels));
    assert!(
        snapshots(&layout).is_empty(),
        "cached pixels are not a live policy surface"
    );

    // A client may redraw its retained window while hidden. Fresh passive
    // facts must not grant policy ownership without a new map request.
    let mut hidden_update =
        crate::live_session::wm_update_coordinator_batch(TransactionId::from_raw(3));
    hidden_update.surface_presentations.push(
        sophia_x_authority::XAuthoritySurfacePresentationObservation {
            surface,
            role: intent.role,
            kind: intent.surface_kind,
            placement_preference: intent.placement_preference,
            owner: None,
            stack_rank: 1,
            mapped: false,
            geometry,
            constraints: intent.constraints,
            generation: 2,
        },
    );
    layout.observe_authority_batch(&hidden_update);
    assert!(snapshots(&layout).is_empty());

    request.transaction = TransactionId::from_raw(4);
    layout.observe_authority_batch(&request);
    assert_eq!(
        snapshots(&layout)[0].surface,
        surface,
        "a remap can request policy again"
    );
    assert_eq!(layout.layers.get(&surface), Some(&pixels));
}
