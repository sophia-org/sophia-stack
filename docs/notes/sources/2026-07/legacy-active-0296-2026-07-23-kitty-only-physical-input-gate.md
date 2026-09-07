---
id: legacy-active-0296
date: 2026-07-23
recorded_date: 2026-07-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-07-23: Kitty-Only Physical Input Gate

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9201–9222. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first run after the Present identity fix appeared completely locked, but
the durable log proved two physical mouse motions were observed and routed and
Super-Enter launched a second Kitty. Both xmonad layouts timed out, and the
cursor path could replace a DMA-BUF application frame with a CPU-only scene.
This was a visible presentation failure, not lost libinput events.

The supported TTY3 gate now starts one automatically focused Kitty with no
external WM while retaining both physical outputs and the independent recovery
guard. Xmonad, Super-Enter, and resize synchronization are deferred until
keyboard, pointer motion, button delivery, drag selection, and clean shutdown
pass together.

DMA-BUF sessions no longer invoke the CPU cursor repaint. Backend-live owns a
64x64 ARGB dumb buffer containing a classic left pointer and uses the DRM
hardware-cursor interface to install it on the active CRTC, move it without a
primary-plane repaint, and transfer it across the extended-horizontal output
layout. Motion is coalesced, presentation latency and hardware update failures
are reduced into the session evidence, and CPU repaint remains available only
for CPU-buffer regression clients.

<!-- END IMPORTED BODY -->
