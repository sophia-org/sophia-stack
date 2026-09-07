---
id: legacy-active-0284
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-18: Production X Uses One Native Service Poll

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8936–8953. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`LiveProductionVisualRuntime::service_native` now owns the asynchronous native service order:
page-flip retirement and cleanup first, eligible queued Present work second, and pending native
frames last. It returns one reduced report with the optional backend tick and phase observations.
The production X event loop no longer inspects pending exporter frames or separately invokes
retirement, GPU scheduling, and native idle submission. Wayland retains its specialized
maintenance service because it correlates client buffer release to its own submission counters.

The full offline all-feature suite passes. The rebuilt X13 QEMU image passed strict two-xterm in
6,986 ms with 117 of 117 transactions, 40 submissions, 38 retirements, and zero cleanup debt.
Resize-enabled classic and confined GTK passed exact input, pointer selection, committed resize
redraw, normal exit, `first_error=none`, native presentation, and clean teardown. The remaining
Milestone 6 exit gap is exact rather than structural: GPU Present prepare\/retire sequencing still
lives in backend visual control and must enter `sophia-engine::runtime_driver` before that module
is the only production visual coordinator.


<!-- END IMPORTED BODY -->
