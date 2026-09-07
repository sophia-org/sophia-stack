---
id: legacy-active-0267
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-07-18: One Session Coordinator Owns Visual State

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8654–8672. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`PersistentBackendRuntime` now owns one session-level `ProductionSessionCoordinator`.
Authority commits, Present preparation, retired Present completion, and public committed
state all use that owner. Per-output backend assemblies receive immutable snapshot
projections for rendering and scanout; they are no longer selected as a primary authority.
A regression deliberately changes the first output projection to generation 99, then
proves the session coordinator independently commits generation 5 to 6 exactly once and
overwrites both output projections with its result.

The full offline all-feature suite passes. On the rebuilt X13 QEMU image, strict two-xterm
completed 300 ticks in 7,013 ms with 117 of 117 authority transactions, 7 ms input
presentation, 42 submissions, 40 retirements, and zero phase or cleanup debt. Confined GTK
committed 58 SHM transactions, accepted exact physical text and pointer selection, exited
normally with `first_error=none`, and retired both outputs cleanly. Live runtime and native
scanout method invocation still need to move behind the production live adapter before the
Milestone 6 coordinator item can close.


<!-- END IMPORTED BODY -->
