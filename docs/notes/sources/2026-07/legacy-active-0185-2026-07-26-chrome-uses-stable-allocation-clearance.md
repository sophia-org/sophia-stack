---
id: legacy-active-0185
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "shell"]
---
# 2026-07-26: Chrome Uses Stable Allocation Clearance

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6287–6311. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first focused-border prototype painted four inside-edge solids over the
committed client rectangle. Increasing its width therefore hid client pixels.
The durable model now distinguishes a focused ring from an optional
focused/unfocused frame. Engine treats WM geometry as an outer allocation and
derives client content by one checked inset equal to the maximum enabled chrome
width. Focus changes only repaint; width changes prepare matching client
buffers before the new visual style becomes active.

The renderer-neutral display list carries one semantic border node per
surface/role. Damage and backend lowering share one fixed four-band expansion,
eliminating per-edge policy records and temporary edge vectors. Client-
positioned surfaces are excluded before display-list construction. KDL schema
2 and WM API v6 name focus-ring and frame policy explicitly; schema 1 is
rejected with migration guidance. This follows Niri's useful separation of
focus indicators and frames without importing its Wayland authority model or
making Sophia's Engine dependent on a particular WM, frontend, or renderer.

Chrome ownership is capability-gated rather than inferred from the presence of
a WM process. A native WM may negotiate the policy; the X11 compatibility
bridge deliberately does not, so xmonad and other external WMs use the
Engine/compositor fallback. Both sources reduce into the same candidate versus
visually committed state, keeping width changes behind one relayout boundary.

<!-- END IMPORTED BODY -->
