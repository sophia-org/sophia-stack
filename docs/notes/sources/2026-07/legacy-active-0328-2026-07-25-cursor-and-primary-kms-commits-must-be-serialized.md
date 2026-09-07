---
id: legacy-active-0328
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-25: Cursor And Primary KMS Commits Must Be Serialized

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10190–10226. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical cycle ran the full four-Kitty workload but failed promotion
after one early primary-plane atomic submission was rejected. The failure
followed physical pointer motion; every later submission recovered, 182 mixed
exports retained exact complete-target lifetime, native drain completed, and
the final failure count remained exactly one.

The owner ordering exposed a KMS transaction race. Native service could submit
a nonblocking primary flip, then the cursor path independently issued a
nonblocking cursor-plane commit on the same card. A successful cursor ioctl
only admitted that asynchronous commit; it did not prove the cursor update had
completed before the next owner tick submitted another primary request. The
earlier cursor-side `EBUSY` handling covered only the opposite ordering, where
a cursor update encountered an existing primary commit.

Backend-live now treats primary page-flip state as the admission boundary for
cursor work. A dirty cursor remains coalesced while any primary flip is in
flight. Once admitted, the cursor-only atomic commit is blocking, so it cannot
remain pending across the next primary submission. Completion evidence records
both primary-in-flight deferrals and maximum cursor update duration, and the
physical verifier caps the latter at 100 ms. This is the bounded daily-driver
repair. The long-term graphics path should build primary and cursor plane state
through one per-output atomic KMS transaction owner rather than preserve two
independent commit builders.

The required xmonad real-client preflight exposed an independent proof
regression before the next physical run. Its headless configuration has one
virtual output and intentionally disables native scanout, but startup readiness
waited for the native-only `OutputsPresented` event. The owner now satisfies
that output fact only when native scanout is absent; physical sessions still
require every real output's callback or synchronous modeset evidence. The
preflight verifier also correlates the injected resize transaction directly,
instead of assuming initial placement and the later 960x640 configure collapse
into one asynchronous WM response. The corrected preflight passes with matched
configure delivery, later pixels, exact synthetic input, and clean teardown.

<!-- END IMPORTED BODY -->
