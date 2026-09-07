---
id: legacy-active-0279
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-07-18: GTK QEMU Gate Now Proves Resize Redraw

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8858–8874. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The retained classic and confined GTK QEMU profiles previously passed input and native
presentation while reporting `surface_resize=disabled`, even though Milestone 5 requires a
CPU\/SHM redraw after an Engine-owned resize. Both guest profiles now request 640x360, and
the host harness rejects evidence unless the application record carries the complete semantic
tail: zero protocol errors, exact physical text, routed pointer selection, committed resize,
CPU\/SHM buffer path, native presentation, and clean teardown.

On the rebuilt X13 QEMU image, classic and confined Zenity each committed the resize with a
configure acknowledgement and changed pixels, accepted exact `sophia` input plus pointer
selection, exited normally with `first_error=none`, and retired both virtio-gpu outputs with
zero cleanup debt. Strict two-xterm also passed in 6,989 ms with 117 of 117 authority
transactions, 40 submissions, 38 retirements, and zero phase or cleanup debt. The remaining
Milestone 5 promotion gate is the deliberately operator-driven paired physical X13 capture.


<!-- END IMPORTED BODY -->
