---
id: legacy-active-0530
date: 2026-08-24
recorded_date: 2026-08-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-24: a policy answer is not a failed attempt

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16253–16317. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The rerun that cleared the XInput errors died with a native page flip past the 500 ms
hard-stall boundary, five seconds after `Super+B`. The cause was an extra `Super+N`
earlier in the guide. It advanced the active view one further step around the layout
cycle, into monocle, which places exactly one window however many it is shown. When
the browser arrived, Hagia's committed proposals placed only the guide -- correctly,
and freshly re-evaluated every cycle; its own test pins four windows to one placement.

Sophia treated that answer as an absence and re-asked it 1,438 times in five seconds.
Nothing counted the re-asks: a proposal that places nothing never becomes pending, so
it commits synchronously and the timeout path that would have noticed never runs, and
the cause dedup only covers a request that is queued or in flight, both of which clear
at every settlement. Each commit then re-applied focus the seat already held and
republished indicator chrome that had not changed, and each of those invalidated the
composed scene, so the retained queue overwrote the pending frame under the flip until
one output stopped retiring.

The instrument that looked designed for this was the wrong one. `admission_retries`
counts timeouts on the way to withdrawing a surface: one more past a retry sends
`WithdrawSurface` and tears the window down. Counting policy answers there would have
turned a slow admission into a killed window, and its damping is permanent besides --
nothing re-offers a damped surface, and the public restart path never replays a Manage
request at all.

So the fix records the answer rather than the attempt. A committed proposal raised for
`Manage(s)` whose layers exclude `s` is a settlement, keyed by the connection epoch and
scene generation it was given. The scene generation advances exactly when the snapshot
changed and the epoch when policy restarted, so any later settlement-carrying commit
retires every entry recorded against other facts and nothing sweeps on a timer. Map
state and authority surface facts re-arm explicitly, since neither moves the
generation on its own.

The reason a settled surface is not stranded is that admission never needed the
request: `stage` derives its geometry and admission sets from a proposal's layers
alone, with no reference to the cause, so whichever proposal eventually places the
surface admits it through ordinary visual retirement. Leaving monocle places it. The
settlement withholds a question, not a window.

Two of the amplifiers were defects in their own right. Engine focus reported `Focused`
whether or not anything moved, so a window manager reasserting focus drove an X11
control every commit; it now reports `AlreadyFocused`, per seat, after the
committed-surface check. Indicator chrome was published with the projection commit
serial inside it, and consumers compare the whole publication, so every policy commit
missed the strip's raster cache, damaged the whole strip, and cancelled any indicator
click in flight. The serial is gone from the publication and the hit target, and the
publication generation now advances only when the published content differs. That
cancelled-click behaviour was a live defect on every commit, not only under this loop,
and is fixed rather than recorded.

The retained composition queue also took whatever it was handed, overwriting an
unscanned frame with a copy of itself. It now skips a cohort whose scene the output is
already holding, compared against content still pending in the head rather than the
last checksum queued -- that value outlives its frame, and retained composition is
edge-triggered with no re-arm, so suppressing on it would lose a frame instead of
skipping one. With the first three fixes the loop has nothing to recompose, so this is
depth rather than the cure.

The guide fixture let the extra keypress through: its action wait was a threshold
rather than a count, and the browser step then waited forever for a two-surface layout
that policy would never propose. It now fails where the extra press happened, and the
browser wait is bounded with the hint that a single-window layout is what it is seeing.

The signed installed rerun remains the promotion gate.

<!-- END IMPORTED BODY -->
