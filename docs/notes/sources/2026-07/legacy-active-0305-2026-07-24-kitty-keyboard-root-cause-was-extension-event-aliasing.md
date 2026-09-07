---
id: legacy-active-0305
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-24: Kitty Keyboard Root Cause Was Extension Event Aliasing

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9498–9523. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The strict real-Kitty input gate now passes. Physical input discovery, routing,
focus, event selection, and XCB receipt had all been working. An instrumented
libX11 showed that each core KeyPress/KeyRelease reached its queue and was then
rejected by the installed wire converter. Sophia advertised GLX with traditional
event base zero, so libGLX registered its seventeen extension converters over
core event numbers 0 through 16, including KeyPress 2 and KeyRelease 3.

Sophia now assigns non-core, mutually disjoint traditional event ranges to
RANDR, XFIXES, SYNC, GLX, XKEYBOARD, XInputExtension, and MIT-SHM. The XKB
names reply also reports level-name counts consistent with the two levels
advertised by XkbGetMap. Installed Kitty 0.48.0 consumes routed `ll` plus
Return, writes the exact shell result, and submits three later Presents. This
is protocol-level, application-agnostic behavior; no Kitty policy exists in
the engine.

The subsequent guarded TTY3 run provided the physical promotion proof. Kitty
became visibly ready in 798 ms; physical keyboard input and two pointer-button
transitions were routed; cursor motion-to-submit remained bounded at 13 ms;
Kitty exited with status zero; protocol health was clean; and the originating
TTY modes were restored without emergency recovery. A separate report-field
bug falsely rejected that successful run because the stable-Present readiness
path logged readiness without persisting its elapsed time; both paths now
populate the same readiness measurement.

<!-- END IMPORTED BODY -->
