---
id: legacy-active-0547
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-28: the latency gate was measuring the wrong page flip

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16933–16970. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Attributing the failed dwell-to-submit budget led somewhere worse than a slow
path. In all thirty-seven session logs of the first full physical run, the
page flip the gate measured against carried a composition built *before* the
keystrokes.

Sample 1 is the whole shape. Scene generation 6 was queued at 97 ms and a
render for it began when the previous submission retired at 107 ms. The keys
were routed at 129 ms, producing generations 7 through 11 -- the ones that
contain the typed text. Those superseded the pending frame, but a render
already under way cannot be restarted, and nothing overlaps it: the pipeline
is strictly render, submit, vblank, retire, next. The in-flight render
finished at 153 ms and submitted frame 11, generation 6. That submission
outranked the press's baseline and ordered correctly against every clock, so
`physical_input_page_flip_correlates` accepted it, the proof completed, and
the session exited on `--exit-after-input-proof`. The generations carrying
the text never reached scanout at all. Every session stalled by between four
and eleven generations this way.

The predicate asked whether a *later submission* had flipped, which is not
the same question as whether the input had been *shown*. Submission counters
order events; they say nothing about the age of the picture. Both the
one-shot correlation and the per-press sampling now also require the
presented composition to be newer than the newest one the head held anywhere
in its pipeline -- pending, rendering, submitted, or presented -- at the
moment the press was routed. The baseline is the maximum across those stages,
because a head can be rendering one frame while displaying another, and input
must beat all of them.

This makes the gate honest rather than fast. The numbers it reported (62 ms
p99) measured input-to-next-flip; a true input-to-photon figure has to
include the stale render's completion and is necessarily larger. The
optimization work this attribution originally aimed at -- a full-frame
`glReadPixels` proof landing on the input frame, and a software composite
rastered for evidence nobody presents -- remains real and is now measurable
against something that means what it says.

<!-- END IMPORTED BODY -->
