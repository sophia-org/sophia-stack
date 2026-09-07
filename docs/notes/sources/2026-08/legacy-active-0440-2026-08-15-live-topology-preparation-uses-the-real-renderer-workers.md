---
id: legacy-active-0440
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "tooling"]
---
# 2026-08-15: live topology preparation uses the real renderer workers

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13228–13250. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The session owner now captures one committed scene twice: once through the
  provisional candidate viewports/targets and once through the still-published
  rollback topology. It queues the candidate head frames, polls their ordinary
  persistent renderer workers, then queues and polls every rollback head. Normal
  frame scheduling is quarantined while either pool is active.
- A frame set is admitted only with exact enabled-head candidate coverage, exact
  previously-enabled-head rollback coverage, and damage extents matching each
  native selection.
  A candidate-disabled head has no candidate raster but still has a rollback
  raster and prepared detach properties.
- Failures enter an abort phase: unsubmitted frames are discarded, in-flight
  worker results are polled and their leases released, partial native owners are
  cancelled, and retryable cleanup is retained separately because candidate and
  rollback cancellation may produce two owners for one head. Session completion
  performs the same bounded abort before native suspension.
- The live path deliberately still cancels a fully prepared dual pool and emits
  `kms_submits=0`; applying it before runtime/head-table installation exists would
  expose a physically changed topology with no authoritative owner. The next
  slice is therefore accepted-owner installation plus card apply/rollback, not
  more renderer scaffolding.

<!-- END IMPORTED BODY -->
