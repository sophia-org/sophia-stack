---
id: legacy-active-0546
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-28: the first full physical latency run, and what it measured

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16895–16932. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Thirty-five sessions completed at TTY3 on `67bf886b`. The readiness fix, the
NumLock tolerance, and the stall retry all held: DP-2 withheld two more
teardown vblanks (samples 2 and 10) and the retry budget absorbed both, which
makes three occurrences today and a pattern on this host rather than a
one-off. The default budget is now four, because two was exactly consumed by
one run at the observed rate. The run's report came out empty through my own
fault: the preserved stall evidence was named `sample-002.stall-1`, the
reporter globs `sample-*/session.log`, and it correctly refused a terminated
session's log it should never have been fed. Stalled evidence now lives under
`stalled-sample-*`, outside the namespace.

Rerun over the thirty-five real sessions, the reporter gives the first honest
physical numbers, and they fail the stage contract in one specific place:

- queue dwell: 0-1 ms everywhere -- passes its 1 ms budget.
- submit-to-page-flip: bimodal 0-3 ms and 13-17 ms -- vsync phase, within
  one refresh, as designed.
- dwell-to-submit: 28-44 ms in every session against a 17 ms budget. Not a
  tail; the minimum is already over. Full chain p99 62 ms against 34.

The population is thinner than it looks. The injector types its fourteen
events in one burst, they ride one page flip, and each session's seven
"samples" settle with microsecond-identical latencies -- thirty-five
independent measurements, not two hundred forty-five. The distribution's
shape between sessions is pure vsync phase on the submit side. The burst also
means the per-session p99 is the session maximum, and the reporter's
worst-session rule makes the reported p99 the global maximum.

The structural finding stands regardless of sampling fidelity: about two
refresh periods elapse between input delivery and the KMS submission that
carries the resulting pixels, uniformly. That interval contains the client's
redraw, damage observation, CPU scene composition, per-head render, and the
wait behind whatever submission is already in flight. Where those ~35 ms go
is the next question the latency row asks, and it is measurement-driven
optimization work, not a harness defect.

<!-- END IMPORTED BODY -->
