---
id: legacy-active-0254
date: 2026-07-17
recorded_date: 2026-07-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-07-17: Unattended GTK Input Acceptance In QEMU

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8368–8387. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

A direct-kernel, diskless, networkless QEMU guest now runs the real Zenity
entry dialog under both `classic_shared` and `confined` namespace profiles.
The host harness uses QMP only to drive virtio keyboard and mouse devices; the
guest receives those events through the normal physical-input poller. Both
profiles type exact `sophia`, observe changed pixels, route a physical OK-button
click, match Zenity stdout, exit normally, and cleanly retire both virtio-gpu
outputs with `protocol_errors=0`.

The trace-driven compatibility slices added core ChangeGC and CreateCursor,
XIChangeCursor, bounded opaque non-input SendEvent delivery, XIUngrabDevice,
and a protocol-shaped XIQueryPointer reply. It also exposed a proof-loop bug:
Return suppression was scoped to the entire pointer-proof run rather than only
the pre-selection phase. Suppression now ends when pointer selection becomes
ready, and an application proof cannot complete before its primary child exits.
The QEMU result closes the deterministic semantic gap; guarded target-hardware
classic/confined captures with resize remain the promotion gate.


<!-- END IMPORTED BODY -->
