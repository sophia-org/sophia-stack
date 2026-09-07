use sophia_session::resource_sampling::{
    RESOURCE_SAMPLE_CAPACITY, RESOURCE_SAMPLE_INTERVAL, ResourceSamplingSchedule,
};
use std::time::{Duration, Instant};

#[test]
fn daily_observations_continue_through_eight_hours_while_proofs_remain_finite() {
    let started = Instant::now();
    let mut daily = ResourceSamplingSchedule::new(started, true);
    let mut proof = ResourceSamplingSchedule::new(started, false);
    for sample in 1..=5760u64 {
        let now = started + RESOURCE_SAMPLE_INTERVAL * sample as u32;
        assert_eq!(daily.advance(now), Some(sample));
        assert_eq!(
            proof.advance(now),
            (sample <= RESOURCE_SAMPLE_CAPACITY).then_some(sample)
        );
    }
    assert_eq!(daily.samples(), 5760);
    assert!(!daily.saturated());
    assert_eq!(proof.samples(), RESOURCE_SAMPLE_CAPACITY);
    assert!(proof.saturated());
}

#[test]
fn resume_records_one_current_reading_without_fabricating_a_backlog() {
    let started = Instant::now();
    let mut schedule = ResourceSamplingSchedule::new(started, true);
    assert_eq!(schedule.advance(started), None);
    let resumed = started + Duration::from_secs(3600);
    assert_eq!(schedule.advance(resumed), Some(1));
    assert_eq!(schedule.advance(resumed), None);
    assert_eq!(
        schedule.advance(resumed + RESOURCE_SAMPLE_INTERVAL),
        Some(2)
    );
}
