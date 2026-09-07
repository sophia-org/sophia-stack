---
id: legacy-active-0145
date: 2026-08-04
recorded_date: 2026-08-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-04: admission evidence requires exact target-buffer identity

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4650–4678. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The live TTY7 vkcube trace selected transaction 683 as a `PresentedBuffer` for
surface 6291456, then committed that same transaction from `cpu_snapshot`.
The authority batch contained both the DMA-BUF Present and a CPU backing update
for the same surface and extent. Sophia retained only transaction and extent in
its safe observation, while a surface-level Presented flag upgraded both
sources. The backing snapshot could therefore impersonate the Present, leaving
the Vulkan client waiting behind a frame that never entered native retirement.

XLibre and yserver use extent to select flip versus copy/clip, not to establish
Present identity. Niri and river likewise keep requested, configured, and
rendering state attached to their owning serial or transaction. Sophia now
carries one exact transaction/surface/target-buffer key through candidate
selection, admission, scheduler ownership, native retirement, and feedback.
A valid buffer that does not match a pending resize renders against committed
geometry without promoting that resize; only malformed or superseded work is
rejected. Layout commit and abort are explicit epoch transitions, so timeout
cannot release staged work through a size-based recovery heuristic.

The regression places a DMA-BUF Present and CPU backing snapshot in the same
transaction and requires only the DMA-BUF key to receive Presented evidence.
The production scheduler regression proves that aborting one epoch does not
disturb another. `AdmissionRecovery.tla` checks exact selection through timeout,
recovery, native retirement, and Complete/Idle feedback, and the physical log
verifier now requires three increasing, clocked Present retirements. Offline
Rust, verifier, and TLC gates pass; one fresh installed TTY run remains the
physical milestone gate.

<!-- END IMPORTED BODY -->
