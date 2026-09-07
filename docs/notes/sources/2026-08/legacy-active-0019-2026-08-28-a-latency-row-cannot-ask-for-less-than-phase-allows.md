---
id: legacy-active-0019
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-28: a latency row cannot ask for less than phase allows

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 636–682. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Milestone 14's remaining latency row asked for physical input "within half a
  refresh period at p99". That is stricter than the one-refresh full-chain
  bound the 2026-07-31 entry already rejected on physical evidence: a randomly
  phased input can spend nearly an entire refresh waiting for the next
  synchronized flip, so a sub-refresh aggregate rewards tearing, VRR, or a
  bypass rather than a faster compositor. No currently measured stage is both
  Sophia-owned and phase-free either -- queue dwell is, and is already gated at
  1 ms; submit-to-page-flip is phase-bound between zero and a full refresh by
  construction. The row is therefore read as the 2026-07-31 stage contract
  proven at p99 rather than p95, and the sentence is superseded in place so it
  is not reinstated by a later reader.
- The row's other half was already true and unprovable. Latest-wins pending is
  a consequence of the cell being an `Option`, one renderer-worker request is a
  hard guard, one KMS submission per head is fail-closed, and
  `FrameServiceArbitration` checks `OneSubmissionInFlight` structurally. None
  of it was observable: `native_max_in_flight_ticks` measures how long a
  submission was in flight, which cannot separate one long submission from two
  overlapping ones, and the pending cell dropped superseded frames in silence.
  Concurrent depth is now sampled per output per tick and both supersession
  paths are counted, in `sophia_live_native_resources` schema 9.
- The bound asserted is per head, not per output. A mirror output holds one
  submission per head by design -- that is what `MirrorHeadPacing` authorizes --
  so asserting one per output would fail every mirror session and would be
  wrong rather than strict.
- Sampling no longer latches. The correlation returned early once
  `input_presented_ust_usec` was set, so a session yielded exactly one sample
  and a p99 needed a hundred sessions, at which point p99 is the maximum. Every
  routed press was already timestamped and only the last was used; each is now
  recorded and settled against the flip that showed it, in pre-allocated rings
  that report what they dropped. One flip can settle several presses: they
  arrived in the same frame and waited different lengths for the same photon.
  The one-shot proof path is untouched, because existing gates read its records
  and it answers a different question.
- The distribution is microseconds. Half a 60 Hz refresh is 8.3 ms, and a
  millisecond-rounded percentile cannot be compared against that honestly. The
  reporter also takes refresh from the session's own head record rather than
  the harness constant, so a 144 Hz display gets a 14 ms end-to-end budget
  instead of a 34 ms one that would pass nearly anything, and it refuses below
  two hundred samples rather than reporting a maximum under a percentile's
  name.
- No timing model was added. `FrameServiceArbitration` covers the structural
  half; a deadline model would restate in a modelled clock a bound only
  hardware can answer. That reasoning is recorded in `validation/tla/README.md`
  so the absence reads as a decision.

<!-- END IMPORTED BODY -->
