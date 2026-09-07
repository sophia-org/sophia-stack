---
id: legacy-active-0196
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-26: Four-Kitty Geometry Follows The Engine Work Area

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6712–6737. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first four-Kitty run after the output-scoped frame-service change rendered
and exited cleanly, but its verifier rejected the missing pre-bar target
`1280x1440_0_0`. The retained layout proved that the expectation was stale:
the active status-bar reservation correctly changed the primary work area to
`2560x1426_0_14`. Xmonad and Engine produced one `1280x1426` master pane and
three `1280x475`, `1280x475`, and `1280x476` stack panes that covered the work
area exactly through `y=1440`.

The verifier now correlates the four-surface atomic resize transaction with its
workspace projection, obtains that output's applied Engine work area, and
derives the expected Tall geometry from the bounded rectangle. It no longer
assumes a particular output mode, origin, or absence of reservations.
Mutation fixtures use the 14-pixel reservation and reject an incomplete
projection or a one-pixel work-area/tile mismatch.

The corrected strict gate passes the retained physical run: 259 mixed exports
matched 259 target, pipeline, frame-surface creation and retirement lifetimes;
generation and recovery replacement remained zero; maximum input dwell was
20 ms, render time 20 ms, and submit-to-page-flip observation 32 ms. Both
outputs retained their valid startup lifecycle, the WM and session-control
ledgers drained, and cleanup recorded zero unexpected protocol errors. The
complete normal xmonad gate and one more consecutive four-Kitty cycle remain
before the frame-service lifecycle slice is promoted.

<!-- END IMPORTED BODY -->
