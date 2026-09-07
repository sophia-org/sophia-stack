---
id: legacy-active-0351
date: 2026-07-31
recorded_date: 2026-07-31
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-31: retained primary CPU pixels follow normalized output damage

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10887–10910. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first physical archive after secondary-output caching completed all 20
  exact input transactions with clean clocks and teardown, but still measured
  25 ms p95 and 28 ms maximum against the 17 ms budget. Native upload remained
  bounded at 2 ms. The remaining owner-path interval was 9–13 ms from input
  dwell to KMS submission.
- `LiveProductionCpuScene` retained the prior primary allocation but cleared
  and rebuilt all 3840x960 pixels on every changed display list. The xterm
  surface occupied only a bounded part of that output, so the compositor did
  work already excluded by Engine's conservative output-damage proof.
- Primary composition now snapshots the current display list, surfaces, and
  software cursor before drawing, compares it with the retained snapshot, and
  reduces the result through `plan_output_repaint`. Partial plans clear only
  clipped damage and replay every intersecting surface, solid border, and
  cursor pixel in original stacking order. Skip plans retain the allocation
  without a copy. Missing history, invalid proof, full-repaint policy, or an
  incompatible retained allocation uses the existing full-frame path.
- A uniquely owned retained frame is mutated in place. Shared storage is copied
  before a changed partial repaint so an in-flight or observed frame remains
  immutable. Regressions cover outside-damage preservation, removed pixels,
  stacking, clipped damage, old/new cursor extents, shared storage, invalid
  baseline fallback, and the production snapshot route.

<!-- END IMPORTED BODY -->
