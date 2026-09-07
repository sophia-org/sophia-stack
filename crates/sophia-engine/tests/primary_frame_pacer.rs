use std::time::{Duration, Instant};

use sophia_engine::PrimaryFramePacer;

#[test]
fn busy_content_is_latest_wins_at_the_refresh_cadence() {
    let start = Instant::now();
    let mut pacer = PrimaryFramePacer::new(Duration::from_millis(16));

    assert!(!pacer.defer_production(start, false));
    pacer.observe_production(start, true);
    for offset in [1, 4, 8, 12, 15] {
        let now = start + Duration::from_millis(offset);
        assert!(pacer.defer_production(now, false));
        pacer.observe_production(now, false);
    }
    assert!(!pacer.repaint_due(start + Duration::from_millis(15)));
    assert!(pacer.repaint_due(start + Duration::from_millis(16)));

    pacer.observe_repaint(start + Duration::from_millis(16));
    assert!(!pacer.repaint_pending());
    assert!(!pacer.repaint_due(start + Duration::from_millis(31)));
}

#[test]
fn unrelated_input_wakeups_do_not_move_the_deadline() {
    let start = Instant::now();
    let mut still = PrimaryFramePacer::new(Duration::from_millis(16));
    let mut moving = still;

    assert!(!still.defer_production(start, false));
    still.observe_production(start, true);
    assert!(!moving.defer_production(start, false));
    moving.observe_production(start, true);

    let content = start + Duration::from_millis(4);
    assert!(still.defer_production(content, false));
    still.observe_production(content, false);
    assert!(moving.defer_production(content, false));
    moving.observe_production(content, false);

    // Simulated input wakes only query the wait; they do not request frames.
    for offset in 5..16 {
        assert_eq!(
            moving.cap_wait(
                start + Duration::from_millis(offset),
                Duration::from_millis(25)
            ),
            Duration::from_millis(16 - offset),
        );
    }
    assert_eq!(
        still.repaint_due(start + Duration::from_millis(16)),
        moving.repaint_due(start + Duration::from_millis(16)),
    );
}

#[test]
fn backpressure_also_has_a_bounded_repaint() {
    let start = Instant::now();
    let mut pacer = PrimaryFramePacer::new(Duration::from_millis(16));

    assert!(pacer.defer_production(start, true));
    pacer.observe_production(start, false);
    assert_eq!(
        pacer.cap_wait(start, Duration::from_millis(25)),
        Duration::from_millis(16),
    );
    assert!(pacer.repaint_due(start + Duration::from_millis(16)));
}

#[test]
fn refresh_change_rephases_a_pending_repaint() {
    let start = Instant::now();
    let mut pacer = PrimaryFramePacer::new(Duration::from_millis(16));
    assert!(pacer.defer_production(start, true));

    pacer.set_interval(start + Duration::from_millis(2), Duration::from_millis(8));

    assert_eq!(pacer.interval(), Duration::from_millis(8));
    assert!(!pacer.repaint_due(start + Duration::from_millis(9)));
    assert!(pacer.repaint_due(start + Duration::from_millis(10)));
}

#[test]
fn a_refused_composition_arms_the_next_cadence_repaint() {
    let start = Instant::now();
    let mut pacer = PrimaryFramePacer::new(Duration::from_millis(16));

    // A turn this pacer admitted: the deadline had elapsed, so it asked for a
    // composition and left nothing pending behind it.
    assert!(!pacer.defer_production(start, false));
    assert!(!pacer.repaint_pending());

    // The renderer refused it anyway. A scene that must preserve GPU-owned
    // output composes nothing, and the damage it already took into the scene
    // has no other path to the display.
    pacer.observe_production(start, false);

    assert!(
        pacer.repaint_pending(),
        "a refused composition must leave a request behind",
    );
    assert!(!pacer.repaint_due(start + Duration::from_millis(15)));
    assert!(pacer.repaint_due(start + Duration::from_millis(16)));
}

#[test]
fn a_repaint_the_backend_declines_is_retried_without_spinning() {
    let start = Instant::now();
    let mut pacer = PrimaryFramePacer::new(Duration::from_millis(16));

    assert!(!pacer.defer_production(start, false));
    pacer.observe_production(start, false);

    let due = start + Duration::from_millis(16);
    assert!(pacer.repaint_due(due));
    // Overdue is exactly the state that pins the owner at a zero wait, which
    // is only safe while the repaint it is waiting for can actually run.
    assert_eq!(
        pacer.cap_wait(due, Duration::from_millis(25)),
        Duration::ZERO
    );

    pacer.observe_repaint_deferred(due);

    assert!(
        pacer.repaint_pending(),
        "the pixels still need publishing, so the request survives",
    );
    assert!(
        !pacer.repaint_due(due),
        "a declined repaint must not be retried on the same turn",
    );
    assert_eq!(
        pacer.cap_wait(due, Duration::from_millis(25)),
        Duration::from_millis(16),
        "and the owner must be given something to wait on",
    );
    assert!(
        pacer.repaint_due(due + Duration::from_millis(16)),
        "retried on the next cadence instead",
    );
}

#[test]
fn a_composition_after_a_refusal_settles_the_request() {
    let start = Instant::now();
    let mut pacer = PrimaryFramePacer::new(Duration::from_millis(16));

    assert!(!pacer.defer_production(start, false));
    pacer.observe_production(start, false);
    assert!(pacer.repaint_pending());

    // Whichever way the pixels reach the display -- a later production turn
    // that did compose, or the cadence repaint itself -- the request clears.
    pacer.observe_production(start + Duration::from_millis(4), true);
    assert!(!pacer.repaint_pending());
    assert!(!pacer.repaint_due(start + Duration::from_millis(64)));
}
