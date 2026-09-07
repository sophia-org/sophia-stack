---
id: legacy-active-0568
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-30: the cursor archives, and three that tested nothing

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17715–17761. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Archive `0004` established the legacy baseline the atomic path had to match,
rather than assuming the ioctl kept working over directly scanned frames:
twelve positions driven through the same `Pointer::place` entry physical input
uses, 519 hardware updates with no failures, the cursor never leaving
`legacy_ioctl`, `composed_cursor` still zero, and twenty-six client buffers
reaching the plane after the motion stopped. Motion-to-submit peaked at 9
milliseconds.

The three archives before it had asserted this and tested none of it. Their
cursor records read `moves_coalesced=0 max_motion_to_submit_msec=0
hardware_updates=1` -- a cursor initialized once and never moved. The lesson is
narrow and repeats elsewhere in this log: a counter at its initial value is
indistinguishable from a counter nothing exercised, and an archive that names
one is evidence of nothing.

Archive `0005` put the cursor on an atomic plane: the card accepted it, twelve
moves reached it, no hardware failures, direct scanout undisturbed, and
`updates_primary_in_flight=0` against the legacy path's fifteen -- the kernel's
per-CRTC serialization observed rather than assumed. It cost worst-case
latency, 21 milliseconds against 9, which is about one frame of waiting for a
busy CRTC. It also showed the owner never combining primary and cursor state:
every atomic update was a standalone commit, and `plan_cursor_commit`'s
`RideNextPrimary` branch was unreachable because nothing populated it.

Archive `0006` closed that. The same twelve-move sweep produced only eight
cursor-only commits, because the rest rode primary commits as one combined
atomic request -- the thing the owner was asked to be able to do, observed
rather than claimed -- and the worst case came down to 17 milliseconds. A
rejected combined commit retries with the primary alone, prepared beside the
combined request rather than rebuilt after failure, so a cursor can never cost
a frame; this run needed no such retry, and the counters would have named it.

The legacy baseline was then replaced under continuous motion rather than
twelve synthetic moves: 57.97 fps at p95 16.687 ms, no cursor failures, and no
commit overlapping a page flip. The same day's accidental legacy run is what it
replaces and is the first legacy evidence under continuous motion: 298 hardware
updates, 243 of them overlapping a flip. The atomic path did that work in 56
commits with none -- a five-fold reduction with pacing intact.

Repairing the gate that grades this was part of the row. The pacing gate
demanded `wm_policy=external` from a benchmark that had become standalone and
matched a resource schema three revisions old, so it could only ever pass
against its own fixture. That is the same defect class as the reader drift
recorded below, found the same week in a different file.

<!-- END IMPORTED BODY -->
