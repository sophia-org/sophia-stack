---
id: legacy-active-0197
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-07-26: Output-Scoped Frame Service Passes The Xmobar Gate

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6738–6756. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical capture from frame-service commit `9b14ea9` passed the
focused unmodified-xmobar verifier. One 14-pixel top reservation reduced both
output work areas exactly; managed Kitty pixels began at `y=14`, and the bar
remained in the full-output client-positioned scene. Both button and axis
packets routed to that generic role without changing Kitty keyboard focus.
Workspace 2/1 and three VT suspend/resume cycles preserved the bar, managed
pixels, pointer, and keyboard.

The run created and retired 50 mixed composition targets, pipelines, and frame
surfaces with zero generation or recovery replacement. Both startup outputs
retained synchronous modeset proof, the WM worker drained with no pending or
rejected requests, and native suspension, session health, protocol accounting,
cleanup, input guard, and TTY restoration were clean. This closes the focused
status-bar gate and provides the first physical confirmation of the
output-scoped reducer. The four-Kitty and complete normal xmonad captures
remain required from the same commit before the lifecycle change is promoted.

<!-- END IMPORTED BODY -->
