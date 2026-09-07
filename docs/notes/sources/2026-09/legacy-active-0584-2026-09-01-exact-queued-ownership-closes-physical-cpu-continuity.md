---
id: legacy-active-0584
date: 2026-09-01
recorded_date: 2026-09-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "architecture"]
---
# 2026-09-01: exact queued ownership closes physical CPU continuity

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18443–18469. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The single-attempt terminal gate on signed commit
`b9f0735ae3de0ab3f963fe19d6d117e0cbe6d403` passed both machine and
operator visual verdicts. Retained run `20260902T002500Z` records
`terminal-gate-result schema=2 status=passed`, `attempts=1`,
`retry-policy=disabled`, complete kernel deltas, and no machine or visual
failure. Quiescence completed in 36 ms with `authority_pending=0`,
`cpu_pending=0`, and `native_pending=false`; protocol and cleanup health were
clean.

The schema-6 performance record accounted all 7,116 accepted post-startup CPU
updates: 1,190 were presented, 5,926 were superseded, none were lifecycle-
superseded, and none remained pending. The run
retired 2,390 native frames, including 1,191 changed primary retirements, and
bound 3,192 exact native targets. Maximum source gap was 16.586 ms, maximum
display gap was 18.825 ms, and maximum accepted-update-to-retirement latency
was 31.737 ms against a 34.334 ms refresh-relative deadline.

The stable-backing path performed 7,152 patch updates and one full CPU
replacement, with one resident buffer at peak. The visually changing
deterministic probe made stale presentation observable throughout the run.
Together with the positive and mutation-controlled formal models and the
canonical `cargo xtask check` result on the same candidate, this closes
CP-14.1. It does not close the diagnostic same-hardware comparison or the
fresh two-hour current soak required for the remainder of Milestone 14.

<!-- END IMPORTED BODY -->
