#[path = "../src/gbm_platform/scanout/context/image_transfer_policy.rs"]
mod policy;

use policy::{BridgeSelection as Choice, select_bridge, source_attempt_order};

#[test]
fn all_busy_slots_defer_without_reusing_a_matching_layout() {
    assert_eq!(
        select_bridge([(false, true); 3].into_iter(), 3),
        Choice::Deferred
    );
    assert_eq!(
        select_bridge([(false, false); 3].into_iter(), 3),
        Choice::Deferred
    );
}

#[test]
fn completed_matching_storage_wins_over_an_unrelated_idle_slot() {
    assert_eq!(
        select_bridge([(true, false), (false, true), (true, true)].into_iter(), 3),
        Choice::Reuse(2)
    );
    assert_eq!(
        select_bridge([(true, false), (false, true)].into_iter(), 3),
        Choice::Replace(0)
    );
    assert_eq!(
        select_bridge([(false, true); 2].into_iter(), 3),
        Choice::Allocate
    );
    assert_eq!(select_bridge([].into_iter(), 3), Choice::Allocate);
}

#[test]
fn a_source_hint_neither_omits_other_devices_nor_repeats_its_slot() {
    assert_eq!(
        source_attempt_order(4, Some(2)).collect::<Vec<_>>(),
        [2, 0, 1, 3]
    );
    assert_eq!(
        source_attempt_order(4, None).collect::<Vec<_>>(),
        [0, 1, 2, 3]
    );
    assert_eq!(
        source_attempt_order(4, Some(8)).collect::<Vec<_>>(),
        [0, 1, 2, 3]
    );
    assert!(source_attempt_order(0, Some(0)).next().is_none());
}

#[test]
fn a_remembered_foreign_layout_never_bypasses_the_next_actual_import() {
    let mut remembered_source = Some(2);
    let mut imports = 0;
    let result = policy::capture_direct_or_transfer(
        &mut remembered_source,
        |hint| {
            assert_eq!(*hint, Some(2));
            imports += 1;
            Ok::<_, bool>("local actual FD")
        },
        |error| error,
        |_, _| panic!("a source hint must not force another transfer"),
    );
    assert_eq!(result, Ok("local actual FD"));
    assert_eq!(imports, 1);
}

#[test]
fn fallback_requires_an_import_refusal_and_preserves_resource_failures() {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Failure {
        Import,
        Budget,
        Context,
        DeviceLost,
    }
    for failure in [
        Failure::Import,
        Failure::Budget,
        Failure::Context,
        Failure::DeviceLost,
    ] {
        let mut attempts = Vec::new();
        let result = policy::capture_direct_or_transfer(
            &mut attempts,
            |attempts| {
                attempts.push("actual direct import");
                Err(failure)
            },
            |error| error == Failure::Import,
            |attempts, error| {
                assert_eq!(error, Failure::Import);
                attempts.push("transfer");
                Ok(())
            },
        );
        if failure == Failure::Import {
            assert_eq!(result, Ok(()));
            assert_eq!(attempts, ["actual direct import", "transfer"]);
        } else {
            assert_eq!(result, Err(failure));
            assert_eq!(attempts, ["actual direct import"]);
        }
    }
}
