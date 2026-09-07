---
id: legacy-active-0157
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-01: First physical Firefox rendering/input diagnosis

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5111–5140. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The isolated-profile physical M10 run advanced the deterministic Firefox page
through its loaded and keyboard stages and produced a real 1280-by-1040 DRI3
Present frame. After xmonad tiled the Firefox toplevel to 1276-by-1422, Sophia
rejected every subsequent frame because the final unit-scale predicate compared
the DMA-BUF dimensions with the clipped surface extent. The renderer placement
was already pixel-aligned: it retained a 1280-by-1040 target and clipped that
target to the client-positioned X child. Unit scale is now proved against the
unclipped target, so size transitions clip without stretching while Firefox is
between swapchain extents.

The same run showed that a physical click could hit Firefox's client-positioned
render child while Engine focus remained on Kitty; the later `Ctrl+Q` was then
correctly delivered to Kitty. Pointer delivery still targets the exact child,
but focus handoff now resolves that child to the highest containing
policy-managed surface owned by the same frontend client. X reparent operations
also publish their resulting presentation role, policy-to-client transitions
withdraw stale WM ownership, and WM requests reject client-positioned nodes.
This keeps X hierarchy semantics in the X/session frontend rather than the
protocol-neutral Engine.

Finally, the VT suspend/exit path found the Firefox request stream filling the
bounded observation channel. Observation overload now remains fail-closed and
bounded by disconnecting only the overloaded X client; it no longer turns a
client worker error into failure of the persistent authority service. Focused
regressions cover clipped unit-scale frames, descendant focus resolution,
policy withdrawal, and reparent role publication. A new physical run is still
required before any M10 workflow item is closed.

<!-- END IMPORTED BODY -->
