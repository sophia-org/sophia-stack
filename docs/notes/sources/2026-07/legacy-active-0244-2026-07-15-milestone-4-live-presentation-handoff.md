---
id: legacy-active-0244
date: 2026-07-15
recorded_date: 2026-07-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "tooling"]
---
# 2026-07-15: Milestone 4 Live-Presentation Handoff

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8141–8170. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Commit `11f93ee` leaves Milestone 4 at the boundary between proven protocol
transport and unimplemented native GPU presentation. The frontend publishes
DMA-BUF registrations, fence registrations, and Present submissions through
`XAuthorityObservedTransactionBatch`. `LiveDmaBufPresentationRegistry` owns the
reusable source and per-Present FD model, and
`XServerFrontendProtocolRouter` owns protocol-only completion delivery. No
persistent-session consumer currently connects those pieces, so the bounded
`vkcube` result remains Engine-transaction evidence rather than proof that its
Vulkan pixels reached KMS.

The current live-session assembly is also an explicit architecture debt.
`PersistentNativeScanout` and `PersistentCpuScene` remain in the CLI command;
the latter retains a CPU-only `SurfaceId` projection outside the normative
Engine scene owner. Moving the entire session loop before proving GPU
presentation would broaden the active milestone, while wiring more durable
scene and renderer authority directly into the CLI would deepen the debt.

The chosen continuation is a narrow hybrid extraction. Establish an
Engine/backend-owned live-presentation seam, then move only DMA-BUF import,
acquire-fence polling, mixed CPU/GPU composition, KMS submission correlation,
and page-flip retirement through it. Source and fence FDs transfer immediately
into renderer-private ownership. Engine preserves the last committed
geometry-plus-pixels state while a presentation is pending or rejected. Only a
real page flip containing the imported pixels may route Present Complete, then
Idle, trigger the idle fence, and retire the presentation exactly once. Broader
CLI session-loop extraction and Milestone 5 compatibility work remain deferred
until the software-plus-`vkcube` native KMS matrix passes.

<!-- END IMPORTED BODY -->
