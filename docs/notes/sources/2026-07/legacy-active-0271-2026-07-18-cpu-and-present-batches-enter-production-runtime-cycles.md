---
id: legacy-active-0271
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-18: CPU And Present Batches Enter Production Runtime Cycles

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8738–8759. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

CPU authority batches now enter `ProductionSessionCoordinator::run_cycle`, which owns the
commit, immutable-snapshot composition, output submission, retirement poll, and feedback
order. The X session loop supplies translated authority updates and observes only the reduced
submission report. Present batches now likewise cross a single runtime-owned GPU production
entry point: CPU-background composition, per-output frame creation, native initialization,
and Present scheduling no longer occur in the outer CLI loop. Present remains asynchronous;
its prepared Engine state is committed only after the matching page flip through the existing
coordinator retirement gate.

The full offline all-feature suite passes. The rebuilt X13 target passed the guarded mixed
CPU-plus-DMA-BUF diagnostic with zero live sources, fences, or transactions. Its QEMU image
passed strict two-xterm in 6,966 ms with 117 of 117 authority transactions, 8 ms input
presentation, 40 submissions, 38 retirements, and zero cleanup debt. Classic and confined
GTK accepted exact physical text and pointer selection, exited normally with
`first_error=none`, and retired both outputs cleanly. Emergency recovery flushed all five
routed chord deliveries and shut down cleanly in 161 ms. The remaining Milestone 6 boundary
is structural: extract the runtime-owned visual control object from the X router, proof
counters, and process supervision, then delete the legacy committed-snapshot entry points.


<!-- END IMPORTED BODY -->
