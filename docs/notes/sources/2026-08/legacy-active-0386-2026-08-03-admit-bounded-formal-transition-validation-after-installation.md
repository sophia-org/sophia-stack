---
id: legacy-active-0386
date: 2026-08-03
recorded_date: 2026-08-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-03: admit bounded formal transition validation after installation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11726–11750. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The roadmap now keeps Milestone 11 focused on the installed product path,
  adds one unattended formal-transition gate before the Milestone 12 soak, and
  requires Milestone 13 lifecycle optimizations to extend that model before
  changing frame-slot, coalescing, multi-output, shared-worker, scanout, or
  resource-release semantics. TLA+ adds no physical operator choreography.
- `validation/tla/VisualRetirement.tla` models proposal, preparation,
  submission, output-scoped retirement, rejection, timeout, disconnect,
  removal, and release. Its boundary map names the corresponding Engine and
  frame reducers while explicitly recording that this is not a refinement
  proof and must not normalize the direct-commit gaps found by the preceding
  audit.
- The first three-generation configuration exceeded one million distinct
  states without adding a new ordering class. The retained configuration uses
  two outputs and two generations: the smallest bounds that still exercise
  out-of-order retirement and supersession. TLC v1.7.4 exhaustively checked
  12,348 distinct states to depth 17 with all safety invariants and the
  admitted-work liveness property passing.
- `tools/check_tla.sh` requires an absolute path to the pinned official jar,
  verifies its SHA-256, runs one worker with a fixed fingerprint polynomial,
  and isolates TLC state in a temporary directory. The ordinary command is
  offline. Valid terminal quiescence is not treated as a deadlock, while the
  explicit weak-fairness liveness property remains checked.

<!-- END IMPORTED BODY -->
