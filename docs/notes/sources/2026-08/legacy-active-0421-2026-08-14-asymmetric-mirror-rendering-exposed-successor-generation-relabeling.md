---
id: legacy-active-0421
date: 2026-08-14
recorded_date: 2026-08-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-14: asymmetric mirror rendering exposed successor-generation relabeling

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12782–12805. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Diagnostic mirror attempt `0001` on source `bbd50ef0` reached direct-CPU
  bootstrap and worker readiness on both DP-1 and DP-2 without an AMDGPU
  command-stream rejection. DP-2 rendered, submitted, and flipped logical frame
  2 while DP-1 was still rendering that frame. DP-2 then completed retained
  frame 3, but the scheduler preferred the still-active group identity 2 and
  attempted to submit the connector twice for generation 2. The lifecycle
  correctly rejected that transition as `Duplicate`; the gate failed at the
  runtime stage and archived the trace rather than promoting it.
- A mirror generation is now reserved when it is queued, before either renderer
  or KMS submission. A connector that has submitted the active generation stays
  fenced even after its early physical callback, and exporter results retain
  the identity of `rendering_content` instead of borrowing a newer pending
  frame. Cleanup-blocked heads preserve the active frame in their exporter and
  defer one latest successor per head; the successor is promoted only after the
  active frame enters rendering or submits. A progress-based 500 ms watchdog
  covers stalls before first KMS submission and reports each connector's KMS,
  cleanup, worker, pending, rendering, and deferred state.
- Regressions cover the observed fast-head sequence and the adjacent case where
  repeated successors arrive while a slow head is cleanup-blocked before worker
  capture. The full all-feature workspace suite passes. Physical acceptance
  remains open until a clean signed source reruns the tty4 gate successfully.

<!-- END IMPORTED BODY -->
