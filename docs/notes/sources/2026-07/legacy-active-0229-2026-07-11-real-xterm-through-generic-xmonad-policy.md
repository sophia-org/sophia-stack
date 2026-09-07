---
id: legacy-active-0229
date: 2026-07-11
recorded_date: 2026-07-11
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-07-11: Real Xterm Through Generic Xmonad Policy

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7677–7693. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`sophia-live-session` now supervises an arbitrary Sophia WM socket process via
generic executable/argument flags. With `sophia-x11-wm-bridge` selected as that
process, the live session sends one opaque xterm surface to real xmonad, Engine
validates the response, and the committed placement drives composition,
hit-testing, backend visual state, and scanout. A headless integrated run proves
one moved surface, committed Engine focus, and a later injected terminal pixel
change. No xmonad identity enters Engine or the live-session policy path.

The Engine-to-X-Authority control seam now supports bounded configure/focus
commands and reduced acknowledgements keyed by `SurfaceId`. Probing arbitrary
full-output xterm resizing exposed a repaint loop in the core-drawing path, so
the first real one-client gate pins min/max size to the established live buffer
and uses xmonad for placement, stacking, and focus. Removing that constraint is
tracked explicitly rather than overstating resize compatibility.

<!-- END IMPORTED BODY -->
