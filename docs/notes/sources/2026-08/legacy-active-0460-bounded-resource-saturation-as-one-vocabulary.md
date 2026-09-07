---
id: legacy-active-0460
date: 2026-08-17
recorded_date: 2026-08-17
date_basis: first-heading-commit
date_commit: ac12604b294a4cec78e4a6ebe851e2ec4191286f
committed_at: 2026-08-17T12:54:39-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling", "architecture"]
---
# Bounded-resource saturation as one vocabulary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13864–13923. The heading has no date. Its first recorded addition is commit
`ac12604b294a4cec78e4a6ebe851e2ec4191286f` (2026-08-17T12:54:39-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Seven consecutive physical-gate failures each turned out to be a different
bounded resource whose overflow was fatal: journal capacity, observation batch,
restart budget, transport write deadline, recovery request, owner deadlock, and
finally the native input acquisition queue. Every fix was correct and every one
revealed the next instance, which is the signature of a class defect rather than
seven bugs.

An inventory of the live-session path found 30+ bounded resources: 24 fatal, 6
silent drops, 1 unbounded, and 1 blocking with no deadline, against 6 doing
proper backpressure and 5 degrading with cause. The defect was not a missing
policy. Five dispositions were already in use and already documented. The defect
was that each site picked one ad hoc, so identical resource classes behaved
oppositely -- per-client input queues correctly closed one endpoint while the
shared ingress killed the session; focus handoff correctly discarded a held
sequence while the pressed-key ledger treated the same pressure as fatal;
page-flip decode correctly stopped and reported while its sibling callback queue
was fatal.

The root cause was a misreading, and it is worth naming precisely. `dod.md` said
of the pre-admission group FIFO that "capacity exhaustion or a mismatched member
is an explicit terminal error, never an unbounded allocation." That sentence is
about one FIFO where continuing really would split an atomic group. Only its
second clause generalises. Read as blanket policy it justified 24 fatal sites,
and the fix is not to soften it but to scope it: what a resource does at its
bound is now data attached to the resource, so changing that behaviour changes a
value rather than control flow at a call site. The FIFO itself stays terminal,
which is what makes this a classification rather than a general softening.

`TargetInputPacing.tla` gained the producer it never had. Device acquisition is
the one input the compositor cannot decline to have happen -- the packet already
exists when capacity is examined -- so a full queue is a choice of disposition
rather than an absence of work. Modelling it first was worth more than expected,
because two of the invariants written for it did not survive their own negative
controls. `EscalationCanAlwaysFlush` was a strict consequence of
`BoundaryCapacityIsReserved` and no edit could break one without the other, and
constraining `deferralTicks` in both `TypeOK` and `DeferralIsBounded` meant
`TypeOK` failed first, so the ceiling was never the invariant under test. Both
were removed rather than kept as decoration. The fairness control matters most:
all four new actions sit outside fairness, and deleting the assumption on `Drain`
still fails `QueueEventuallyEmpties`, so admitting a deferral disposition did not
quietly convert progress into a tautology.

Two model results changed the implementation rather than confirming it. The
reserve turned out to be load-bearing in a way the original model only implied:
two slots per active seat is exactly what an endpoint closure spends on its
terminating boundary, so the admission bound is what makes closure always
possible. And conservation at the acquisition boundary -- produced equals
admitted plus discarded plus held -- is what forbids two arrivals collapsing into
one record. A resource that coalesces under pressure reports success while losing
user input, which is the failure mode a gate cannot see.

The verification itself had a defect worth recording. The first several model
runs used the TLA+ jar named by a locally exported variable, which did not match
the checksum `tools/check_tla.sh` pins; the pinned 1.7.4 jar was elsewhere on
disk. The results happened to agree once re-run, but the pin exists precisely so
that agreement is not assumed, and the honest reading is that those runs proved
nothing until repeated under the pinned toolchain.

<!-- END IMPORTED BODY -->
