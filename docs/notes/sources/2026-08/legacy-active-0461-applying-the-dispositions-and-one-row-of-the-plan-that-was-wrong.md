---
id: legacy-active-0461
date: 2026-08-17
recorded_date: 2026-08-17
date_basis: first-heading-commit
date_commit: 9d94ed49985a9709e2974d1215eb85555abc93dc
committed_at: 2026-08-17T13:24:05-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# Applying the dispositions, and one row of the plan that was wrong

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13924–13976. The heading has no date. Its first recorded addition is commit
`9d94ed49985a9709e2974d1215eb85555abc93dc` (2026-08-17T13:24:05-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Seven of the ten converted sites went as designed. The libinput acquisition
worker now defers for 50ms before abandoning a batch, and counts what it
abandons; the seven routed-input ingress sites close the endpoint epoch rather
than the session; the XKB worker returns typed errors and waits on a deadline
instead of panicking inside its thread; the present-cadence sampler slides its
window instead of latching an overflow flag; a lost key-timing sidecar is
consumed rather than fatal; and a full pressed-key ledger closes the epoch and
flushes what it holds.

Two of those needed a distinction the plan did not draw. At the acquisition
worker, teardown and a departed recipient are not degradations of a running
session, and a site that counts them as loss reports it on every clean exit --
so the shared batch driver names `Cancelled` and `RecipientGone` separately from
`Saturated`, and only the last carries a discard count. At the timing sidecar,
an *absent* measurement and a *mismatched* one are different failures: absence
is capacity pressure and is consumed, but a sidecar that disagrees with its
event means the serial-to-timing association is wrong, which would make every
latency number untrustworthy. That one stays fatal. Softening it would have been
the same mistake as relaxing the layout-recovery invariant earlier in this work.

The routed-input conversion turned out not to need the depth-tracked reserve the
model suggested. `advance_control_epoch` is a compare-exchange on an atomic
rather than a queued record, so the close is deliverable no matter how full the
queue is. The epoch is therefore itself the terminating boundary, and applying
it clears active grabs and frozen input. Releases still get a short bounded wait
that ordinary input does not, because a lost release leaves a client believing a
key is held, but the reserve that the model proves is load-bearing is discharged
here by an out-of-band mechanism rather than by held-back queue slots.

One row of the plan was wrong and is dropped. It proposed making the session
runtime observation batch `RejectAndConsume` and raising its bound from 64 to
256. Reading `session/reducer.rs` first shows why that would have been a real
defect: `SessionRuntimeObservation` is not telemetry. Every observation drives a
phase transition and emits a command -- `TickStarted` yields `PollXEvents`,
`FrameRendered` yields `SubmitScanout`. Dropping one would skip a frame render or
a scanout submission while reporting success, which is precisely the quiet
degradation this whole change exists to prevent. The existing terminal error is
correct, and the real fix was already in place on the producer side, where
`AUTHORITY_MERGE_TRANSACTION_LIMIT` bounds a merged run so the batch cannot
overflow. Raising the bound to 256 would also have loosened that throttle and
risked the long owner turn its comment warns about. The lesson repeats: check
whether a bounded resource carries state or carries diagnostics before choosing
its disposition, because the two look identical at the call site.

The pressed-key ledger change interacts with the ingress change in a way worth
recording. Because a dropped release now leaves its key recorded as pressed --
truthfully, since the client never saw the release -- pressure on the ledger
went up, not down. That is why the epoch close flushes every held key rather
than only reporting: without the flush the ledger would stay full and refuse
every later press for the same reason forever.

<!-- END IMPORTED BODY -->
