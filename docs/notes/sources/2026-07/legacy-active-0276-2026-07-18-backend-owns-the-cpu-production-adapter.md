---
id: legacy-active-0276
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-18: Backend Owns The CPU Production Adapter

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8816–8833. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Backend-live now owns `LiveProductionCpuCycleAdapter`. It applies renderer updates after the
Engine commit, composes or coalesces from the immutable committed snapshot, creates native
frames for every output, invokes one narrow output-runtime callback, and returns reduced
composition timing and evidence. The CLI no longer implements `ProductionPresentationAdapter`
or defines the CPU production frame record; its remaining callback projects the snapshot and
invokes backend runtime/scanout objects pending their final owner extraction.

The full offline all-feature suite passes. On the rebuilt X13 QEMU image, strict two-xterm
completed in 6,971 ms with 117 of 117 authority transactions, 7 ms input presentation, 40
submissions, 38 retirements, and zero cleanup debt. Classic and confined GTK accepted exact
physical text and pointer selection, exited normally with `first_error=none`, and retired both
outputs cleanly after 54-56 CPU compositions. The next extraction is GPU scheduling and the
concrete per-output runtime owner; legacy committed-snapshot entry points remain only for the
then-active Wayland maintenance path and tests.


<!-- END IMPORTED BODY -->
