---
id: legacy-active-0102
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-08-07: Inset Present content is distinct from outer layout geometry

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3350–3385. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The live Firefox proof received every Super+Space action, but its standing
resize target never retired. Firefox presented through a child inset by five
pixels on every side: the managed outer surface was `1276x1422`, while the
exact DMA-BUF was `1266x1412`. Sophia had already accumulated the child offset
for rendering, but the layout coordinator treated the raw buffer extent as the
outer extent. It therefore retained the `1280x1040` launch recovery constraint
and reconciled every later xmonad proposal around the stale fallback.

XLibre keeps parent and child window geometry separate through
`ConfigureWindow`, and yserver likewise retains window-tree geometry and
Present offsets as independent facts. Sophia now makes the same distinction at
its protocol-neutral boundary: `SurfaceTransaction::target_geometry` remains
the logical surface rectangle, while `target_content_size` records the exact
buffer extent. X Authority derives the latter before projecting a descendant
Present to its toplevel. The live reducer accepts the logical extent only when
the imported buffer exactly matches that content extent, and the visual tracker
retains both sizes so native retirement cannot be forged by either one. An
inset regression covers exact acceptance, stale-buffer rejection, standing
target retirement, and recovery release.

The checked-in xmonad order also placed `Tall` immediately after
`ThreeColMid`. For the focused master those layouts share the same outer extent,
so one valid NextLayout action resized only the two Kitty surfaces while the
Firefox proof waited for a Firefox resize. The order is now `ThreeColMid`,
`Mirror Tall`, `Tall`, `Full`, `Spiral`; the first action changes the focused
master from vertical to horizontal geometry. The configured real-xmonad smoke
proves the exact sequence. This is policy behavior, not an Engine shortcut or
an application-specific proof exception.

The full Rust all-features suite and configured real-xmonad smoke pass. Because
the executable protocol contract and packaged policy changed, the installed
`56dad4de` xterm and ten-cycle results remain historical evidence; a new
immutable successor must repeat installation and physical gates.

<!-- END IMPORTED BODY -->
