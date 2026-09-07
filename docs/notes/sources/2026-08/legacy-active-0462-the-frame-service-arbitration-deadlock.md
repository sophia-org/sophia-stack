---
id: legacy-active-0462
date: 2026-08-17
recorded_date: 2026-08-17
date_basis: first-heading-commit
date_commit: 40491c23daccd3068f90a59162736c8c3fe2c540
committed_at: 2026-08-17T17:39:13-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# The frame-service arbitration deadlock

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13977–14041. The heading has no date. Its first recorded addition is commit
`40491c23daccd3068f90a59162736c8c3fe2c540` (2026-08-17T17:39:13-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The mixed-output gate stalled at the topology quiescence wait with
`runtime_blocker=runnable_queued_present` and `kms_submits=0` for the full two
seconds. Four guesses at the mechanism were wrong in a row; every one of them
was corrected by instrumentation rather than by reasoning, which is the first
thing worth recording. The decomposed timeout line, then the per-head report,
then the multi-clause runtime report each narrowed it, and only the last one
was specific enough to trace.

The cause was a mutual suppression between a reducer and the handler it feeds.
`drive_gpu_presentation` refused a queued present while the primary output owed
a native frame, and refused it silently -- no pop, no rejection, no log. The
frame-service reducer, meanwhile, withheld `SubmitPendingFrame` from the
primary whenever a present was queued, and that effect is the only thing that
drains such a frame. The pending frame blocked the present; the present blocked
the drain. Nothing re-defers a runnable present, so `has_runnable_queued`
stayed true forever and no submission ever reached the kernel.

The gap existed because the two halves measured different things. The reducer
admitted a present on `native_phase == Idle`, derived from scanout submission
and cleanup alone; the handler refused on `pending_frame`, which additionally
counts queued mirror successors, prepared scanouts, and exporter frames that
have not yet entered a renderer worker. Every state in between satisfied the
reducer and failed the handler.

An existing test asserted the defect as correct. `frame_service.rs` contained
`queued_presentation_reserves_primary_but_not_secondary`, pinning the exact
reservation that caused the stall. This is the second time in this work that a
test encoded the bug it should have caught -- the first was the topology
quarantine, where a test drove the hotplug writer through a policy-change
quarantine. Both tests were named after the mechanism they exercised rather
than the property they protected, which is what let them look correct.

The fix gives the reducer the ordering and makes each handler refusal
unreachable by construction: a present is emitted only when the primary owes
nothing, the primary's drain is never withheld, and software staging is emitted
only when every output is idle. That last one closed a latent session fatal
rather than a stall: `run_native_pending_output` errors outright when staging
is blocked, so the reducer had been able to emit an effect that killed the
session. `EmittedEffectsAreExecutable` in the new
`validation/tla/FrameServiceArbitration.tla` states each handler precondition
as a consequence of its reducer gate, which is what keeps the two from drifting
apart again. Restoring the old reservation in that model violates
`PresentSettles` at exactly the production state.

Two further things surfaced while tracing. A waiting software present makes
every output report a pending frame and suppresses all present dispatch, so the
same wedge exists in the software half; splitting `software_frame_waiting` out
of the per-output flag makes the reducer able to tell "this output owes a
frame" from "something global is pending". And bound software presents settle
only on a real page flip of the head frames they were lowered onto, while
topology installation discards exactly those frames -- so an installed topology
could strand a binding forever, leaving the runtime permanently non-quiescent
and the client permanently without feedback. Both terminals now settle them.

The escalation added alongside is the `BoundedDeferral` disposition from the
capacity vocabulary, applied to a wait that had none. The wait blocks on owners
that can only advance while it waits, so an expiry that merely reports a stall
has not tried to clear it; the first expiry now skips what is runnable and what
is waiting, settling each as `Skipped` so no client is left expecting feedback,
and grants one more window. A second expiry rejects the candidate as before.
The displayed topology is never given up, which is what distinguishes this from
the suspend path it was factored out of.

<!-- END IMPORTED BODY -->
