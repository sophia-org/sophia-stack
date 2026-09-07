---
id: legacy-active-0478
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: 14fcf238a6a560d72f45bd82a37491b9c028e940
committed_at: 2026-08-21T07:20:02-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# The desk changed and the transport kept answering with the old one

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14520–14546. The heading has no date. Its first recorded addition is commit
`14fcf238a6a560d72f45bd82a37491b9c028e940` (2026-08-21T07:20:02-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The idempotence fix did not stop the restart cascade, and the reason is one
line that never appears in any of these logs:
`sophia_live_output_authority status=hardware_snapshot_published`. A snapshot
reaches output clients only through the hardware path -- a rescan or a hotplug
-- and a policy-driven topology commit does not take it.

So the compositor committed epoch 2, advanced its own published snapshot, told
the submitting client the new epoch in its outcome, and left the output
transport service holding epoch 1. That stored copy is what every future
connection is answered with. The restarted proof therefore received a desk
from before the change, saw its topology was not applied -- correctly, given
what it was told -- rebuilt the candidate against epoch 1, and was refused as
stale. Three times.

The idempotence check was still worth having and stays; it just could not
answer a question it was being lied to about. What was missing is that a
committed topology becomes the desk, so the transport's copy has to become it
too. `finish_output_settlement` now publishes the settled snapshot through the
same path the hardware publication uses, which the two now share rather than
each keeping its own transaction counter and degradation warning.

The shape is familiar by now: one fact -- what the outputs currently are --
living in two places, with only one of them updated. The compositor's authority
knew. The service that answers clients did not.

<!-- END IMPORTED BODY -->
