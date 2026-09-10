#![cfg(all(test, unix))]

use super::*;
use crate::runtime::XPresentAllocationSubject;

fn subject(
    runtime: &mut XAuthorityRuntime,
    window: XResourceId,
    transaction: u64,
) -> XPresentAllocationSubject {
    let response = runtime.present_standard_pixmap(
        TransactionId::from_raw(transaction),
        NS,
        window,
        PIXMAP,
        0,
        0,
        None,
        None,
    );
    assert_eq!(response.outcome, XAuthorityResponseOutcome::Accepted);
    assert_eq!(response.transactions.len(), 1);
    let published = &response.transactions[0];
    let sophia_protocol::BufferSource::DmaBuf { handle } = published.target_buffer() else {
        panic!("a reallocation subject must come from an admitted DMA Present");
    };
    runtime
        .present_allocation_subject(
            NS,
            PRESENTER.raw(),
            window,
            PIXMAP,
            &XAuthorityPresentSubmission {
                transaction: response.transaction,
                surface: published.surface,
                buffer: BufferHandle::from_raw(handle),
                x_offset: 0,
                y_offset: 0,
                acquire_fence: None,
                idle_fence: None,
            },
        )
        .unwrap()
}

#[test]
fn present_reallocation_claim_is_once_per_surface_not_per_transaction() {
    let fixture = Fixture::new();
    let mut runtime = fixture.state.runtime.lock().unwrap();
    let first = subject(&mut runtime, WINDOW, 101);
    let later = subject(&mut runtime, WINDOW, 102);
    assert!(runtime.claim_present_reallocation(first, fixture.comparison));
    assert!(!runtime.claim_present_reallocation(first, fixture.comparison));
    assert!(!runtime.claim_present_reallocation(later, fixture.comparison));
    assert!(runtime.compare_present_layout(later, fixture.comparison));
}

#[test]
fn present_reallocation_claim_survives_rejected_updates_and_resets_only_on_acceptance() {
    let fixture = Fixture::new();
    let mut runtime = fixture.state.runtime.lock().unwrap();
    let subject = subject(&mut runtime, WINDOW, 103);
    assert!(runtime.claim_present_reallocation(subject, fixture.comparison));

    assert_eq!(
        runtime.update_window_allocation_preferences(Fixture::snapshot(fixture.comparison)),
        XWindowAllocationUpdate::Stale
    );
    assert!(!runtime.claim_present_reallocation(subject, fixture.comparison));

    let mut next = fixture.comparison;
    next.preference_generation += 1;
    let mut invalid = Fixture::snapshot(next);
    invalid.windows.push(invalid.windows[0].clone());
    assert_eq!(
        runtime.update_window_allocation_preferences(invalid),
        XWindowAllocationUpdate::Invalid
    );
    assert!(!runtime.claim_present_reallocation(subject, fixture.comparison));

    let mut stale_topology = Fixture::snapshot(next);
    stale_topology.topology_generation += 1;
    assert_eq!(
        runtime.update_window_allocation_preferences(stale_topology),
        XWindowAllocationUpdate::Stale
    );
    assert!(!runtime.claim_present_reallocation(subject, fixture.comparison));
    assert_eq!(
        runtime.update_window_allocation_preferences(Fixture::snapshot(next)),
        XWindowAllocationUpdate::Applied
    );
    assert!(!runtime.claim_present_reallocation(subject, fixture.comparison));
    assert!(runtime.claim_present_reallocation(subject, next));
    assert!(!runtime.claim_present_reallocation(subject, next));
}

#[test]
fn present_reallocation_claim_does_not_consume_a_refused_comparison() {
    for case in 0..6 {
        let fixture = Fixture::new();
        let mut runtime = fixture.state.runtime.lock().unwrap();
        let subject = subject(&mut runtime, WINDOW, 104);
        let mut invalid = fixture.comparison;
        match case {
            0 => invalid.buffer = BufferHandle::from_raw(invalid.buffer.raw() + 1),
            1 => invalid.original_modifier = 0,
            2 => invalid.alternative_modifier = TILED,
            3 => invalid.native_context.generation += 1,
            4 => invalid.geometry.x += 1,
            5 => invalid.preference_generation += 1,
            _ => unreachable!(),
        }
        assert!(
            !runtime.claim_present_reallocation(subject, invalid),
            "case={case}"
        );
        assert!(
            runtime.claim_present_reallocation(subject, fixture.comparison),
            "the refused comparison consumed the claim in case={case}"
        );
        assert!(!runtime.claim_present_reallocation(subject, fixture.comparison));
    }
}

#[test]
fn present_reallocation_claims_are_independent_for_current_surfaces() {
    let fixture = Fixture::new();
    let mut runtime = fixture.state.runtime.lock().unwrap();
    let other_window = XResourceId::new(0x200002, 1);
    let other_surface = SurfaceId::new(74, 1);
    create_window(&mut runtime, other_window, other_surface);
    let mut first_comparison = fixture.comparison;
    first_comparison.preference_generation += 1;
    let mut other_comparison = first_comparison;
    other_comparison.surface = other_surface;
    let mut snapshot = Fixture::snapshot(first_comparison);
    snapshot
        .windows
        .extend(Fixture::snapshot(other_comparison).windows);
    assert_eq!(
        runtime.update_window_allocation_preferences(snapshot),
        XWindowAllocationUpdate::Applied
    );
    let first = subject(&mut runtime, WINDOW, 105);
    let other = subject(&mut runtime, other_window, 106);
    assert!(runtime.claim_present_reallocation(first, first_comparison));
    assert!(runtime.claim_present_reallocation(other, other_comparison));
    assert!(!runtime.claim_present_reallocation(first, first_comparison));
    assert!(!runtime.claim_present_reallocation(other, other_comparison));
}

#[test]
fn present_reallocation_claim_requires_the_exact_window_lifetime_and_current_row() {
    let fixture = Fixture::new();
    let mut runtime = fixture.state.runtime.lock().unwrap();
    let original = subject(&mut runtime, WINDOW, 107);
    assert!(runtime.claim_present_reallocation(original, fixture.comparison));
    assert_eq!(runtime.destroy_window(NS, WINDOW).unwrap(), SURFACE);
    assert!(!runtime.claim_present_reallocation(original, fixture.comparison));
    let replacement_surface = SurfaceId::new(73, 2);
    create_window(&mut runtime, WINDOW, replacement_surface);
    let replacement = subject(&mut runtime, WINDOW, 108);
    let mut comparison = fixture.comparison;
    comparison.surface = replacement_surface;
    assert!(!runtime.claim_present_reallocation(replacement, comparison));
    comparison.preference_generation += 1;
    assert_eq!(
        runtime.update_window_allocation_preferences(Fixture::snapshot(comparison)),
        XWindowAllocationUpdate::Applied
    );
    assert!(!runtime.claim_present_reallocation(original, comparison));
    assert!(runtime.claim_present_reallocation(replacement, comparison));
    assert!(!runtime.claim_present_reallocation(replacement, comparison));
}
