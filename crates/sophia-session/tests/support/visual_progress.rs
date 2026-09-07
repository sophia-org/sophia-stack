#![cfg(test)]

use super::*;
use sophia_backend_live::LiveProductionNativeFrameId;

fn snapshot() -> HeadSnapshot {
    HeadSnapshot {
        generation: 1,
        enabled: true,
        pending: None,
        rendering: None,
        submitted: None,
        presented: None,
        submissions: 2,
        retirements: 1,
    }
}

#[test]
fn unchanged_snapshots_emit_nothing_and_gaps_are_not_fabricated_transitions() {
    let before = snapshot();
    assert_eq!(head_line((1, 2), Some(before), before), None);
    let after = HeadSnapshot {
        submissions: 6,
        retirements: 4,
        ..before
    };
    let line = head_line((1, 2), Some(before), after).unwrap();
    assert!(line.contains("submissions_delta=4 retirements_delta=3 missed_count=5"));
    assert!(line.contains("baseline=false"));
    assert!(line.contains("status=head_snapshot"));
}

#[test]
fn new_generation_and_counter_reset_establish_a_baseline() {
    let before = snapshot();
    for next in [
        HeadSnapshot {
            generation: 2,
            ..before
        },
        HeadSnapshot {
            submissions: 0,
            retirements: 0,
            ..before
        },
    ] {
        let line = head_line((1, 2), Some(before), next).unwrap();
        assert!(line.contains("baseline=true"));
        assert!(line.contains("submissions_delta=0 retirements_delta=0 missed_count=0"));
    }
}

#[test]
fn pixels_and_checksums_never_become_progress_identity() {
    let frame = LiveProductionNativeFrameId::from_raw(7);
    let a = ContentIdentity::from_content(LiveProductionScanoutContent::Cpu {
        frame,
        checksum: 0xdeadbeef,
    });
    let b = ContentIdentity::from_content(LiveProductionScanoutContent::Cpu {
        frame,
        checksum: 0x12345678,
    });
    assert_eq!(a, b);
    assert_eq!(a.to_string(), "cpu:7:none");
    let mixed = ContentIdentity::from_content(LiveProductionScanoutContent::MixedPresent {
        frame,
        transaction: TransactionId::from_raw(19),
        nonzero_rgb_pixels: 99999,
    });
    assert_eq!(mixed.to_string(), "mixed_present:7:19");
}

#[test]
fn opt_in_and_intake_labels_do_not_claim_acceptance() {
    assert!(enabled_value(Some("1")));
    assert!(enabled_value(Some("true")));
    for value in [
        None,
        Some("false"),
        Some("0"),
        Some("TRUE"),
        Some("anything"),
    ] {
        assert!(!enabled_value(value));
    }
}
