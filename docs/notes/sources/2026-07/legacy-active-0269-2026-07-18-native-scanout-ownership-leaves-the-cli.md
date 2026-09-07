---
id: legacy-active-0269
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "architecture"]
---
# 2026-07-18: Native Scanout Ownership Leaves The CLI

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8694–8714. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The concrete native output owner is now backend-live's `LiveProductionNativeScanout`.
It owns real atomic card/session groups, per-head GBM exporters, native callback queues,
page-flip correlation, submission and retirement counters, mixed-frame export, and the
production composed-frame record. The implementation is feature-gated in its own backend
domain module. `live_session.rs` no longer defines or directly names real atomic sessions,
GBM exporter types, or the page-flip tracker, and shrank by roughly 570 lines. The existing
opt-in native lifecycle diagnostic is preserved verbatim at the backend boundary.

Default and all-feature backend builds pass, as does the full offline all-feature suite.
The guarded X13 native EGL/vkcube diagnostic exported one CPU and one DMA-BUF layer with
zero live resources afterward. The rebuilt QEMU image passed strict two-xterm in 6,973 ms
with 120 of 120 transactions, 3 ms input presentation, 38 submissions, 36 retirements, and
zero phase or cleanup debt. Classic and confined GTK passed exact physical text/pointer
selection, normal exit, `first_error=none`, and clean two-output retirement. Emergency
recovery flushed all five routed chord deliveries and shut down cleanly in 164 ms. The
remaining Milestone 6 ownership gap is `PersistentBackendRuntime` plus the CPU composition
callbacks still implemented in the CLI.


<!-- END IMPORTED BODY -->
