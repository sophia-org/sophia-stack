---
id: legacy-active-0559
date: 2026-08-29
recorded_date: 2026-08-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-29: sharing costs nothing measurable, and a retry rule that cost a run

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17378–17409. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first latency run with a card's outputs on one renderer thread died on its
fifteenth sample, but the fourteen that completed answer the question it was
asked. Across 63 presses at `renderer_workers=1`: full chain p99 23 ms against
the 34 ms budget, dwell-to-submit p99 7 ms, submit-to-page-flip p99 18 ms --
each identical to the dedicated run within a millisecond, including the stage
a shared queue could plausibly cost. Only `insufficient_samples` failed, which
is the run ending early rather than anything the numbers say. Sharing appears
to cost nothing measurable; a full run is still owed before that is a claim.

What ended it was the retry classifier, not the session. A stall landed
between arming and proof completion, and the rule refused to redo that window
on the reasoning that the stall might be what the sample was measuring. It was
not: the stall was on the head with no client on it, and the session recorded
no measurement at all, so there was nothing to contaminate. The rule failed a
thirty-five session run over a sample that produced nothing.

The budget decides now, not the timing. Any hard stall is redone within a
bounded budget, and a stall that keeps happening exhausts it and fails the
run, which is the escalation that belongs there -- a sample either contributes
a measurement or it does not, and one that does not is free to redo whenever
it died. The rule that replaced it is also simpler to state, which is usually
the sign that the first one was carrying an argument it should not have been.

The stall itself was the tenth of the pattern and the first to carry the
schema-2 attribution: `poller_pending=0 poller_routes=2
poller_last_read=WouldBlock`. Empty, routed, last read clean -- the completion
event never crossed the card descriptor, which is the signature that says the
fault is below this process. The instrumentation built for that question
answered it on its first real occurrence.

<!-- END IMPORTED BODY -->
