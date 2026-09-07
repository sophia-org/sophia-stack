---
id: legacy-active-0579
date: 2026-08-31
recorded_date: 2026-08-31
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-31: frontend teardown must remain cancellable under backpressure

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18177–18233. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical candidate after the card-scoped completion-pump repair
showed continuous xterm progress to the operator, but the outer watchdog
returned status 124 before the terminal gate asked for visual confirmation.
The retained lifecycle stopped at `joining_frontend`. Session telemetry still
owned one pending CPU visual update, while X Authority had emitted thousands
of backpressure waits. The display result and the shutdown failure were
therefore independent; suppressing the visual question had discarded useful
evidence and made the successful visible portion look like another render
failure.

Code-level ownership explained the hang. Client workers sent transaction
envelopes into a separate egress sequencer, which accumulated out-of-order
tickets in an unbounded map and could then block on the Engine-facing bounded
channel. Frontend teardown joined that path, but normal session exit had only
a stop-admission request followed later by teardown cleanup. It did not define
when frontend authority, already accepted batches, CPU updates, and native
presentation work were collectively drained, and the cancellation owner was
not guaranteed to remain reachable while the join was blocked.

The sequencer and its unbounded reorder map are removed. One shared ordered
egress coordinator now makes producer ownership the bound: every client worker
may retain only its current envelope, the raster service owns at most one, and
a condition variable wakes out-of-order producers when the next ticket
advances or shutdown begins. The Engine-facing channel remains the existing
bounded channel. `StopAccepting` closes only new admission so accepted work can
finish; `StopAndDisconnect`, command-channel loss, or transport loss cancels
the coordinator and wakes every waiter. Stable schema-1 egress telemetry
reports tickets, deliveries, peak waiting producers, wait episodes, resumes,
and cancellations.

Normal successful session exits now enter an explicit two-second quiescence
phase. Admission stops once, physical input is suppressed, and the owner keeps
servicing authority, CPU composition, and native retirement until the frontend
sender disconnects, no authority batch remains, CPU visual accounting is
settled, and no native work needs owner progress. Completion wins at the exact
deadline. A timeout reports each remaining owner, sends forced disconnect, and
fails rather than silently joining forever. Emergency and fatal exits retain
their immediate recovery path.

`XAuthorityShutdown.tla` models the split producer, ordering, delivery,
stop-admission, cancellation, and session-drain transitions. The positive
model checks bounded producer ownership, ordered delivery, cancellable egress,
and safe session exit. Exact negative controls independently enable premature
exit and unbounded ingress and must violate `NoUncancellableEgress` and
`BoundedProducerOwnership`. Deterministic Rust regressions cover cancellation
of a capacity-zero blocked worker, transport disconnect, completion only after
all quiescence owners drain, and exact CPU pending-update retirement.

The physical wrapper now executes exactly one attempt and never infers that a
final `WouldBlock` merits an automatic retry. It always asks the visual
question after the benchmark process returns, records separate machine and
visual verdicts in schema 2, and passes only when both succeed. Another
physical run is justified only after this candidate passes production checks,
is signed, and materially replaces the diagnosed teardown path.

<!-- END IMPORTED BODY -->
