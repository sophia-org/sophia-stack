---
id: legacy-active-0259
date: 2026-07-17
recorded_date: 2026-07-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-17: CPU Pixel Storage Leaves The CLI

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8480–8501. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Renderer-live now owns a protocol-neutral `LiveCpuBufferRegistry`. It accepts
immutable replacements and packed damage patches, rejects stale generations,
missing bases, metadata changes, invalid bounds, and malformed byte lengths,
and retires unreferenced handles. The X frontend remains responsible for
read-only MIT-SHM admission and emits its existing immutable updates; the CLI
only converts those packets at the renderer boundary. `PersistentCpuScene` no
longer contains a CPU buffer map or applies pixel patches itself.

Four focused registry regressions cover replacement/patch ordering, stale
generation rejection, fail-closed malformed replacement and patch behavior,
and resource retention. The live CLI suite passes. On the rebuilt X13-hosted
image, strict two-xterm QEMU completed 300 ticks with two CPU layers, 7 ms input
presentation, 40 submissions, 38 retirements, and zero cleanup debt. Confined
GTK passed its high-volume SHM redraw path, exact text/pointer proof, normal
exit, `first_error=none`, and clean two-output retirement. The remaining
Milestone 6 scene gap is narrower but explicit: CLI still projects a
`SurfaceId` to geometry/handle table because commit and composition have not yet
been split into coordinator phases.


<!-- END IMPORTED BODY -->
