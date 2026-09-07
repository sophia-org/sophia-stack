---
id: legacy-active-0119
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-06: VT resume transfers renderer-owned snapshots

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3929–3979. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Native-chrome attempt `0001` isolated the resume failure. The seat and KMS
state quiesced and reacquired correctly, but `LiveProductionVisualRuntime`
retained renderer-image IDs after the old native owner—and therefore its image
table—was destroyed. The replacement worker received a retained mixed frame
whose IDs belonged to the discarded generation. It correctly rejected that
frame as `InvalidTarget`, and every later Present remained mixed with the same
unresolvable scene state.

Keeping the old worker alive is not currently safe because its GBM device is a
clone of the seat-leased primary DRM node. Sophia instead exports each promoted
compositor snapshot as an opaque, bounded DMA-BUF lease after native work has
drained. The old KMS, EGL, and GBM owners are then released. After seat
reacquisition, the replacement renderer copies and promotes the exact same
image-ID set before `resume_native_scanout` may queue retained content. Missing,
duplicate, invalid, or unexpected identities fail the lifecycle transition.
An unsolicited revoke cannot guarantee a handoff, so it explicitly clears
stale runtime identities rather than entering a permanent invalid-target loop.

This keeps migration inside renderer/backend ownership and leaves Engine
protocol-neutral. It also preserves the future optimization suggested by
niri's split render/KMS model: Sophia can later move composition onto a
persistent same-GPU render-node owner and make the same typed handoff a no-copy
generation transfer. The current implementation does not retain revoked KMS
authority to obtain that optimization prematurely.

Installed native-chrome attempt `0005` exposed an ordering defect in that
handoff. VT release drained native work and captured both retained images, but
resume tried to import them before native output initialization had created the
replacement renderer worker. The new exporter therefore had neither an inline
context nor a worker image table and rejected the first snapshot. Resume now
initializes every replacement output owner, restores the exact image set, then
publishes the output runtime and queues retained content. A transition reducer
rejects restore-before-owner and duplicate lifecycle observations; an exporter
regression proves that the image owner does not exist before initialization.

This matches the mature reference sequence: niri reactivates DRM devices and
connectors before it schedules redraw, while yserver re-establishes modesets
before its full-damage repaint. Sophia additionally preserves its explicit
renderer-generation handoff because its retained scene stores renderer-owned
image identities across native-owner replacement.

The installed `d29e2f2c` rerun passed as native-chrome archive `0006`. The VT
transition drained with no abandoned scanout, captured two images, restored
both after tty7 reacquisition, and retired the first nonzero retained mixed
frame before later Present work. The session routed 28 physical keys, completed
all chrome generations, and logged out normally with zero native submit,
retirement, callback, renderer-worker, protocol, or cleanup failures. This
closes the focused installed switch-away/switch-back gate.

<!-- END IMPORTED BODY -->
