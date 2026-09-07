---
id: legacy-active-0115
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-06: Retained X11 remaps do not owe redundant geometry

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3825–3852. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The unattended M12 QEMU lifecycle reached Firefox focus isolation, then the
xmonad bridge disconnected after waiting three seconds for one synthetic
`ConfigureWindow`. Engine had switched to an unfocused workspace containing
one previously admitted Firefox surface. Its action snapshot already carried
the exact committed node and geometry; xmonad legally left that unchanged
during the remap.

The private facade compounded the wait by deleting its window and stacking
record on `UnmapNotify`. XLibre and Yserver both retain the window, change only
its map state, and leave destruction to `DestroyWindow`. The facade now follows
that lifecycle. A sole-node focus cycle still sends the bounded profile chord
so xmonad's stack converges, but does not require a geometry response that has
no remaining policy choice. The regression queries the retained unmapped child,
remaps it without a configure response, and requires the opaque `FocusSurface`
result. New-window admission and every non-deterministic layout fence remain
fail closed.

The first rerun passed all eight Firefox stages and the repaired refocus cycle,
then exposed the adjacent teardown invariant. Destroying focused Firefox left
the private core-focus record stale, and xmonad's next focus-stack update named
an unmapped workspace child. Engine correctly rejected that proposal as
`HiddenFocus`. The facade now reverts focus on unmap or destroy, returns X11
`BadMatch` for a later hidden `SetInputFocus`, and translates focus only for a
mapped synthetic target. Regressions retain the unmapped child while proving
focus reversion, local rejection, and suppression at the blind-WM boundary.

<!-- END IMPORTED BODY -->
