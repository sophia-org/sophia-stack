---
id: legacy-active-0210
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation", "tooling"]
---
# 2026-07-18: Milestone 8 Evidence And Offline Application Shape

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7129–7160. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Normal sessions now identify every startup and action-launched application by
its approved registry ID, terminate the complete process group when its leader
exits, and treat every reduced X protocol error as fatal. Separate health and
post-teardown records preserve schema-14 while exposing pending WM/action/input
state and confirming frontend, namespace, Xauthority, and application cleanup.

The retained Firefox probe forces native X11, uses an isolated profile and
bounded process-group termination, and requires nonzero pixels with
`first_error=none`. A real host run now passes after adding bounded
`MIT-SHM CreatePixmap`, core `GetImage` and `ReparentWindow`, XKB `GetControls`,
larger bounded image/property payloads, and semantic classification of the
window-zero probes Firefox deliberately tolerates. The offline
page advances visible content and monotonically sized title witnesses after
keyboard, clipboard, primary-selection, resize, and dialog stages without
putting title bytes in evidence. Explicit `xmonad-m8-mix` and
`xmonad-m8-soak` scenarios use an optional Firefox/vkcube/Lavapipe image profile
and have strict positive plus negative fixture verifiers. The self-contained
image builder now resolves an installed xmonad and recursively includes and
validates Firefox's ELF dependency closure. The host Firefox proof completed
396 requests across 45 opcodes, two committed runtime surfaces, 5,971,968 CPU
buffer bytes, nonzero pixels, and no unexpected protocol error.

The first real mixed QEMU run reached xterm and Lavapipe but exposed an
application-size/recovery seam: `vkcube` retained its 500x500 default while the
post-bar xmonad layout requested 640x720. The guest now starts it at the stable
tiled size, and the host harness waits for per-application start/exit evidence
instead of fixed sleeps. The mix and 30-minute soak remain open until those
revised real gates pass; fixture verifiers and the complete offline test suite
are green.

<!-- END IMPORTED BODY -->
