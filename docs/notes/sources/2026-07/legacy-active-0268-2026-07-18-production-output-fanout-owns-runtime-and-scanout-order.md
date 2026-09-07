---
id: legacy-active-0268
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-18: Production Output Fanout Owns Runtime And Scanout Order

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8673–8693. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Engine now defines a protocol-neutral `ProductionOutputRuntimeAdapter`, and backend-live
provides its bounded callback implementation. The session coordinator projects its single
committed snapshot and enumerates outputs. Steady CPU ticks, committed-snapshot ticks, GPU
Present submission, native idle submission, page-flip retirement and cleanup, and displayed
buffer teardown all enter through that fanout. During the audit, retired Present completion
was also corrected to use the session coordinator directly and to project its result to
every output; it no longer mutates the former primary output coordinator.

Engine and backend regressions prove one snapshot reaches every output and projection plus
runtime invocation remain one adapter callback. The full offline all-feature suite passes.
The rebuilt X13 QEMU image passed strict two-xterm in 6,897 ms with 120 of 120 transactions,
7 ms input presentation, 40 submissions, 38 retirements, and zero phase or cleanup debt.
Classic and confined GTK accepted exact physical text and pointer selection, exited normally
with `first_error=none`, and cleanly retired both outputs. Emergency recovery flushed all
five routed chord deliveries and shut down cleanly in 189 ms. The concrete closures still
live beside `PersistentNativeScanout` in `live_session.rs`; extracting that implementation
into backend-live is the remaining runtime/scanout ownership step.


<!-- END IMPORTED BODY -->
