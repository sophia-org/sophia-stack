---
id: legacy-active-0130
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-08-05: X map deferral requires a live policy owner

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4232–4254. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The second commit-pinned Kitty fallback attempt retired one valid black Present
and then reached the visual-readiness deadline without another submission.
Native scanout, page-flip retirement, focus delivery, and cleanup were healthy.
The real-Kitty frontend probe also passed with production's idle-before-complete
Copy feedback, excluding Present routing as the stalled boundary.

The live frontend had unconditionally enabled policy-deferred mapping, while
the no-WM layout deliberately bypassed policy admission. Those two decisions
left no owner capable of admitting the deferred toplevel. Kitty's pre-map
bootstrap Present could retire, but the window could never cross the X11 map
transition or receive the MapNotify, VisibilityNotify, and Expose events that
start its visible rendering.

XLibre maps immediately unless a SubstructureRedirect owner intercepts the
request. yserver independently implements the same rule and emits map,
visibility, and exposure only after the window becomes viewable. Sophia now
derives frontend deferral and Engine admission from one policy-map mode:
external-WM sessions defer, while no-WM sessions fulfill MapWindow directly.
The reducer regression makes the two states mutually exclusive, and the real
Kitty probe now uses production's Copy feedback order and cadence.

<!-- END IMPORTED BODY -->
