---
id: legacy-active-0476
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: be31143d345442f7952adbada120e6ec561f82a6
committed_at: 2026-08-21T07:05:26-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# An answer nobody will read is not a stuck key

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14452–14484. The heading has no date. Its first recorded addition is commit
`be31143d345442f7952adbada120e6ec561f82a6` (2026-08-21T07:05:26-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

With the raster measured, the member-optimized gate ran its full thirty
seconds -- presents flowing, the extended head exact at 1920x1080 with real
pixels, the mirror pair fitting a 1080p group onto a 1440p panel -- and then
failed at the very end:

```
runtime deadline key-release barrier timed out:
pressed=0 pending_deliveries=0 release_barrier_pending=0 policy_requests=1
```

Everything the barrier exists for was empty. The only outstanding obligation
was one projection request the session had asked the policy at the boundary.

That request is waited for on purpose, and the reasoning still holds: a
deadline lands at an arbitrary instant, the last pointer motion before it can
raise a focus request that cannot settle in the same tick, and ending on it
discards the user's final intent. What was wrong is what happens when the wait
expires. A key the session believes a client holds is a fault if nobody
released it -- somebody is left with a stuck key. An answer nobody will read
once the session stops is not a fault, and abandoning it is the correct end.

`AbandonedPolicyRequests` now names that outcome, reported with its count and
not raised as an error, while anything owed to a client still times out as
before. The test that pinned the old behaviour was rewritten rather than
deleted: it still asserts the wait, and now asserts the two different endings.

Three times in this work the same distinction has had to be drawn -- a socket
timeout that is not a dead peer, an epoch the session closed itself, and now an
unanswered question at shutdown. Each time the code had one outcome where the
situation had two.

<!-- END IMPORTED BODY -->
