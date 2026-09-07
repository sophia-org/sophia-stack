// Periodic resource sampling, so "no steady-state growth" is measured rather
// than inferred.
//
// Every resource figure the session reports is emitted once, at completion. A
// single end-of-session record answers whether the session drained -- live
// entries at zero, no slot still leased -- and cannot answer whether anything
// grew while it ran. A session that leaks one buffer a minute for two hours and
// frees them all at teardown produces the same clean completion record as one
// that never held more than three.
//
// Milestone 14 exits on "bounded warmed resource counts, no steady-state
// allocation growth", and until now nothing measured the second clause. This
// samples the same gauges the completion record carries, on a bounded cadence,
// so a verifier can compare the run against itself.
//
// The sampler states facts and draws no conclusion. Whether a population grew
// is decided by the verifier from the samples, because an emitter that graded
// its own health would be the only witness to its own failure.

pub(crate) use crate::resource_sampling::{RESOURCE_SAMPLE_CAPACITY, RESOURCE_SAMPLE_INTERVAL};

struct LiveResourceSampler {
    started: Instant,
    schedule: crate::resource_sampling::ResourceSamplingSchedule,
}

/// One reading of the gauges a growth check compares.
///
/// Every field is a live count rather than a total, because a total only ever
/// rises and says nothing about whether anything is being held. `rss_kib` is
/// the process's own resident size, which is the only figure here that includes
/// allocations Sophia does not itself account for.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct LiveResourceSample {
    pub cpu_registry_buffers: usize,
    pub cpu_registry_bytes: usize,
    pub cpu_cow_splits: u64,
    pub frame_slots_leased: u32,
    pub snapshot_live_entries: usize,
    pub import_cache_live_entries: usize,
}

impl LiveResourceSampler {
    fn new(started: Instant, continuous: bool) -> Self {
        Self {
            started,
            schedule: crate::resource_sampling::ResourceSamplingSchedule::new(started, continuous),
        }
    }

    /// Whether a sample is due, which the caller checks before gathering one.
    ///
    /// Asking first keeps the gauge reads out of the hot path: they walk a map
    /// and read `/proc`, and doing that per loop iteration would make the
    /// measurement part of what it measures.
    fn is_due(&self, now: Instant) -> bool {
        self.schedule.is_due(now)
    }

    /// Record one sample, and say when the next is due.
    fn record(&mut self, now: Instant, sample: LiveResourceSample) {
        let Some(sequence) = self.schedule.advance(now) else { return; };
        let uptime_msec = u64::try_from(now.duration_since(self.started).as_millis())
            .unwrap_or(u64::MAX);
        crate::session_println!(
            "sophia_live_resource_sample schema=1 seq={} uptime_msec={uptime_msec} rss_kib={} cpu_registry_buffers={} cpu_registry_bytes={} cpu_cow_splits={} frame_slots_leased={} snapshot_live_entries={} import_cache_live_entries={}",
            sequence,
            resident_kib().unwrap_or(0),
            sample.cpu_registry_buffers,
            sample.cpu_registry_bytes,
            sample.cpu_cow_splits,
            sample.frame_slots_leased,
            sample.snapshot_live_entries,
            sample.import_cache_live_entries,
        );
    }

    /// The population this session produced, reported without a verdict.
    ///
    /// `saturated=true` means sampling stopped before the session did, so the
    /// samples describe only the bounded prefix rather than the whole run. A
    /// verifier that reasoned over them as if they covered the session would
    /// be reading a truncated population as a complete one.
    fn report(&self) {
        crate::session_println!(
            "sophia_live_resource_steady_state schema=1 status=complete samples={} saturated={} interval_msec={}",
            self.schedule.samples(),
            self.schedule.saturated(),
            RESOURCE_SAMPLE_INTERVAL.as_millis(),
        );
    }
}

/// The process's resident set size, in kibibytes.
///
/// Read from `/proc/self/status` rather than tracked, because the figure that
/// matters includes every allocation the process made, not only the ones Sophia
/// counts. `None` where the file cannot be read or does not carry the field,
/// which the record reports as zero: a missing reading is not a small one, and
/// the verifier's growth rule treats a flat zero series as nothing to compare.
fn resident_kib() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status.lines().find_map(|line| {
        let rest = line.strip_prefix("VmRSS:")?;
        rest.split_whitespace().next()?.parse().ok()
    })
}
