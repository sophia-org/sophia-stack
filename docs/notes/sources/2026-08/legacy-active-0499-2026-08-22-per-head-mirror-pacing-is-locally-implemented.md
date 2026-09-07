---
id: legacy-active-0499
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-22: per-head mirror pacing is locally implemented

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15252–15283. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The topology contract now names one primary `RenderHeadId` for every logical
  output. Candidate planning rejects missing, extra, disabled, or cross-output
  primary assignments before KMS mutation, Engine reports the primary's refresh
  as the logical clock, and topology apply installs the same choice into both
  the head registry and mirror lifecycle.
- `OutputPresentationCohort` retains the prepare-all barrier but separates its
  two terminal facts: the primary flip completes logical presentation, while a
  generation becomes releasable only after each physical owner has either been
  skipped before submission or cleaned up after its head moved on. The live
  lifecycle mirrors that split with per-head in-flight/displayed generations.
- The native scheduler accepts a complete successor while an older generation
  remains submitted. Each head prepares ahead independently, submits only the
  newest generation when its own KMS lane is free, and cancels stale prepared
  or renderer-completed work instead of relabelling it. A primary callback can
  therefore release frame feedback while a slower secondary still owns an
  older buffer; the secondary later skips directly to the newest prepared
  generation.
- Evidence now emits `sophia_live_mirror_pacing` records for primary logical
  presentation, coalescing, and last-head release. The mirror and mixed-output
  verifiers require an ordered primary-presentation/release pair, and their
  operator prompts explicitly permit transient motion-time lag while requiring
  final settled convergence. Deterministic lifecycle/cohort tests, both shell
  verifier fixture suites, and the feature-complete Rust suites cover the local
  contract. This is not physical promotion: clean signed mirror and mixed
  hardware reruns remain required. The umbrella atomic-local wrapper still
  stops in its source-layout audit on pre-existing unreviewed oversized and
  inline-test files (including files already above the limit at `HEAD`); that
  repository-wide decomposition debt is not promoted by these passing pacing
  checks.

<!-- END IMPORTED BODY -->
