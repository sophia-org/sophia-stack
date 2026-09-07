---
id: legacy-active-0327
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "validation"]
---
# 2026-07-25: Asynchronous WM Physical Gate Passes

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10170–10189. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first two-output four-Kitty cycle after moving WM socket waits off the
owner thread completed normally. Maximum physical-input phase time fell from
100 ms to below the millisecond evidence resolution, input queue dwell fell
from 246 ms to 12 ms, and submit-to-page-flip observation fell from 210 ms to
23 ms. The WM transport reached a 101 ms round trip without holding the owner;
its peak depth was one and it drained with zero rejection, stale response, or
pending request.

The session produced 220 mixed exports with exactly 220 complete targets,
pipelines, and frame surfaces, zero recovery or generation replacement, clean
callback retirement, and clean input, control, protocol, and process teardown.
The fourth-window transition atomically held and committed four surfaces
because xmonad promoted the newly focused window to master. The verifier had
assumed the old master remained fixed and required exactly three changed
surfaces. It now correlates the held transaction with its matching commit and
accepts either three or four changed surfaces while retaining the exact
pixel-matched four-pane geometry checks.

<!-- END IMPORTED BODY -->
