use sophia_session::input_delivery::{InputDeliveryState, settle_input_delivery};
use sophia_x_authority::{
    XAuthorityClientInputDelivery, XAuthorityInputDeliveryId, XAuthorityInputDeliveryOutcome,
    XServerFrontendClientId,
};
use std::collections::BTreeSet;

#[test]
fn failed_client_receipt_leaves_other_clients_deliverable_and_settles_once() {
    for failure in [
        XAuthorityInputDeliveryOutcome::RouteRejected,
        XAuthorityInputDeliveryOutcome::WriteFailed,
    ] {
        let failed = XAuthorityInputDeliveryId::from_raw(1);
        let healthy = XAuthorityInputDeliveryId::from_raw(2);
        let mut state = InputDeliveryState {
            fail_on_client_error: false,
            pending: [failed, healthy].map(|id| (id, pending_ticket(id))).into(),
            events_expected: 2,
            ..InputDeliveryState::default()
        };
        let mut barrier: BTreeSet<_> = state.pending.keys().copied().collect();
        let receipt = XAuthorityClientInputDelivery {
            client: XServerFrontendClientId::from_raw(4),
            delivery: failed,
            outcome: failure,
        };
        assert_eq!(
            settle_input_delivery(&mut state, &mut barrier, receipt),
            Ok(Some(failure))
        );
        assert_eq!(state.events_flushed, 0);
        assert_eq!(state.events_failed, 1);
        assert_eq!(barrier, BTreeSet::from([healthy]));
        assert_eq!(
            state.pending.keys().copied().collect::<BTreeSet<_>>(),
            barrier
        );
        assert_eq!(
            settle_input_delivery(&mut state, &mut barrier, receipt),
            Ok(None)
        );
        assert_eq!(state.events_expected, 1);
        assert_eq!(state.events_failed, 1);
        settle_input_delivery(
            &mut state,
            &mut barrier,
            XAuthorityClientInputDelivery {
                client: XServerFrontendClientId::from_raw(1),
                delivery: healthy,
                outcome: XAuthorityInputDeliveryOutcome::Flushed,
            },
        )
        .unwrap();
        assert!(barrier.is_empty());
        assert!(state.pending.is_empty());
        assert_eq!(state.events_expected, state.events_flushed);
        assert_eq!(state.events_flushed, 1);
    }
}

#[test]
fn strict_proof_keeps_delivery_failure_and_cannot_claim_a_flush() {
    for outcome in [
        XAuthorityInputDeliveryOutcome::RouteRejected,
        XAuthorityInputDeliveryOutcome::WriteFailed,
    ] {
        let delivery = XAuthorityInputDeliveryId::from_raw(3);
        let mut state = InputDeliveryState {
            pending: [(delivery, pending_ticket(delivery))].into(),
            events_expected: 1,
            ..InputDeliveryState::default()
        };
        let mut barrier: BTreeSet<_> = state.pending.keys().copied().collect();
        assert!(
            settle_input_delivery(
                &mut state,
                &mut barrier,
                XAuthorityClientInputDelivery {
                    client: XServerFrontendClientId::from_raw(4),
                    delivery,
                    outcome,
                }
            )
            .is_err()
        );
        assert_eq!(state.events_flushed, 0);
        assert_eq!(state.events_expected, 1);
        assert!(barrier.is_empty());
    }
}

fn pending_ticket(
    delivery: XAuthorityInputDeliveryId,
) -> sophia_session::input_delivery::PendingInputDelivery {
    sophia_session::input_delivery::PendingInputDelivery {
        ticket: sophia_x_authority::XAuthorityInputDeliveryTicket {
            delivery,
            surface: sophia_protocol::SurfaceId::new(1, 1),
            seat: sophia_protocol::SeatId::from_raw(1),
            control_epoch: 1,
            admitted_at: std::time::Instant::now(),
            client: None,
        },
        release_barrier: true,
    }
}

#[test]
fn a_receipt_from_another_connection_does_not_release_the_barrier() {
    let id = XAuthorityInputDeliveryId::from_raw(1);
    let client = XServerFrontendClientId::from_raw(1);
    let mut pending = pending_ticket(id);
    pending.ticket.client = Some(client);
    let mut state = InputDeliveryState {
        pending: [(id, pending)].into(),
        ..Default::default()
    };
    let mut barrier = BTreeSet::from([id]);
    assert_eq!(
        settle_input_delivery(
            &mut state,
            &mut barrier,
            XAuthorityClientInputDelivery {
                client: XServerFrontendClientId::from_raw(2),
                delivery: id,
                outcome: XAuthorityInputDeliveryOutcome::Flushed,
            }
        ),
        Ok(None)
    );
    assert_eq!(state.pending.len(), 1);
    assert!(barrier.contains(&id));
    assert_eq!(state.events_flushed, 0);
}
