---
id: legacy-active-0032
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-27: exact retirement unlocks three complete native targets

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1059–1093. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Milestone 14 begins with the model boundary rather than a renderer lifetime
  guess. `VisualRetirementSlots.tla` complements the existing two-output model
  with four ordered generations, one two-head mirror output, and three complete
  target slots. It reaches the displayed + submitted + prepared capacity state,
  treats the fourth generation as deferred, advances incarnation on reuse, and
  makes a delayed old release an explicit no-op. The corresponding Category B
  modeling brief maps worker, KMS, mirror, failure, VT, and latest-wins paths.
- Each physical-head renderer worker now owns a passive three-slot ledger.
  Allocation returns a typed slot ID/incarnation token which travels beside the
  worker lease ID. Page-flip retirement drops that exact lease; the worker first
  validates both identities, retires or recycles the buffer, and only then
  releases the slot. Duplicate, stale, or mismatched returns leave a reused slot
  occupied and increment refusal telemetry.
- A fourth live generation is ordinary bounded backpressure. The worker returns
  its immutable frame instead of reporting export failure; the exporter restores
  it only when no newer latest-wins frame is already pending. Requests settled
  this way count as slot deferrals, not renderer completions or failures.
- GPU fallback paths select the target bundle associated with the acquired slot.
  The bundle retains EGL context, GL pipeline, GBM/EGL frame surface, and import
  cache together and is reused only after the worker slot is explicitly free.
  Size changes and retryable target failures rebuild a free bundle. Direct CPU
  BO recycling and renderer-image capture remain separate bounded mechanisms.
- Native resource schema 7 reports acquisitions, reuses, deferrals, stale
  releases, live slots, and aggregate high-watermark. Existing schema-5/6
  evidence remains accepted; schema 7 balances worker requests as completed
  exports plus bounded slot deferrals.
- Pinned TLC 1.7.4 exhaustively checked 4,149,619 generated and 1,100,230
  distinct states to depth 34. An occupied-slot allocation mutation violated
  `ActiveGenerationOwnsSlot` at depth 5; an ABA mutation that cleared the
  current owner on stale return violated it at depth 8. Deterministic Rust tests
  separately cover fourth-generation deferral, round-robin reuse with advanced
  incarnation, and refusal of an old token after reuse.

<!-- END IMPORTED BODY -->
