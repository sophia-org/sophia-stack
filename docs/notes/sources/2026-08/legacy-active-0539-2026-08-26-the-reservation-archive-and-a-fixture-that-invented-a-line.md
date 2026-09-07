---
id: legacy-active-0539
date: 2026-08-26
recorded_date: 2026-08-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-26: the reservation archive, and a fixture that invented a line

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16600–16626. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Signed Hagia archive `0007` closed the work-area reservation row: three claims
admitted and presented, two reductions and two releases, no refusal, the claim
retained across the shell's death and re-made at connection epoch 2, zero
protocol errors, clean teardown.

Two things the archive corrected about what had been written for it.

The matcher fixture carried two `sophia_live_work_area schema=1 status=applied`
lines. A public-policy session emits none: that print path returns before them
when `public` is set, and the archived run contains no work-area line of any
status. Nothing waited on them and no check read them, so they cost nothing --
but a fixture exists to stand in for a run, and a line no run writes is the
drift it is supposed to prevent. Removed; the reduction is evidenced by the
shell's own `reservation_reduced` records, which the session does write.

The reduced-band counts also did not fall where the guide's ordering assumed.
The second switcher presentation and its restart trigger land in the same tick,
so the band for that claim is published one tick later -- after the reconnect
line, not before the restart line. The run passed because the guide's waits are
cumulative and the verifier's retention check is anchored between the restart
and the reconnect, where nothing appears. Had either been written as a strict
before-the-restart ordering, a correct run would have failed. Cumulative counts
and narrowly anchored windows are what made an assumption about ordering
survive being wrong.

<!-- END IMPORTED BODY -->
