---
id: legacy-active-0264
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "tooling"]
---
# 2026-07-18: Present Complete And Idle Follow Backend Resource Retirement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8594–8617. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`LiveProductionPresentFeedbackCoordinator` now owns the presentation resource
session and produces paired, protocol-neutral Complete/Idle outcomes only after
the matching page flip or controlled rejection retires the live presentation.
Missing or already-retired transactions return an explicit error and emit no
feedback. The live CLI translates the reduced Flip/Skip mode to X wire events
and observes counters, but no longer orders resource retirement, Complete, Idle,
or idle-fence accounting independently. Diagnostic abort still tears resources
down without falsely producing client feedback.

Tests prove a client-released DMA-BUF source and presentation retire before the
Flip/Idle outcome, a second completion fails closed, and an unknown Skip emits
nothing. The full offline all-feature suite passes. The guarded X13 native EGL
vkcube diagnostic passed with one CPU layer, one DMA-BUF layer, and zero live
sources, fences, or transactions. A rebuilt strict two-xterm QEMU image completed
300 ticks in 6,919 ms with 123 of 123 transactions applied, 7 ms input
presentation, 40 submissions, 38 retirements, and zero phase or cleanup debt.
The full real-KMS Milestone 4 proof could not run unattended because X13 sudo
requested a password before any modeset; it remains a later interactive gate.
Prepared Engine commit application and runtime/scanout invocation are still in
`live_session.rs` and remain the next production ownership migration.


<!-- END IMPORTED BODY -->
