---
id: legacy-active-0046
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-08-16: native startup now crosses a semantic prepare-all barrier

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1465–1491. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The initial empty desktop, first CPU authority cycle, explicit repaint, and
resume/recovery paths no longer modeset a projected flat raster and then replace
it with per-head work. Each validates and queues one complete
`HeadCompositionPlan` result per physical head, establishes every renderer
worker before export, prepares every framebuffer and modeset property owner
without KMS mutation, and admits the blocking card commit only when the worker
and prepared-owner sets exactly cover the required opaque heads. Mirror groups
remain card-local, so the first visible mutation is one atomic card-scoped
request rather than a sequential head prefix. Accepted owners are all adopted
before any bookkeeping error may return; rejection cancels prepared owners into
the retryable cleanup ledger. A failure before KMS also drains or discards every
queued renderer-worker command before clearing its passive content and damage
state, so startup cannot leave an affine renderer lease detached from rollback.

The physical mirror contract now rejects `direct_cpu` startup evidence. It
requires matching plan, queue, worker-composed, and synchronous-modeset records
for one shared semantic scene, plus the later plan/queue/KMS chain for the final
interactive frame. A pure barrier regression proves that missing workers,
missing prepared owners, prepared-before-worker state, and foreign heads cannot
reach KMS. Whole-output admission additionally rejects duplicate or incomplete
logical-output batches before any output runtime advances. Resume and recovery
restore retained renderer images before deriving the same per-head startup
transaction; there is no compatibility flat-baseline modeset left in the native
startup lifecycle.

<!-- END IMPORTED BODY -->
