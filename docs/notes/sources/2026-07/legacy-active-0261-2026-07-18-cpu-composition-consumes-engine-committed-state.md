---
id: legacy-active-0261
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-18: CPU Composition Consumes Engine Committed State

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8527–8551. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The persistent CPU path now splits authority preparation from per-output runtime
ticks. Each batch commits once, renderer pixel updates are reconciled against the
resulting immutable `CommittedSurfaceState` slice, and composition resolves
geometry, buffer handles, stacking, readiness, and proof generations from that
slice before any KMS submission. `PersistentCpuScene` retains only the renderer
buffer registry and composition evidence; its independent `SurfaceId` table and
raised-surface state are deleted. Native runtime construction also no longer
requires a pre-commit frame or a blank modeset: KMS initializes with the first
frame composed from committed state.

The full offline all-feature suite passes. A rebuilt X13-hosted QEMU image passed
the strict two-xterm 300-tick profile in 6,824 ms with 123 of 123 authority
transactions applied, two CPU layers, 8 ms input presentation, 2 ms maximum
composition, 40 submissions, 38 retirements, and zero cleanup debt. Confined GTK
passed 56 committed SHM transactions, exact text and pointer evidence, normal
exit, `first_error=none`, 107 submissions, 105 retirements, and clean shutdown.
Classic GTK also passed the final image with exact application evidence and clean
retirement. The emergency profile flushed all five routed chord deliveries and
shut down without native debt in 151 ms. The duplicate
scene milestone item is complete, while CLI ownership of runtime/scanout and
feedback sequencing remains open.


<!-- END IMPORTED BODY -->
