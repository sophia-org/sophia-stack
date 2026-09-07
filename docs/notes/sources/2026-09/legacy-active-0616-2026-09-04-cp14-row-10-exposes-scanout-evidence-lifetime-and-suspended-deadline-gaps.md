---
id: legacy-active-0616
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-09-04: CP14 row 10 exposes scanout evidence lifetime and suspended deadline gaps

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19565–19613. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The preparation note above is historical. Run `cp14-schema4-251d9acd` now has
nine sealed Kitty rows: three repetitions each for Sophia, XLibre+xmonad, and
niri. All 74 run/attempt checksum entries match. The next row, Sophia
`firefox-local` repetition 1, remains under
`incoming/10-sophia-firefox-local-1.partial`; typed `status` correctly refuses
progress while that capture is unresolved.

Row 10 completed a 60,048 ms measurement with 60 focused, visible, exclusively
owned DP-1 samples and 3,600 consecutive kernel frame sequences. The session log
contains 3,602 retired Present records. After measurement, the operator switched
to VT2; native suspension drained without abandoned scanouts. Resume occurred
around session uptime 102 seconds, after the configured 95-second deadline.
Quiescence then completed with zero pending authority/coordinator/CPU/native
work, but final native validation failed with `native_retirements=0`.

The code explains the mismatch. `owner_loop/lifecycle.rs` drops `native_scanout`
on VT release and creates a fresh object on resume. Its constructor initializes
submission, retirement, callback, and related counters to zero. Completion reads
that last object's counters as session evidence and requires nonzero retirements.
The resumed object exported two bootstrap frames but recorded no retirement
before immediate deadline shutdown. Session Present/CPU-progress observations
retain earlier history, producing contradictory populations in the final report.
These lifecycle/completion paths are unchanged between the pinned `251d9acd`
candidate and current `e79ca477`; the tab implementation does not fix them.

A second code issue is visible in the same path: the suspended-seat branch
continues before checking the runtime deadline. A bounded session can therefore
remain suspended past its own deadline until resume or the external watchdog.
The exact wider failure surface still needs deterministic tests, including
multiple resumes, rejected switches, topology replacement, and failure retention.

Next: retain session-owned aggregate evidence across native-owner replacement,
keep each owner's failure and drain obligations explicit, and service deadline
shutdown without requiring seat reacquisition. Test retirement before replacement,
immediate shutdown after resume, suspension across the deadline, and errors in a
retired owner. Then run a short physical Firefox canary covering VT/deadline
recovery before another full matrix. A runtime correction changes candidate
identity, so its comparison requires a fresh pinned 36-row run; these nine rows
remain historical evidence. Do not promote the partial row from its measurement
alone: its supervisor returned failure, and clean teardown is a required gate.

The volatile launcher log has been copied to
`.artifacts/diagnostics/cp14-251d9acd-firefox-row10/session.log` with a sibling
SHA-256 manifest. Original partial data and sealed rows remain untouched. No
runtime code was changed and no physical session was launched during this
analysis. CP-14.2 remains open; the optional two-hour soak does not block closure.

<!-- END IMPORTED BODY -->
