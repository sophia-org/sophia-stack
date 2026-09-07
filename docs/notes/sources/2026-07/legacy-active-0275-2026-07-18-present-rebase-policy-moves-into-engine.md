---
id: legacy-active-0275
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-07-18: Present Rebase Policy Moves Into Engine

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8806–8815. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The full-state Present generation rebase now lives beside `ProductionSessionCoordinator` in
`sophia-engine::runtime_driver`. The visual runtime no longer reaches back into a CLI library
module to reconcile skipped authority generations with the last visible Engine generation.
The former CLI module is only a compatibility re-export for its retained tests. The full
offline all-feature suite passes; this is a dependency-boundary change with no runtime
behavior change.


<!-- END IMPORTED BODY -->
