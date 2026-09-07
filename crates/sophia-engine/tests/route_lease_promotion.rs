use sophia_engine::{
    ApplicationRouteLeaseCandidate, ApplicationRouteLeaseError, ApplicationRouteLeaseOrigin,
    ApplicationRouteLeasePhase, ApplicationRouteLeaseState, ApplicationRouteScope,
};
use sophia_protocol::{
    ClientAdmissionId, DeviceId, NamespaceId, NamespaceProfile, OutputId, SeatId, SurfaceId,
};

fn click() -> ApplicationRouteLeaseCandidate {
    ApplicationRouteLeaseCandidate {
        seat: SeatId::from_raw(1),
        origin: ApplicationRouteLeaseOrigin::PointerBoundary,
        target_surface: SurfaceId::new(40, 3),
        admission: ClientAdmissionId::from_raw(7),
        scope: ApplicationRouteScope {
            profile: NamespaceProfile::Confined,
            authority: NamespaceId::from_raw(4),
        },
        authority_session_epoch: 9,
        output: OutputId::from_raw(2),
        presentation_epoch: 11,
        initiating_device: Some(DeviceId::from_raw(5)),
        initiating_button: Some(0x110),
    }
}

#[test]
fn own_click_can_become_explicit_before_or_after_its_confirmation_arrives() {
    for confirmed in [false, true] {
        let mut state = ApplicationRouteLeaseState::default();
        let first = state.begin_provisional(click()).unwrap();
        if confirmed {
            state
                .confirm(
                    first.identity,
                    first.target_surface,
                    first.admission,
                    first.authority_session_epoch,
                )
                .unwrap();
        }
        let candidate = ApplicationRouteLeaseCandidate {
            origin: ApplicationRouteLeaseOrigin::ExplicitPointer,
            initiating_device: None,
            initiating_button: None,
            ..click()
        };
        let next = state
            .replace_explicit_provisional(first.identity, candidate)
            .unwrap();
        assert_ne!(next.identity, first.identity);
        assert_eq!(next.phase, ApplicationRouteLeasePhase::Provisional);
        assert!(
            state
                .confirm(
                    first.identity,
                    first.target_surface,
                    first.admission,
                    first.authority_session_epoch
                )
                .is_err()
        );
        assert!(
            state
                .frontend_release(first.identity, first.admission)
                .is_err()
        );
        let next = state
            .confirm(
                next.identity,
                next.target_surface,
                next.admission,
                next.authority_session_epoch,
            )
            .unwrap();
        // Physical button release is not an explicit ungrab.
        assert_eq!(
            state.frontend_release(next.identity, next.admission),
            Err(ApplicationRouteLeaseError::InvalidOrigin)
        );
        assert_eq!(state.lease(next.identity.seat), Some(next));
        state
            .request_exact_release(next.identity, next.admission, 10)
            .unwrap();
        state
            .acknowledge_release(next.identity, next.admission)
            .unwrap();
        assert!(state.lease(next.identity.seat).is_none());
    }
}

#[test]
fn click_promotion_cannot_change_owner_scope_epoch_or_replace_a_releasing_lease() {
    let mut state = ApplicationRouteLeaseState::default();
    let first = state.begin_provisional(click()).unwrap();
    state
        .confirm(
            first.identity,
            first.target_surface,
            first.admission,
            first.authority_session_epoch,
        )
        .unwrap();
    let original = state.clone();
    let candidate = ApplicationRouteLeaseCandidate {
        origin: ApplicationRouteLeaseOrigin::ExplicitPointer,
        initiating_device: None,
        initiating_button: None,
        ..click()
    };
    for bad in [
        ApplicationRouteLeaseCandidate {
            admission: ClientAdmissionId::from_raw(8),
            ..candidate
        },
        ApplicationRouteLeaseCandidate {
            scope: ApplicationRouteScope {
                authority: NamespaceId::from_raw(5),
                ..candidate.scope
            },
            ..candidate
        },
        ApplicationRouteLeaseCandidate {
            authority_session_epoch: 10,
            ..candidate
        },
        ApplicationRouteLeaseCandidate {
            seat: SeatId::from_raw(2),
            ..candidate
        },
    ] {
        assert!(
            state
                .replace_explicit_provisional(first.identity, bad)
                .is_err()
        );
        assert_eq!(state, original);
    }
    state.request_release(first.identity.seat, 20).unwrap();
    let releasing = state.clone();
    assert_eq!(
        state.replace_explicit_provisional(first.identity, candidate),
        Err(ApplicationRouteLeaseError::InvalidPhase)
    );
    assert_eq!(state, releasing);
}

#[test]
fn an_explicit_grab_still_acknowledges_engine_ordered_scope_exit() {
    let mut state = ApplicationRouteLeaseState::default();
    let lease = state
        .begin_provisional(ApplicationRouteLeaseCandidate {
            origin: ApplicationRouteLeaseOrigin::ExplicitPointer,
            initiating_device: None,
            initiating_button: None,
            ..click()
        })
        .unwrap();
    state
        .confirm(
            lease.identity,
            lease.target_surface,
            lease.admission,
            lease.authority_session_epoch,
        )
        .unwrap();
    state.request_release(lease.identity.seat, 20).unwrap();
    state
        .frontend_release(lease.identity, lease.admission)
        .unwrap();
    assert!(state.lease(lease.identity.seat).is_none());
}
