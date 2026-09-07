---
id: legacy-active-0270
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-18: CPU Composition State Moves Into Renderer-Live

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8715–8737. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Renderer-live now owns `LiveProductionCpuScene`: the CPU buffer registry, immutable update
admission, committed-surface handle retention, focus-aware stacking, cursor composition,
focused-surface visual-detail inspection, composition evidence, and per-output composed
frame creation. The X boundary only converts authority SHM records into neutral
`LiveCpuBufferUpdate` values and observes reduced reports. Backend-live also owns composed
frame records and the high-level page-flip cleanup/retry and displayed-output release
operations, so CLI callbacks no longer invoke low-level native cleanup APIs or lifecycle
logging. The protocol-neutral authority, renderer, scanout, and feedback adapter roadmap
item is complete.

The full offline all-feature suite passes. On the rebuilt X13 QEMU image, strict two-xterm
completed in 6,995 ms with 117 of 117 transactions, 7 ms input presentation, 40 submissions,
38 retirements, and zero phase or cleanup debt. The guarded native mixed diagnostic exported
one CPU and one DMA-BUF layer with zero live resources. Classic and confined GTK accepted
exact physical text and pointer selection, exited normally with `first_error=none`, and
retired both outputs cleanly. Emergency recovery flushed all five routed chord deliveries
and shut down cleanly in 162 ms. `PersistentBackendRuntime` remains the last large CLI
visual-control wrapper; its X routing and proof observations must be separated from the
protocol-neutral production state machine before the Milestone 6 ownership item can close.


<!-- END IMPORTED BODY -->
