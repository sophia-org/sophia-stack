//! Routing readiness and presentation binding.
//!
//! Readiness replaced two hand-written phase matches, one in the Engine and one
//! in the session, which agreed only because there happened to be three phases.
//! These tests exist to keep the single decision total and its precedence
//! deliberate, and to hold the line that readiness is not permission.

use sophia_engine::{
    ApplicationRouteLeaseBinding, ApplicationRouteLeaseBindingTimeout,
    ApplicationRouteLeaseCandidate, ApplicationRouteLeaseError, ApplicationRouteLeaseOrigin,
    ApplicationRouteLeasePhase, ApplicationRouteLeaseReadiness, ApplicationRouteLeaseState,
    ApplicationRouteScope, ApplicationRouteTargetEvidence,
};
use sophia_protocol::{
    ClientAdmissionId, DeviceId, NamespaceId, NamespaceProfile, OutputId, SeatId, SurfaceId,
};

const SEAT: SeatId = SeatId::from_raw(1);
const TARGET: SurfaceId = SurfaceId::new(40, 3);
const ADMISSION: ClientAdmissionId = ClientAdmissionId::from_raw(7);
const DEVICE: DeviceId = DeviceId::from_raw(5);
const OUTPUT: OutputId = OutputId::from_raw(2);
const SESSION_EPOCH: u64 = 9;
const DEADLINE_MSEC: u64 = 4_000;

fn scope() -> ApplicationRouteScope {
    ApplicationRouteScope {
        profile: NamespaceProfile::Confined,
        authority: NamespaceId::from_raw(4),
    }
}

fn foreign_scope() -> ApplicationRouteScope {
    ApplicationRouteScope {
        profile: NamespaceProfile::Confined,
        authority: NamespaceId::from_raw(99),
    }
}

fn candidate(
    origin: ApplicationRouteLeaseOrigin,
    binding: ApplicationRouteLeaseBinding,
) -> ApplicationRouteLeaseCandidate {
    ApplicationRouteLeaseCandidate {
        seat: SEAT,
        origin,
        target_surface: TARGET,
        admission: ADMISSION,
        scope: scope(),
        authority_session_epoch: SESSION_EPOCH,
        binding,
        initiating_device: Some(DEVICE),
        initiating_button: Some(0x110),
    }
}

fn bound() -> ApplicationRouteLeaseBinding {
    ApplicationRouteLeaseBinding::Bound {
        output: OUTPUT,
        revision: 11,
    }
}

fn awaiting(pinned: Option<OutputId>) -> ApplicationRouteLeaseBinding {
    ApplicationRouteLeaseBinding::AwaitingPresentation {
        pinned_output: pinned,
        deadline_msec: DEADLINE_MSEC,
    }
}

fn evidence(revision: u64) -> ApplicationRouteTargetEvidence {
    ApplicationRouteTargetEvidence {
        resolved_scope: scope(),
        target_surface: TARGET,
        target_admission: ADMISSION,
        target_eligible: true,
        presentation_revision: revision,
        output: OUTPUT,
        device: DEVICE,
        authority_session_epoch: SESSION_EPOCH,
    }
}

/// An active lease bound to an output, ready to be validated.
fn active_bound() -> (
    ApplicationRouteLeaseState,
    sophia_engine::ApplicationRouteLease,
) {
    let mut state = ApplicationRouteLeaseState::default();
    let lease = state
        .begin_provisional(candidate(
            ApplicationRouteLeaseOrigin::ExplicitPointer,
            bound(),
        ))
        .unwrap();
    let lease = state
        .confirm(lease.identity, TARGET, ADMISSION, SESSION_EPOCH)
        .unwrap();
    (state, lease)
}

/// Every combination of origin, phase and binding has a deliberate answer.
///
/// Written out rather than sampled: a combination nobody considered, silently
/// routing input, is the failure this decision replaces.
#[test]
fn routing_readiness_is_total_over_origin_phase_and_binding() {
    use ApplicationRouteLeaseOrigin::{ExplicitPointer, PointerBoundary};
    use ApplicationRouteLeasePhase::{Active, Provisional};
    use ApplicationRouteLeaseReadiness as R;
    let releasing = ApplicationRouteLeasePhase::Releasing {
        deadline_msec: 1_000,
    };

    let expected = [
        (
            PointerBoundary,
            Provisional,
            true,
            R::ReadyForEvidenceValidation,
        ),
        (PointerBoundary, Provisional, false, R::WaitForPresentation),
        (PointerBoundary, Active, true, R::ReadyForEvidenceValidation),
        (PointerBoundary, Active, false, R::WaitForPresentation),
        (PointerBoundary, releasing, true, R::Releasing),
        (PointerBoundary, releasing, false, R::Releasing),
        (ExplicitPointer, Provisional, true, R::WaitForActivation),
        (ExplicitPointer, Provisional, false, R::WaitForActivation),
        (ExplicitPointer, Active, true, R::ReadyForEvidenceValidation),
        (ExplicitPointer, Active, false, R::WaitForPresentation),
        (ExplicitPointer, releasing, true, R::Releasing),
        (ExplicitPointer, releasing, false, R::Releasing),
    ];
    assert_eq!(expected.len(), 2 * 3 * 2, "every combination is covered");

    let mut state = ApplicationRouteLeaseState::default();
    for (origin, phase, is_bound, want) in expected {
        let binding = if is_bound { bound() } else { awaiting(None) };
        let mut lease = state.begin_provisional(candidate(origin, binding)).unwrap();
        lease.phase = phase;
        assert_eq!(
            lease.routing_readiness(),
            want,
            "{origin:?} {phase:?} bound={is_bound}"
        );
        state = ApplicationRouteLeaseState::default();
    }
}

/// The order of the arms is the contract, not an accident of matching.
///
/// Releasing outranks waiting: a lease on its way out that waited for a
/// presentation it will never need would leave the seat with input going
/// nowhere. Activation outranks presentation: an explicit grab must not begin
/// routing because its target appeared, only because its client activated it.
#[test]
fn readiness_precedence_puts_releasing_first_and_activation_before_presentation() {
    let mut state = ApplicationRouteLeaseState::default();
    let mut lease = state
        .begin_provisional(candidate(
            ApplicationRouteLeaseOrigin::ExplicitPointer,
            awaiting(None),
        ))
        .unwrap();

    // Unbound and unactivated at once: activation wins.
    assert_eq!(
        lease.routing_readiness(),
        ApplicationRouteLeaseReadiness::WaitForActivation
    );

    // Releasing and unbound at once: releasing wins over both.
    lease.phase = ApplicationRouteLeasePhase::Releasing {
        deadline_msec: 1_000,
    };
    assert_eq!(
        lease.routing_readiness(),
        ApplicationRouteLeaseReadiness::Releasing
    );
}

/// An automatic click routes while still provisional.
///
/// Existing behaviour, named so it cannot be lost. The pointer-boundary lease
/// is created and used within one event, so requiring activation for it would
/// break every ordinary click rather than only an exotic case.
#[test]
fn an_automatic_provisional_click_bound_to_an_output_stays_routable() {
    let mut state = ApplicationRouteLeaseState::default();
    let lease = state
        .begin_provisional(candidate(
            ApplicationRouteLeaseOrigin::PointerBoundary,
            bound(),
        ))
        .unwrap();
    assert_eq!(lease.phase, ApplicationRouteLeasePhase::Provisional);
    assert_eq!(
        lease.routing_readiness(),
        ApplicationRouteLeaseReadiness::ReadyForEvidenceValidation
    );
    assert!(state.authorize(lease.identity, evidence(11)).is_ok());
}

/// A changed scene revision is not a refusal.
///
/// This is the repair. The revision advances whenever anything on the output is
/// added or removed, so refusing on it cancelled a held grab whenever an
/// unrelated popup opened anywhere on that output.
#[test]
fn an_unrelated_scene_revision_does_not_end_a_held_grab() {
    let (mut state, lease) = active_bound();
    assert!(state.authorize(lease.identity, evidence(11)).is_ok());

    let authorized = state
        .authorize(lease.identity, evidence(12))
        .expect("a scene change elsewhere does not end the grab");
    assert_eq!(
        authorized.binding,
        ApplicationRouteLeaseBinding::Bound {
            output: OUTPUT,
            revision: 12,
        },
        "the lease records the scene it was last validated against"
    );
}

/// Losing the target ends the grab, whatever the revision says.
#[test]
fn losing_the_target_ends_the_grab() {
    for (name, mutate) in [
        (
            "the target is no longer reachable",
            (|e: &mut ApplicationRouteTargetEvidence| e.target_eligible = false)
                as fn(&mut ApplicationRouteTargetEvidence),
        ),
        ("the target belongs to another client", |e| {
            e.target_admission = ClientAdmissionId::from_raw(8)
        }),
        ("the pointer left the leased scope", |e| {
            e.resolved_scope = foreign_scope()
        }),
        ("another device", |e| e.device = DeviceId::from_raw(6)),
        ("another output", |e| e.output = OutputId::from_raw(3)),
        ("another authority session", |e| {
            e.authority_session_epoch = SESSION_EPOCH + 1
        }),
    ] {
        let (mut state, lease) = active_bound();
        let mut probe = evidence(11);
        mutate(&mut probe);
        assert!(
            state.authorize(lease.identity, probe).is_err(),
            "must refuse when {name}"
        );
    }
}

/// Evidence about one surface cannot authorize its replacement on the same
/// seat.
#[test]
fn evidence_for_a_replaced_surface_cannot_authorize_the_replacement() {
    let (mut state, lease) = active_bound();
    let mut stale = evidence(11);
    stale.target_surface = SurfaceId::new(41, 1);
    assert_eq!(
        state.authorize(lease.identity, stale),
        Err(ApplicationRouteLeaseError::IdentityMismatch)
    );
}

/// A readiness observed earlier is not a permit later.
///
/// Readiness is informational. Between reading one and acting on it the lease
/// can be released or revoked, so authorization re-resolves the lease and asks
/// again rather than accepting a remembered answer.
#[test]
fn a_readiness_seen_earlier_cannot_authorize_a_later_delivery() {
    // Released between the observation and the delivery.
    let (mut state, lease) = active_bound();
    assert_eq!(
        state.routing_readiness(SEAT),
        Some(ApplicationRouteLeaseReadiness::ReadyForEvidenceValidation)
    );
    state.request_release(SEAT, 0).unwrap();
    assert!(
        matches!(
            state.authorize(lease.identity, evidence(11)),
            Err(ApplicationRouteLeaseError::NotRoutable(
                ApplicationRouteLeaseReadiness::Releasing
            ))
        ),
        "a lease that began releasing cannot still deliver"
    );

    // Revoked between the observation and the delivery.
    let (mut state, lease) = active_bound();
    assert_eq!(
        state.routing_readiness(SEAT),
        Some(ApplicationRouteLeaseReadiness::ReadyForEvidenceValidation)
    );
    state.revoke_admission(ADMISSION);
    assert_eq!(
        state.authorize(lease.identity, evidence(11)),
        Err(ApplicationRouteLeaseError::NoLease),
        "a revoked lease cannot still deliver"
    );
}

/// An unbound lease pins an output, inherits the pin, and binds only there.
#[test]
fn an_unbound_lease_pins_an_output_and_binds_only_there() {
    let mut state = ApplicationRouteLeaseState::default();
    let lease = state
        .begin_provisional(candidate(
            ApplicationRouteLeaseOrigin::ExplicitPointer,
            awaiting(None),
        ))
        .unwrap();

    let pinned = state.pin_output(lease.identity, OUTPUT).unwrap();
    assert_eq!(
        pinned.binding,
        ApplicationRouteLeaseBinding::AwaitingPresentation {
            pinned_output: Some(OUTPUT),
            deadline_msec: DEADLINE_MSEC,
        }
    );

    // The pin decides where a binding may land; it is not advice.
    assert!(
        state
            .bind_presentation(lease.identity, OutputId::from_raw(3), 5)
            .is_err(),
        "binding elsewhere than the pin is refused"
    );
    let bound = state.bind_presentation(lease.identity, OUTPUT, 5).unwrap();
    assert_eq!(
        bound.binding,
        ApplicationRouteLeaseBinding::Bound {
            output: OUTPUT,
            revision: 5,
        }
    );
}

/// The binding deadline reports without taking the seat away.
///
/// The frontend still holds an X grab when the deadline passes. Dropping the
/// lease here would let a shell capture take the seat before that grab is
/// released, so expiry is reported and the caller runs the ordered release.
#[test]
fn a_binding_deadline_reports_without_removing_and_stops_once_releasing() {
    let mut state = ApplicationRouteLeaseState::default();
    let lease = state
        .begin_provisional(candidate(
            ApplicationRouteLeaseOrigin::ExplicitPointer,
            awaiting(Some(OUTPUT)),
        ))
        .unwrap();

    assert_eq!(
        state.observe_binding_deadline(SEAT, DEADLINE_MSEC - 1),
        ApplicationRouteLeaseBindingTimeout::Pending
    );
    let expired = state.observe_binding_deadline(SEAT, DEADLINE_MSEC);
    assert!(matches!(
        expired,
        ApplicationRouteLeaseBindingTimeout::Expired(_)
    ));
    assert!(
        state.lease(SEAT).is_some(),
        "ownership is retained until the release handshake completes"
    );

    // Once the caller begins releasing, it is not reported again.
    state.request_release(SEAT, 0).unwrap();
    assert_eq!(
        state.observe_binding_deadline(SEAT, DEADLINE_MSEC + 10_000),
        ApplicationRouteLeaseBindingTimeout::NotAwaiting
    );
    let _ = lease;
}

/// Topology loss takes every lease that could still land on the lost output.
#[test]
fn topology_loss_cancels_unbound_leases_including_unpinned_ones() {
    for pinned in [None, Some(OUTPUT)] {
        let mut state = ApplicationRouteLeaseState::default();
        state
            .begin_provisional(candidate(
                ApplicationRouteLeaseOrigin::ExplicitPointer,
                awaiting(pinned),
            ))
            .unwrap();
        let dropped = state.lose_output(OUTPUT);
        assert_eq!(
            dropped.len(),
            1,
            "an unbound lease does not survive an unknown topology, pinned={pinned:?}"
        );
        assert!(state.lease(SEAT).is_none());
    }

    // A lease pinned to a surviving output is kept.
    let mut state = ApplicationRouteLeaseState::default();
    state
        .begin_provisional(candidate(
            ApplicationRouteLeaseOrigin::ExplicitPointer,
            awaiting(Some(OutputId::from_raw(3))),
        ))
        .unwrap();
    assert!(state.lose_output(OUTPUT).is_empty());
    assert!(state.lease(SEAT).is_some());
}

/// A promotion cannot bind somewhere the lease it replaced could not.
///
/// The original was bound to one output, so the explicit replacement inherits
/// that as a pin even though its candidate named none. Without the inheritance
/// a promotion could land on a different output entirely, and `authorize`
/// requires the output to match, so every later event would fail.
#[test]
fn a_promotion_inherits_the_output_its_original_was_bound_to() {
    let mut state = ApplicationRouteLeaseState::default();
    let first = state
        .begin_provisional(candidate(
            ApplicationRouteLeaseOrigin::PointerBoundary,
            ApplicationRouteLeaseBinding::Bound {
                output: OUTPUT,
                revision: 11,
            },
        ))
        .unwrap();

    let mut replacement = candidate(ApplicationRouteLeaseOrigin::ExplicitPointer, awaiting(None));
    replacement.target_surface = first.target_surface;
    let promoted = state
        .replace_explicit_provisional(first.identity, replacement)
        .unwrap();
    assert_eq!(
        promoted.binding,
        ApplicationRouteLeaseBinding::AwaitingPresentation {
            pinned_output: Some(OUTPUT),
            deadline_msec: DEADLINE_MSEC,
        },
        "the replacement is pinned to where the original was bound"
    );

    // The inherited pin is binding, not advice.
    assert_eq!(
        state.bind_presentation(promoted.identity, OutputId::from_raw(3), 5),
        Err(ApplicationRouteLeaseError::StalePresentation),
        "a promotion cannot bind to an output the original never reached"
    );
    assert!(
        state
            .bind_presentation(promoted.identity, OUTPUT, 5)
            .is_ok()
    );
}

/// A promotion cannot buy itself more time.
///
/// The deadline is absolute from the original reservation. Taking the later of
/// the two would let a client hold a seat indefinitely by promoting repeatedly.
#[test]
fn a_promotion_cannot_extend_the_binding_deadline() {
    let mut state = ApplicationRouteLeaseState::default();
    let first = state
        .begin_provisional(candidate(
            ApplicationRouteLeaseOrigin::PointerBoundary,
            ApplicationRouteLeaseBinding::AwaitingPresentation {
                pinned_output: None,
                deadline_msec: DEADLINE_MSEC,
            },
        ))
        .unwrap();

    let mut replacement = candidate(
        ApplicationRouteLeaseOrigin::ExplicitPointer,
        ApplicationRouteLeaseBinding::AwaitingPresentation {
            pinned_output: None,
            deadline_msec: DEADLINE_MSEC * 10,
        },
    );
    replacement.target_surface = first.target_surface;
    let promoted = state
        .replace_explicit_provisional(first.identity, replacement)
        .unwrap();
    assert_eq!(
        promoted.binding,
        ApplicationRouteLeaseBinding::AwaitingPresentation {
            pinned_output: None,
            deadline_msec: DEADLINE_MSEC,
        },
        "the earlier deadline survives the promotion"
    );

    // A shorter candidate deadline is honoured, so this takes the earlier of
    // the two rather than always keeping the original. Promotion is only
    // allowed from an automatic lease, so this starts from a fresh one.
    let mut state = ApplicationRouteLeaseState::default();
    let first = state
        .begin_provisional(candidate(
            ApplicationRouteLeaseOrigin::PointerBoundary,
            ApplicationRouteLeaseBinding::AwaitingPresentation {
                pinned_output: None,
                deadline_msec: DEADLINE_MSEC,
            },
        ))
        .unwrap();
    let mut shorter = candidate(
        ApplicationRouteLeaseOrigin::ExplicitPointer,
        ApplicationRouteLeaseBinding::AwaitingPresentation {
            pinned_output: None,
            deadline_msec: 10,
        },
    );
    shorter.target_surface = first.target_surface;
    let promoted = state
        .replace_explicit_provisional(first.identity, shorter)
        .unwrap();
    assert_eq!(
        promoted.binding,
        ApplicationRouteLeaseBinding::AwaitingPresentation {
            pinned_output: None,
            deadline_msec: 10,
        }
    );
}

/// Losing an output cancels leases bound to it whatever scene they hold.
///
/// The output is gone. A lease bound against the current revision is no less
/// stranded than one bound against an older scene.
#[test]
fn losing_an_output_cancels_leases_bound_to_it_at_any_revision() {
    for revision in [11, 12, u64::MAX] {
        let mut state = ApplicationRouteLeaseState::default();
        state
            .begin_provisional(candidate(
                ApplicationRouteLeaseOrigin::PointerBoundary,
                ApplicationRouteLeaseBinding::Bound {
                    output: OUTPUT,
                    revision,
                },
            ))
            .unwrap();
        assert_eq!(
            state.lose_output(OUTPUT).len(),
            1,
            "a lost output cancels its leases at revision {revision}"
        );
        assert!(state.lease(SEAT).is_none());
    }

    // A lease on a surviving output is untouched.
    let mut state = ApplicationRouteLeaseState::default();
    state
        .begin_provisional(candidate(
            ApplicationRouteLeaseOrigin::PointerBoundary,
            ApplicationRouteLeaseBinding::Bound {
                output: OutputId::from_raw(3),
                revision: 11,
            },
        ))
        .unwrap();
    assert!(state.lose_output(OUTPUT).is_empty());
    assert!(state.lease(SEAT).is_some());
}
