---
id: legacy-active-0258
date: 2026-07-17
recorded_date: 2026-07-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-07-17: Backend Snapshot Ownership Moves Into Production Coordinator

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8460–8479. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`HeadlessCompositorBackendAssembly` no longer stores an independent
`Vec<CommittedSurfaceState>`. It owns a `ProductionSessionCoordinator`, and the
existing deterministic and live runtime adapters now receive and return the
coordinator-owned snapshot through one split Engine/state borrow. Public
`with_committed_surfaces`, replacement, input routing, rendering, and runtime
reports retain their behavior, but the live backend has one fewer visual-state
owner before the remaining CLI scene and native sequencing migration.

The focused Engine, all-feature live-backend, and live CLI suites pass. A rebuilt
X13 QEMU image also passed the strict two-xterm gate with two CPU layers, exact
keyboard and pointer routing, 8 ms input presentation, 40 submissions, 38
retirements, and zero cleanup debt. The confined GTK gate passed normal Zenity
exit, exact stdout, `first_error=none`, clean two-output retirement, and zero
native debt. This is an ownership migration, not the Milestone 6 exit: the
legacy runtime adapter still sequences commits and the CLI still owns
`PersistentCpuScene` and `PersistentNativeScanout`.


<!-- END IMPORTED BODY -->
