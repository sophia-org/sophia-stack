---
id: legacy-active-0283
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-18: Concrete Visual Control Leaves The CLI

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8914–8935. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Backend-live now owns `LiveProductionVisualRuntime`, including the production coordinator,
per-output runtime set, renderer transaction projection, CPU and GPU cycle entry points, Present
resource admission and scheduling, native submission and retirement, cleanup, and reduced
feedback routing. The `PersistentBackendRuntime` type and roughly 950 lines of implementation
are gone from `live_session.rs`. CLI code constructs the runtime, translates X batches, supervises
clients, records proof evidence, and requests high-level service; it no longer defines visual
state-machine behavior. A reduced diagnostics snapshot replaces direct CLI access to Present
resource and scheduler internals.

The full offline all-feature suite passes. The rebuilt X13 QEMU image passed strict two-xterm in
6,916 ms with 120 of 120 transactions, 40 submissions, 38 retirements, exact keyboard and pointer
proofs, and zero cleanup debt. Resize-enabled classic and confined GTK passed exact text, pointer
selection, committed 640x360 CPU\/SHM redraw, normal exit, `first_error=none`, native presentation,
and clean teardown. Emergency recovery flushed all five routed chord events and completed in 167
ms. The guarded native mixed diagnostic exported one CPU and one DMA-BUF layer with zero live
resources. Milestone 6 ownership, fail-closed, and retained-gate migration items are complete.
The remaining exit item is moving asynchronous GPU-service and retirement trigger timing from
the CLI event loop into `sophia-engine::runtime_driver`.


<!-- END IMPORTED BODY -->
