use super::*;
use sophia_protocol::{
    LayoutNodeCapabilities, OutputId, PolicyOutputProjection, PolicyPresentationState,
    PolicyProjectionProposal, PolicySurfaceKind, PolicySurfacePlacement, PolicySurfaceSnapshot,
    PolicyTransform, SurfaceConstraints,
};

// Exercise the owner settlement boundary without starting a policy process.
// Both displays keep their per-output focus; only the active one owns the seat.
#[test]
fn empty_active_output_releases_previous_keyboard_focus_only_after_commit() {
    let left = sophia_engine::HeadlessOutput::deterministic();
    let right = sophia_engine::HeadlessOutput {
        id: OutputId::from_raw(2),
        ..left
    };
    let surface = SurfaceId::new(91, 1);
    let geometry = Rect {
        x: 8,
        y: 8,
        width: 600,
        height: 600,
    };
    for (active, accepted) in [(right.id, true), (right.id, false), (left.id, true)] {
        let mut fixture = ReloadFixture::new();
        let public = fixture.wm.public.as_mut().unwrap();
        public.transport_unavailable = true;
        let mut scene = LivePublicPolicyState::initial_scene(&[left, right], left.id, vec![]);
        scene.surfaces.push(PolicySurfaceSnapshot {
            surface,
            generation: 1,
            current_output: Some(left.id),
            kind: PolicySurfaceKind::Toplevel,
            capabilities: LayoutNodeCapabilities::STANDARD_TOPLEVEL,
            constraints: SurfaceConstraints {
                min_size: None,
                max_size: None,
            },
            exact_size: None,
            requested_state: PolicyPresentationState::default(),
            current_state: PolicyPresentationState::default(),
            transient_owner: None,
            geometry,
        });
        public.reducer = sophia_engine::PolicyProjectionReducer::new(scene).unwrap();
        public.reducer.connect(1).unwrap();
        let request = public
            .reducer
            .issue_request(vec![left.id, right.id])
            .unwrap();
        let transaction = TransactionId::from_raw(91);
        let proposal = PolicyProjectionProposal {
            transaction,
            connection_epoch: request.connection_epoch,
            request_id: request.request_id,
            base_generation: request.scene_generation,
            active_output: active,
            outputs: vec![
                PolicyOutputProjection {
                    output: left.id,
                    placements: vec![PolicySurfacePlacement {
                        surface,
                        surface_generation: 1,
                        geometry,
                        requested_size: None,
                        crop: None,
                        transform: PolicyTransform::Identity,
                        presentation: PolicyPresentationState::default(),
                    }],
                    focus: Some(surface),
                },
                PolicyOutputProjection {
                    output: right.id,
                    placements: vec![],
                    focus: None,
                },
            ],
            translation_groups: vec![],
            tab_groups: vec![],
            indicators: vec![],
            output_statuses: vec![],
        };
        public.staged = Some(public.reducer.stage_proposal(&proposal).unwrap());
        let settlement = LivePolicySettlementIdentity {
            connection_epoch: request.connection_epoch,
            request_id: request.request_id,
            scene_generation: request.scene_generation,
            transaction,
            expect_session_operation: false,
            session_operation: false,
        };
        public.prepared = Some(settlement);
        public.in_flight_request = Some(request);
        assert_eq!(fixture.wm.reference_output(), Some(left.id));
        let result = fixture
            .wm
            .apply_commit_result(
                LiveWmCommitResult {
                    update: WmTransactionUpdate {
                        commit: TransactionCommit {
                            transaction,
                            outcome: if accepted {
                                TransactionOutcome::Committed
                            } else {
                                TransactionOutcome::TimedOut
                            },
                            applied_surfaces: vec![surface],
                        },
                    },
                    source: None,
                    policy_settlement: Some(settlement),
                },
                Some(surface),
                left.id,
            )
            .unwrap();
        assert_eq!(
            result.clear_focus,
            (accepted && active == right.id).then_some((transaction, surface))
        );
        assert_eq!(
            fixture.wm.reference_output(),
            Some(if accepted { active } else { left.id })
        );
        if accepted {
            let public = fixture.wm.public.as_ref().unwrap();
            assert_eq!(
                public.reducer.committed()[0].focus,
                Some(surface),
                "the left workspace remembers its window even when the seat leaves"
            );
        }
    }
}
