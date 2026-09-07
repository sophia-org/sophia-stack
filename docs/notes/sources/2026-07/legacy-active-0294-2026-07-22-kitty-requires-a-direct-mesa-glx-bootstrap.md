---
id: legacy-active-0294
date: 2026-07-22
recorded_date: 2026-07-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-22: Kitty Requires A Direct-Mesa GLX Bootstrap

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9139–9166. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The guarded physical rerun reached xmonad and input recovery, then Kitty 0.48.0
failed with `failed to create GLFW window`. Sophia advertised GLX 1.4 but no
FBConfigs and omitted `GLX_EXT_libglvnd`; libglvnd therefore selected its
indirect vendor instead of Mesa and never reached the already implemented
DRI3/Present path.

Sophia now exposes a depth-32 ARGB TrueColor visual, maps GLVND's vendor query
to `mesa`, and implements a bounded direct-rendering GLX bootstrap: visual and
FBConfig catalogs, client-info negotiation, direct context lifecycle, GLX
window aliases, and drawable attributes. The catalog deliberately contains
only XRGB linear, ARGB linear, and ARGB sRGB configurations with depth/stencil
zero. Indirect GLX rendering, server-side Render/RenderLarge, and GLX
SwapBuffers remain unsupported; Mesa renders client-side and submits through
DRI3/Present. The first live trace additionally showed Mesa using X Sync to
destroy DRI3 fences and GLFW freeing its ARGB colormap, so the bounded Sync
initialization/fence teardown and core colormap teardown paths are retained as
part of the same compatibility slice.

`x-authority-kitty-smoke` now proves the exact live sequence on an AMD render
node: GLVND vendor selection, FBConfig discovery, two direct context/window
lifecycles, depth-32 modifier negotiation, ARGB DRI3 import, one accepted
Present transaction, one committed runtime surface, and clean protocol state
with `first_error=none`. The guarded physical TTY3 capture remains the session
gate; the standalone smoke deliberately terminates its proof window after the
first committed frame because it has no live renderer feedback loop.

<!-- END IMPORTED BODY -->
