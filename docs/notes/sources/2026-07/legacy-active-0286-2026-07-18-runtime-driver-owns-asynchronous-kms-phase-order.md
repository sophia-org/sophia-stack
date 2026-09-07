---
id: legacy-active-0286
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-18: Runtime Driver Owns Asynchronous KMS Phase Order

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8970–8989. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`ProductionAsyncServiceCoordinator` consumes reduced in-flight, cleanup, queued-Present, and
pending-frame observations and requests at most the ordered Retire, SchedulePresent, and
SubmitPendingFrame phases. Backend-live executes those requests and feeds updated observations
back after each action; it no longer encodes asynchronous phase order. A runtime-driver regression
proves dynamic behavior: retirement unlocks Present scheduling, and a resulting in-flight frame
suppresses pending submission in the same service pass. Together with coordinator-owned CPU
cycles and full-state Present prepare\/retire gates, this closes the last Milestone 6 architecture
checkbox.

The full offline all-feature suite passes. The rebuilt X13 QEMU image passed strict two-xterm in
6,973 ms with 117 of 117 transactions, 40 submissions, 38 retirements, and zero cleanup debt.
Resize-enabled classic and confined GTK passed exact input, pointer selection, committed resize
redraw, normal exit, `first_error=none`, native presentation, and clean teardown. The guarded
native mixed path exported one CPU and one DMA-BUF layer with zero live resources. Milestone 6
implementation is complete; its overall promotion remains coupled to the operator-driven paired
physical X13 GTK evidence still required by Milestone 5.


<!-- END IMPORTED BODY -->
