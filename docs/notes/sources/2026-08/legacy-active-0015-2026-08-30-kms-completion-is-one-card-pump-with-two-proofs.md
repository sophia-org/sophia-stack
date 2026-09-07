---
id: legacy-active-0015
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-30: KMS completion is one card pump with two proofs

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 516–553. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The nine physical terminal attempts did not prove a below-process DRM fault.
  Their final read was `WouldBlock`, but the old runtime checked the watchdog
  before collecting in ordinary paths and relayed each decoded event through a
  poller queue, a bounded card channel, a fresh vector, a bounded output
  channel, and a runtime queue. The native reader also applied `take(max_read)`
  after libdrm had consumed one kernel event buffer, so callbacks beyond that
  iterator prefix could be lost permanently. An empty last read cannot
  distinguish those paths from a kernel that never emitted an event.
- Completion now has one owner and one bounded representation. The visual
  service reads each DRM card once into reusable scratch, routes by opaque head
  into one ledger cell per active submission, retires those cells, and checks
  the watchdog last. The reader consumes the complete iterator returned by a
  kernel read; limits apply to read syscalls and bounded downstream emission,
  never to an already-consumed kernel buffer. Cumulative diagnostics survive
  the last empty read.
- Event delivery is preferred but no longer the only kernel proof. Property
  discovery retains optional CRTC `OUT_FENCE_PTR`; accepted nonblocking
  page-flip commits own the returned sync file with the affine scanout
  submission. A signaled fence retires only when the event ledger is empty.
  That first fallback makes fences authoritative for the head, and later events
  are counted and discarded so a delayed predecessor event cannot retire its
  successor. A tiny `sophia-drm-out-fence` crate contains the kernel raw-FD ABI;
  the backend remains under the workspace's `unsafe_code = "forbid"` contract.
- The first `PageFlipCompletionPump.tla` run found a separate race: an event can
  become ready after the ordinary card pump but before a head crosses the
  terminal check. The service now performs one rescue pump and all-output
  retirement only when a hard-stall candidate exists, keeping normal cycles at
  one card read. The corrected two-head/two-generation model passes 38,410
  generated and 9,846 distinct states to depth 24.
- External regression coverage proves lossless bounded deferral through the
  direct collector and a pollable owned out-fence through the real submission
  assembly. The complete libdrm feature integration surface passes 277 tests.
  Physical closure still requires one clean signed schema-4 terminal run. A
  failed run must be classified from schema-3 cumulative and completion-source
  evidence instead of another blind retry.

<!-- END IMPORTED BODY -->
