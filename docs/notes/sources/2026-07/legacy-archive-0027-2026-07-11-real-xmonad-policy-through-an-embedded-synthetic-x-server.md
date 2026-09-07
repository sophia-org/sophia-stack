---
id: legacy-archive-0027
date: 2026-07-11
recorded_date: 2026-07-11
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-07-11: Real xmonad Policy Through An Embedded Synthetic X Server

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 1448–1472. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The isolated `sophia-x11-wm-bridge` now has a production-facing binary and an
embedded local X11 server dedicated to one supervised xmonad process. The
server clears xmonad's inherited environment, supplies a private empty home and
`DISPLAY`, and exposes only the setup/root facts, generic atoms, empty property
answers, synthetic top-level IDs, and core lifecycle operations required for
layout policy. Rendering and input extensions are absent. No physical input,
client X socket, client buffer, title, class, PID, namespace ID, rendering
resource, or scanout object crosses the bridge.

The compatibility trace against the pinned xmonad checkout at commit `a9a8b5c`
covered real root-mask selection, Xinerama absence/fallback, color allocation,
keyboard/modifier discovery, property reads, focus, map, and configure traffic.
The minimal server now handles that bounded core subset and fails closed on an
unsupported opcode instead of silently proxying it elsewhere. xmonad is built
from its Nix flake, so this proof did not install GHC or xmonad into the host.

`tools/xmonad_wm_bridge_smoke.sh` passes with two synthetic surfaces in a 1280
by 720 root. Real xmonad emits two distinct 640 by 720 `ConfigureWindow`
requests. The bridge resolves the private synthetic IDs back to opaque Sophia
surface IDs and returns matching `ConfigureSurface`/`RenderSurface` commands in
transaction 1. The same runtime also exposes standard framed Sophia WM IPC over
its `serve-socket` mode; xmonad never sees the Sophia socket or transaction IDs.

<!-- END IMPORTED BODY -->
