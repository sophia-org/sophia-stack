use sophia_backend_live::{
    LiveProductionAuthorityBatch, LiveProductionNativeScanout, LiveProductionScanoutContent,
};
use sophia_protocol::{BufferSource, CommittedSurfaceState, SurfaceId, TransactionId};
use std::collections::{BTreeMap, hash_map::RandomState};
use std::hash::BuildHasher;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ContentIdentity {
    frame: u64,
    transaction: Option<u64>,
    kind: &'static str,
}
impl ContentIdentity {
    fn from_content(content: LiveProductionScanoutContent) -> Self {
        let (frame, transaction, kind) = match content {
            LiveProductionScanoutContent::Cpu { frame, .. } => (frame, None, "cpu"),
            LiveProductionScanoutContent::MixedPresent {
                frame, transaction, ..
            } => (frame, Some(transaction.raw()), "mixed_present"),
            LiveProductionScanoutContent::RetainedMixed { frame, .. } => {
                (frame, None, "retained_mixed")
            }
            LiveProductionScanoutContent::HeadComposition { frame, .. } => {
                (frame, None, "head_composition")
            }
        };
        Self {
            frame: frame.raw(),
            transaction,
            kind,
        }
    }
}
impl std::fmt::Display for ContentIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:", self.kind, self.frame)?;
        match self.transaction {
            Some(transaction) => write!(f, "{transaction}"),
            None => f.write_str("none"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HeadSnapshot {
    generation: u64,
    enabled: bool,
    pending: Option<ContentIdentity>,
    rendering: Option<ContentIdentity>,
    submitted: Option<ContentIdentity>,
    presented: Option<ContentIdentity>,
    submissions: usize,
    retirements: usize,
}

// Snapshots describe owners observed now, not an inferred history between polls.
// Surface pseudonyms are salted per tracker; no XID, pixel or checksum is logged.
pub(super) struct VisualProgress {
    enabled: bool,
    surfaces: RandomState,
    committed: BTreeMap<SurfaceId, u64>,
    heads: BTreeMap<(u64, u64), HeadSnapshot>,
}
impl VisualProgress {
    pub(super) fn new() -> Self {
        let enabled = enabled_value(std::env::var("SOPHIA_LIVE_VISUAL_PROGRESS").ok().as_deref());
        if enabled {
            crate::session_println!("sophia_live_visual_progress schema=1 status=enabled");
        }
        Self {
            enabled,
            surfaces: RandomState::new(),
            committed: BTreeMap::new(),
            heads: BTreeMap::new(),
        }
    }

    pub(super) fn observe_intake(&self, batch: &LiveProductionAuthorityBatch) {
        if !self.enabled {
            return;
        }
        for group in &batch.groups {
            for update in &group.cpu_buffer_updates {
                self.intake(update.identity.transaction, update.identity.surface, "cpu");
            }
            for submission in &group.present_submissions {
                self.intake(submission.transaction, submission.surface, "dma_present");
            }
            for submission in &group.software_present_submissions {
                self.intake(
                    submission.transaction,
                    submission.surface,
                    "software_present",
                );
            }
        }
    }

    fn intake(&self, transaction: TransactionId, surface: SurfaceId, source: &'static str) {
        let token = self.surfaces.hash_one(surface);
        crate::session_println!(
            "sophia_live_visual_progress schema=1 status=content stage=offered transaction={} surface_token={token:016x} source={source}",
            transaction.raw()
        );
    }

    pub(super) fn observe_committed(&mut self, surfaces: &[CommittedSurfaceState]) {
        if !self.enabled {
            return;
        }
        self.committed
            .retain(|surface, _| surfaces.iter().any(|s| s.surface == *surface));
        for state in surfaces {
            if self
                .committed
                .insert(state.surface, state.committed_generation)
                == Some(state.committed_generation)
            {
                continue;
            }
            let token = self.surfaces.hash_one(state.surface);
            let source = match state.buffer() {
                BufferSource::None => "none",
                BufferSource::XPixmap { .. } => "x_pixmap",
                BufferSource::CpuBuffer { .. } => "cpu",
                BufferSource::DmaBuf { .. } => "dma_buf",
            };
            crate::session_println!(
                "sophia_live_visual_progress schema=1 status=committed_snapshot surface_token={token:016x} generation={} source={source}",
                state.committed_generation,
            );
        }
    }

    pub(super) fn observe_native(&mut self, native: &LiveProductionNativeScanout) {
        if !self.enabled {
            return;
        }
        self.heads.retain(|&(output, head), _| {
            native
                .heads
                .iter()
                .any(|current| current.output.id.raw() == output && current.head.raw() == head)
        });
        for head in &native.heads {
            let key = (head.output.id.raw(), head.head.raw());
            let next = HeadSnapshot {
                generation: head.target_generation,
                enabled: head.enabled,
                pending: head.pending_content.map(ContentIdentity::from_content),
                rendering: head.rendering_content.map(ContentIdentity::from_content),
                submitted: head.submitted_content.map(ContentIdentity::from_content),
                presented: head.presented_content.map(ContentIdentity::from_content),
                submissions: head.submissions,
                retirements: head.presented_submissions,
            };
            if let Some(line) = head_line(key, self.heads.insert(key, next), next) {
                crate::session_println!("{line}");
            }
        }
    }
}

fn enabled_value(value: Option<&str>) -> bool {
    matches!(value, Some("1" | "true"))
}
fn content(value: Option<ContentIdentity>) -> String {
    value.map_or_else(|| "none".into(), |value| value.to_string())
}
fn head_line(
    key: (u64, u64),
    previous: Option<HeadSnapshot>,
    next: HeadSnapshot,
) -> Option<String> {
    if previous == Some(next) {
        return None;
    }
    let continuous = previous.filter(|old| {
        old.generation == next.generation
            && old.submissions <= next.submissions
            && old.retirements <= next.retirements
    });
    let (submitted_delta, retired_delta, missed_count) = continuous.map_or((0, 0, 0), |old| {
        let submitted = next.submissions - old.submissions;
        let retired = next.retirements - old.retirements;
        (
            submitted,
            retired,
            submitted
                .saturating_sub(1)
                .saturating_add(retired.saturating_sub(1)),
        )
    });
    Some(format!(
        "sophia_live_visual_progress schema=1 status=head_snapshot output={} head={} target_generation={} enabled={} baseline={} pending={} rendering={} submitted={} presented={} submissions={} retirements={} submissions_delta={submitted_delta} retirements_delta={retired_delta} missed_count={missed_count}",
        key.0,
        key.1,
        next.generation,
        next.enabled,
        continuous.is_none(),
        content(next.pending),
        content(next.rendering),
        content(next.submitted),
        content(next.presented),
        next.submissions,
        next.retirements
    ))
}

#[path = "../../tests/support/visual_progress.rs"]
mod tests;
