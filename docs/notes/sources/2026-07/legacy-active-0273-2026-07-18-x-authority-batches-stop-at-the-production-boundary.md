---
id: legacy-active-0273
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "security", "architecture"]
---
# 2026-07-18: X Authority Batches Stop At The Production Boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8778–8794. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The X session loop now translates each projected authority batch once into a protocol-neutral
production record containing Engine transactions and surface removals, renderer DMA-BUF and
fence registrations, Present submissions, and release handles. X resource IDs, client IDs,
protocol errors, and authority-specific CPU update records do not cross that boundary.
`PersistentBackendRuntime` no longer accepts `XAuthorityObservedTransactionBatch`; CPU and GPU
production entry points consume only the reduced production batch plus renderer updates.

The full offline all-feature suite passes. The rebuilt X13 QEMU image passed strict two-xterm
in 7,008 ms with 117 of 117 authority transactions, 7 ms input presentation, 42 submissions,
40 retirements, and zero cleanup debt. The guarded native mixed diagnostic translated and
exported one CPU plus one DMA-BUF layer with zero live sources, fences, or transactions. The
remaining Milestone 6 ownership work is moving the now-neutral visual control implementation
out of the CLI module and retiring its legacy committed-snapshot APIs.


<!-- END IMPORTED BODY -->
