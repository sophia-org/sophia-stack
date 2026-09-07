---
id: legacy-active-0477
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: 6599b2dba024949ecf78e848c472cfe772eb58ec
committed_at: 2026-08-21T07:11:41-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# A proof that could not survive its own restart

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14485–14519. The heading has no date. Its first recorded addition is commit
`6599b2dba024949ecf78e848c472cfe772eb58ec` (2026-08-21T07:11:41-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The member-optimized run died three times over in eleven seconds:

```
sophia_live_wm restart_requested reason=public_transport_failed error=TimedOut
sophia_live_wm restarted epoch=2 restarts=1 preserved_layout=true
output_authority rejected transaction=1 error=Projection(InvalidCandidate(StaleTopology))
Error: NonCommittedOutcome(Rejected)      x3
Error: "public WM supervisor did not restart Hagia"
```

Everything before that worked. The mixed topology committed at epoch 2 early in
the run, presents flowed for half a minute, and when the policy transport went
quiet for twelve seconds the session did what it is designed to do: restart the
policy, preserve the layout, carry on. The restart is recovery, and it
succeeded.

What could not survive it was the reference proof. Restarted, it rebuilt the
candidate it had submitted the first time and sent it again -- naming a base
epoch the compositor had already moved past. The compositor refused it as
stale, correctly. The proof reads any non-committed outcome as failure, exited,
was restarted, did the same thing, and exhausted the supervisor.

A supervised process is restarted by definition, so its work has to be
idempotent, and the idempotent form of "apply this topology" is "is this
topology applied?". The proof now asks that first and reports the live epoch as
settled when the answer is yes. Optimizing for the other head is a different
desk and is not mistaken for this one, which the test pins alongside.

The restart itself is left alone. Twelve seconds of silence from a policy
client is a long time, restarting a wedged one is the designed recovery, and
loosening that on one observation would trade a working mechanism for a guess.
It is on the roadmap as something to distinguish if it recurs.

<!-- END IMPORTED BODY -->
