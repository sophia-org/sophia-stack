---
id: legacy-active-0280
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-18: Production X Cursor Repaint Stops Replacing Engine State

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8875–8891. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The physical-pointer cursor repaint no longer composes frames in the outer X session loop or
calls the legacy committed-snapshot replacement entry point. A visual-runtime repaint method
reads the production coordinator snapshot, asks renderer-live to compose the cursor, creates
per-output frames, and submits them through the backend-owned output runtime set. The remaining
snapshot replacement API was named and called only by the Wayland maintenance adapter and its
regression; production X had no caller. The 2026-07-19 retirement removed that adapter.

The full CLI all-feature suite passes. On the rebuilt X13 QEMU image, strict two-xterm completed
in 6,941 ms with 120 of 120 transactions, exact keyboard and pointer proofs, 42 submissions,
40 retirements, and zero cleanup debt. Resize-enabled classic and confined GTK accepted exact
text and pointer selection, committed 640x360 CPU\/SHM redraws, exited normally with
`first_error=none`, and cleanly retired both outputs. The guarded native mixed diagnostic
exported one CPU and one DMA-BUF layer with zero live sources, fences, or transactions.


<!-- END IMPORTED BODY -->
