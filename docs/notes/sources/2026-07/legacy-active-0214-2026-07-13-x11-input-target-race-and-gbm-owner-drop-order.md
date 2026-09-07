---
id: legacy-active-0214
date: 2026-07-13
recorded_date: 2026-07-13
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-13: X11 Input Target Race And GBM Owner Drop Order

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7217–7245. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Two fresh dedicated-X13 milestone attempts stopped at different seams. The first
routed and flushed all fourteen physical key events but received no later xterm
pixels. The second reached terminal content and Engine-applied focus, then
aborted with `free(): invalid pointer` before input readiness. Both runs restored
`keyd`, released DRM ownership, and left no Sophia process or core file.

Inspection found that Engine `FocusSurface` control and client-selected keyboard
delivery shared one atomic X window. A late focus command could replace xterm's
VT child with its top-level surface window after the child selected key events.
Those states are now separate: focus control updates only the surface window,
while key delivery retains the latest client window selecting key events and
uses the focused surface only as a fallback.

The native CPU-upload path keeps each locked GBM front buffer and its originating
surface alive through KMS retirement. That owner's destruction order is now
explicit: release the front-buffer lock first, then release the surface. A
shared persistent GBM/EGL surface was tested and rejected after it reproduced a
pre-input guest crash; the proven per-frame EGL surface path remains in place.

`tools/run_x11_live_session_stability.sh` adds bounded normal, lifecycle-trace,
GDB, and core-capture modes. Timeout evidence now distinguishes no client
update, stale buffer generation, unchanged composition, and missing native
presentation. An X13 QEMU rerun now reaches changed pixels for all fourteen
physical key events and exits without the allocator abort. Its independent
pointer-selection proof still times out with observed-but-unrouted events, so
repeated local TTY runs and the full physical milestone gate remain required.

<!-- END IMPORTED BODY -->
