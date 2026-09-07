---
id: legacy-active-0433
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: per-head planning and output control need two distinct barriers

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13072–13100. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The multi-monitor target is now represented by production types rather than
  only prose. `SurfaceContentSet` carries bounded ready raster variants with
  density, transform, fidelity, and damage. Engine captures one immutable
  `OutputSceneSnapshot` and derives one native-size `HeadCompositionPlan` per
  opaque target. Logical viewports are stored independently from native head
  shapes, so unequal-mode mirror members no longer erase the logical output.
- The production CPU transaction constructs these plans from its exact committed
  slice before queueing the existing flattened compatibility frame. This is a
  deliberate shadow cutover: it proves the running caller and rejects invalid
  plans, while the native renderer still has to consume each plan directly.
  Claiming native per-head rasterization before that lowering replaces the flat
  queue would repeat the evidence mistake this architecture exists to prevent.
- Two Engine reducers keep resource and policy settlement separate.
  `OutputPresentationCohort` requires every head candidate to prepare and agree
  on logical identity before the first KMS submit, then joins flips and cleanup.
  `OutputTopologyTransaction` keeps the old topology published through prepare
  and physical apply, forces partial apply into rollback, and commits only after
  every new logical output presents once.
- The first `sophia_output_v1` Rust contract is implemented as its own exclusive
  authenticated role. A complete candidate independently selects each head's
  mode, transform, and VRR policy and groups arbitrary heads into mirrored
  logical outputs while leaving other groups extended. Backend projection
  exposes opaque head and bounded mode identities without leaking card, CRTC, or
  plane objects. Live supervision, renderer-target preparation, KMS
  apply/rebuild, and generated C conformance remain open and are not implied by
  the new transport tests.

<!-- END IMPORTED BODY -->
