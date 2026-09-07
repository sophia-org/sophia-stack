---
id: legacy-active-0311
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-07-24: Scroll And Hidden Focus Were Missing Input Semantics

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9666–9689. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The Firefox promotion audit found that libinput wheel events were dropped
before entering Sophia packets. Input now carries signed, protocol-neutral
horizontal and vertical v120 units through Engine hit-testing. Only the X
frontend maps those units to core X11 scroll buttons, emitting a bounded
press/release pair. The deterministic Firefox workload now proves scroll,
focus-away/focus-return, and a pointer-opened dialog in addition to keyboard,
CLIPBOARD, PRIMARY, resize, and normal exit.

The same audit found that workspace policy cleared its hidden-focus record
without clearing the Engine seat or X frontend focus. A hidden but still
committed surface could therefore remain the keyboard target. Workspace-away
now issues a surface-scoped clear-focus control, waits for X authority
acknowledgement, clears Engine focus, and records the transition. A harmless
key typed before workspace return must be explicitly suppressed for lack of
focus. The physical verifier requires the ordered sequence: workspace away,
focus cleared, key suppressed, workspace returned.

Adding axis routing pushed the mutable route registry over the cohesion
threshold. Resolved-input selection and frozen-input draining now live with
the existing routing-input owner, returning the registry below 1000 lines
without changing its public facade.

<!-- END IMPORTED BODY -->
