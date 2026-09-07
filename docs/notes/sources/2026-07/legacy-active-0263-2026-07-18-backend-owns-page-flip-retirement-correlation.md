---
id: legacy-active-0263
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-18: Backend Owns Page-Flip Retirement Correlation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8571–8593. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`PersistentNativeScanout` no longer owns an Engine presentation registry, a
per-head scheduled frame slot, or a reduced UST/MSC feedback queue. The
protocol-neutral `LiveProductionPageFlipTracker` in backend-live now schedules
each submitted output against a production cycle, rejects overlap, validates
monotonic page-flip sequence and timestamp evidence, retires the exact scheduled
frame, and emits a `ProductionRetirement<LiveProductionPageFlipRetirement>` only
after all those gates succeed. Per-output take and discard operations preserve
the existing CPU-frame versus Present-frame separation without exposing backend
state to the frontend.

Regressions prove no retirement exists at submit time, a matching accepted flip
retains the originating cycle and reduced UST/MSC, overlap fails closed, and
non-monotonic callbacks produce no retirement. The full offline all-feature
suite passes. On the rebuilt X13-hosted QEMU image, strict two-xterm completed
300 ticks in 6,970 ms with 120 of 120 transactions applied, 8 ms input
presentation, 40 submissions, 38 retirements, and zero phase or cleanup debt.
Confined GTK passed 57 SHM transactions, exact text and pointer evidence, normal
exit, `first_error=none`, 113 submissions, 111 retirements, and clean shutdown.
Present protocol routing still resides in the CLI and is the next ownership seam.


<!-- END IMPORTED BODY -->
