use super::{
    LiveProductionPresentLayoutState, LiveProductionPresentScheduler, SurfaceId,
    SurfaceTransactionKey,
};
use sophia_protocol::Rect;
use std::time::{Duration, Instant};

/// Why a surface's first candidate is parked. Release has to be evaluated
/// against the condition that parked it: the first variant is entered
/// *because* the surface is absent from the presentation order, so requiring
/// its presence there to leave again is a wait nothing can end.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveProductionFirstVisibilityReason {
    /// Absent from the last projection applied at the Engine boundary.
    OutsidePresentationOrder,
    /// No output it routes to could carry the candidate.
    NoApplicableOutput,
    /// Its image did not enter a captured head frame.
    OutsideHeadFrames,
}

/// How long a first candidate may wait to become visible. A settling
/// animation finishes far inside this. Admission allows two four-second
/// attempts, so expiring well before that is what turns a silent withdrawal
/// into a fast, reported failure.
const FIRST_VISIBILITY_BUDGET: Duration = Duration::from_millis(2_000);

impl LiveProductionPresentScheduler {
    /// Keep an unpresented surface's exact first candidate until it can enter
    /// a physical frame. Skipping it would strand admission waiting for that
    /// candidate's retirement, with every successor still quarantined.
    ///
    /// The wait is bounded. Nothing guarantees the parking condition ever
    /// clears, and a candidate that waits forever is withdrawn by admission
    /// with no record of why.
    pub fn defer_first_visibility(
        &mut self,
        candidate: SurfaceTransactionKey,
        reason: LiveProductionFirstVisibilityReason,
        now: Instant,
    ) -> bool {
        let Some(queued) = self.queued.front_mut() else {
            return false;
        };
        if queued.candidate.key() != candidate || !queued.runnable() {
            return false;
        }
        queued.layout_state = LiveProductionPresentLayoutState::AwaitingFirstVisibility {
            reason,
            deadline: now + FIRST_VISIBILITY_BUDGET,
        };
        true
    }

    pub fn awaiting_first_visibility(
        &self,
    ) -> impl Iterator<Item = (SurfaceId, Rect, LiveProductionFirstVisibilityReason)> + '_ {
        self.queued.iter().filter_map(|queued| {
            let LiveProductionPresentLayoutState::AwaitingFirstVisibility { reason, .. } =
                queued.layout_state
            else {
                return None;
            };
            Some((queued.surface, queued.candidate.target_geometry, reason))
        })
    }

    pub fn release_first_visibility(&mut self, visible: &[SurfaceId]) -> usize {
        let mut released = 0;
        for queued in &mut self.queued {
            if matches!(
                queued.layout_state,
                LiveProductionPresentLayoutState::AwaitingFirstVisibility { .. }
            ) && visible.contains(&queued.surface)
            {
                queued.layout_state = LiveProductionPresentLayoutState::Runnable;
                released += 1;
            }
        }
        self.observe_queue_depth();
        released
    }

    /// Return parked candidates whose budget has run out to the runnable
    /// queue, marked as having spent it.
    ///
    /// They are not rejected here. Releasing them lets the ordinary Present
    /// path reach them again, and the exhausted mark makes that pass take the
    /// rejection every non-first candidate already takes. Reusing that route
    /// is the point: it already releases the content owner, frees the groups
    /// waiting behind it on that surface, and sends the client Complete with
    /// a skipped mode followed by Idle, so bounding the wait needs no second
    /// way to unwind a candidate.
    ///
    /// It does not settle an admission candidate that has already been
    /// selected and is awaiting retirement; an ordinary skip never did. What
    /// this bounds is the candidate that arrives *before* admission selects
    /// anything -- a client that presents its first frame before mapping its
    /// window -- where there is no such debt to settle yet.
    ///
    /// Releasing the buffer is also not the same as the client drawing again.
    /// These notifications end its wait; whether it then redraws is its own
    /// behaviour, and only a real client can demonstrate it.
    pub fn expire_first_visibility(
        &mut self,
        now: Instant,
    ) -> Vec<(SurfaceId, LiveProductionFirstVisibilityReason)> {
        let mut expired = Vec::new();
        for queued in &mut self.queued {
            let LiveProductionPresentLayoutState::AwaitingFirstVisibility { reason, deadline } =
                queued.layout_state
            else {
                continue;
            };
            if now < deadline {
                continue;
            }
            queued.layout_state = LiveProductionPresentLayoutState::Runnable;
            queued.first_visibility_exhausted = true;
            expired.push((queued.surface, reason));
        }
        if !expired.is_empty() {
            self.observe_queue_depth();
        }
        expired
    }

    /// Remove a queued candidate by exact DMA-BUF identity, yielding its
    /// transaction so the caller can settle it the ordinary way.
    ///
    /// Only a queued candidate. One already in flight has been selected and
    /// committed to a frame, and taking it back here would settle a present
    /// the kernel still owns. Matching includes the buffer, because a
    /// transaction and surface pair can name more than one source and a
    /// backing snapshot must not be mistaken for the client's Present.
    pub fn remove_queued_dma_candidate(
        &mut self,
        key: sophia_protocol::DmaBufPresentKey,
    ) -> Option<sophia_protocol::TransactionId> {
        let position = self.queued.iter().position(|queued| {
            let candidate = queued.candidate.key();
            candidate.transaction == key.transaction
                && candidate.surface == key.surface
                && matches!(
                    candidate.target_buffer,
                    sophia_protocol::BufferSource::DmaBuf { handle, .. }
                        if handle == key.buffer.raw()
                )
        })?;
        let queued = self.queued.remove(position)?;
        self.observe_queue_depth();
        Some(queued.submission.transaction)
    }

    /// Whether the candidate at the head has already spent its
    /// first-visibility budget, so the Present path knows not to park it
    /// again.
    pub fn front_first_visibility_exhausted(&self) -> bool {
        self.queued
            .front()
            .is_some_and(|queued| queued.first_visibility_exhausted)
    }
}
