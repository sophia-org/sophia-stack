---
id: legacy-active-0582
date: 2026-09-01
recorded_date: 2026-09-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-09-01: page-flip and out-fence completion must share one clock domain

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18327–18385. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The terminal gate on signed commit
`168cb9a2dc82fed0d67e1bc20195275e88addd19` passed operator visual
confirmation. Its machine record failed only at the final native pacing
invariant: 1,054 overlap rejections and 1,055 phase rejections. The same archive
proved the preceding lifecycle repair. X Authority drained in 21 ms,
quiescence completed in 25 ms with `authority_pending=0 cpu_pending=0
native_pending=false`, all 6,948 accepted post-startup CPU updates were
accounted, and the primary output retired 1,191 changed frames with a 17.905 ms
maximum display gap.

The counter delta and source populations identify one causal transition rather
than duplicate telemetry. Output 2 accepted 141 real kernel page-flip events,
then 1,055 authoritative out-fence completions. Kernel page-flip UST is
absolute `CLOCK_MONOTONIC`; the single-output fallback instead used
`presentation_started.elapsed()`, relative to session start. The first
out-fence timestamp therefore moved backward. `OutputPresentationRegistry`
rejected it as `NonMonotonicTimestamp`, and
`LiveProductionPageFlipTracker` returned before retiring and removing its
pending frame. Every later KMS submission and physical completion remained
healthy, while that one orphan generated exactly the next 1,054 overlap
rejections and all 1,055 phase rejections.

Single and mirror completion paths now share one reducer. Page-flip events use
their matching kernel UST; missing kernel evidence and authoritative out-fences
use a fresh absolute monotonic UST. A stale timestamp keyed to an out-fence
completion is consumed but cannot override the authoritative source. The
tracker accepts only UST and derives milliseconds internally, preventing its
two timing inputs from naming different epochs.

Ownership and cadence validation are also separated. Once the physical owner
has accepted a callback, the tracker retires and removes that exact pending
frame even if monotonicity validation rejects the timing sample. The original
error is still tallied, logged with its source, serial, and UST, and fails the
session; it can no longer strand cleanup or create an unbounded diagnostic
cascade. Integration regressions prove kernel-to-out-fence-to-missing-kernel
clock continuity, stale timestamp refusal, exact retirement, and a successful
successor submit after deliberately invalid cadence feedback.

`PageFlipPresentationTracker.tla` models KMS and tracker ownership across both
completion sources. The positive model passes 28 generated / 12 distinct states
to depth 7. Its mixed-clock control physically retires an out-fence while
retaining the tracker owner and must violate
`PhysicalTrackerOwnerAgreement`. The complete pinned corpus passes, including
the pre-existing exhaustive `TargetResolvedInput` model at 28,820,677
generated / 5,518,840 distinct states to depth 20.

The operator's separate legibility finding was in the workload, not the
renderer: `seq 1 1` printed the literal `1` every iteration. The probe now
emits deterministic full-period 16-bit pseudo-random lines containing ten
zero-padded numbers. It preserves the one-line/16-ms cadence and completed
iteration ledger. Offline coverage fixes the first lines, requires variation
without per-iteration reseeding, and retains geometry, backpressure, timeout,
and orphan-cleanup contracts.

The complete `cargo xtask check` production gate passes. One signed,
single-attempt physical terminal gate remains; both its machine and visual
verdicts must pass before CP-14.1 closes.
<!-- END IMPORTED BODY -->
