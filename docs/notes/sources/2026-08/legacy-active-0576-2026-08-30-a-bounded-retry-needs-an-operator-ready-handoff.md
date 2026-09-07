---
id: legacy-active-0576
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-30: a bounded retry needs an operator-ready handoff

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18093–18117. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first bounded run on signed commit `44297a21` proved the classifier and
exposed the next procedural boundary. Attempt 1 stopped head 1 after four
retirements with `poller_pending=0`, two routes, `WouldBlock`, and no decoded or
rejected callback. The wrapper retained the complete attempt and selected
exactly one retry. Its kernel delta was complete and empty.

Attempt 2 never entered graphics takeover. The operator had treated the
restored display as the end of the run and left TTY3 while the next session
created a new recovery guard. That guard was not armed within 30 seconds, so
the launcher refused takeover, restored greetd, and returned a failed terminal
gate with `attempts=2 stall-retries=1`. The missing session and schema-4 report
are expected consequences of the pre-takeover refusal. This archive is not
CP-14.1 evidence and does not identify another Engine defect.

The safety chord cannot remain armed across retries because each session owns a
new independent guard process. Starting the retry immediately therefore hides
a required operator transition inside an apparently automatic loop. The
terminal gate now records `awaiting_operator`, pauses on the originating TTY
until the operator presses Enter, explains that the next guard must be armed
again, and records `operator_ready` before launching the retry. Closed input
fails instead of silently proceeding. The retry count remains bounded, but its
safety-critical handoff is deliberately operator-paced.

<!-- END IMPORTED BODY -->
