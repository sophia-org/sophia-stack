//! Resource observation cadence, shared by finite proofs and rolling daily history.
use std::time::{Duration, Instant};

pub const RESOURCE_SAMPLE_INTERVAL: Duration = Duration::from_secs(5);
pub const RESOURCE_SAMPLE_CAPACITY: u64 = 1_560;

/// Passive cadence and sample population; callers own gauge reads and output.
pub struct ResourceSamplingSchedule {
    next: Instant,
    sequence: u64,
    continuous: bool,
}

impl ResourceSamplingSchedule {
    pub fn new(started: Instant, continuous: bool) -> Self {
        Self {
            next: started + RESOURCE_SAMPLE_INTERVAL,
            sequence: 0,
            continuous,
        }
    }

    pub fn is_due(&self, now: Instant) -> bool {
        !self.saturated() && now >= self.next
    }

    pub fn advance(&mut self, now: Instant) -> Option<u64> {
        if !self.is_due(now) {
            return None;
        }
        self.sequence = self.sequence.saturating_add(1);
        // Resume with one current reading, not a burst of invented missed samples.
        self.next = now + RESOURCE_SAMPLE_INTERVAL;
        Some(self.sequence)
    }

    pub fn samples(&self) -> u64 {
        self.sequence
    }
    pub fn saturated(&self) -> bool {
        !self.continuous && self.sequence >= RESOURCE_SAMPLE_CAPACITY
    }
}
