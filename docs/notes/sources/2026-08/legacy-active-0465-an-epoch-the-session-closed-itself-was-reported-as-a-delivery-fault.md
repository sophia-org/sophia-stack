---
id: legacy-active-0465
date: 2026-08-18
recorded_date: 2026-08-18
date_basis: first-heading-commit
date_commit: a37c945a5cc9464a885990f87f1ae49b6eb010db
committed_at: 2026-08-18T20:02:14-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# An epoch the session closed itself was reported as a delivery fault

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14127–14145. The heading has no date. Its first recorded addition is commit
`a37c945a5cc9464a885990f87f1ae49b6eb010db` (2026-08-18T20:02:14-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The same run died before any of that mattered, four seconds in, with
`X11 input delivery failed: outcome=RouteRejected client=1`. The pointer moved
while an output policy change closed the input security epoch. Closing it
strands whatever was in flight -- frozen input is drained, and any event
stamped with the old epoch is refused at the registry -- and both paths
reported `RouteRejected`, which the owner loop treats as fatal.

So every pointer motion that overlapped a topology, policy, or seat boundary
was a session-ending race. This is the timeout-is-not-a-fault distinction in
another costume: an outcome that means "the session did this on purpose" was
sharing a name with one that means "this could not be delivered".
`XAuthorityInputDeliveryOutcome::EpochRevoked` now names the first, and the
owner loop retires it the way it retires a departed target -- the event is no
longer expected, and it is reported rather than fatal. `RouteRejected` keeps
its meaning and keeps ending the session: an unresolvable window or an
unmappable button is still a fault.

<!-- END IMPORTED BODY -->
