---
id: legacy-active-0054
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-15: RenderHeadId replaces raw connector identity above the backend

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1659–1684. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Engine records no longer carry connector or CRTC integers. The DRM registry
  is replaced by `EngineHeadRegistry`: bounded `HeadRenderTarget` records
  grouped by logical output, keyed by an opaque session-scoped `RenderHeadId`
  the backend mints when it builds page-flip sessions. The backend retains the
  card/connector/CRTC/name mapping privately in
  `LiveProductionNativeHeadTable`; sysfs discovery moved with that identity
  into `sophia-backend-live`.
- Head targets are generation-stamped: readmitting a head with a different
  shape must advance `target_generation` or the registry rejects it as stale,
  which is the record-level footing for stale-plan rejection when per-head
  composition plans land.
- The logical view of an output is the shape all of its heads agree on, fail
  closed; refresh is deliberately excluded from that agreement because a
  mirror group's heads legitimately run near-but-not-equal rates. Logical
  pacing reduces to the slowest head's rate, matching the joined-retirement
  contract.
- Mirror lifecycles, page-flip routes, callbacks, kernel timestamps, and the
  per-head evidence lines are keyed by `RenderHeadId`. The one place a head id
  is printed beside its connector name is the readiness mapping line
  (`sophia_live_native_head schema=2 status=ready`), which is how physical
  verifiers correlate; every later per-head record carries only `head=`.
- A callback whose CRTC resolves to no admitted head fails closed with a named
  unknown-head error instead of being dropped or matched by output.

<!-- END IMPORTED BODY -->
