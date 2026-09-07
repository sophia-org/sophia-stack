---
id: legacy-active-0481
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: 8fd0a6f0419f43200acd59fb7b6aa89a82786f08
committed_at: 2026-08-21T11:33:22-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# A reader that left is not a transport that broke

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14624–14656. The heading has no date. Its first recorded addition is commit
`8fd0a6f0419f43200acd59fb7b6aa89a82786f08` (2026-08-21T11:33:22-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Publishing the committed topology made the default gate fail where it had been
passing, and the line explains itself once the order is visible:

```
committed_snapshot_published transaction=2 topology_epoch=2 transport_published=true
sophia_output_v1_reference status=settled kind=Committed topology_epoch=2
output_authority status=degraded reason="Io(\\"Connection reset by peer\\")"
```

The proof asks one question, gets its outcome, and its output thread ends --
which is correct, and which is exactly when the snapshot published after the
settlement arrives at a closed socket. The write path mapped every I/O error to
a service failure, so the whole output service ended and took its listening
socket with it.

That also explains the other run. With the socket gone, the restarted policy
found nothing to connect to: `Io(NotFound)` three times, and the supervisor
gave up. One defect, two different-looking failures, both introduced by
publishing to clients who had every right to have left.

The read path had drawn this line already -- an exhausted stream is
`PeerDisconnected`, not an error -- so the fix is to make the write path agree.
A write that fails with a reset, a broken pipe, or an unexpected end retires
that connection the same way a read does: disconnect, tell the owner, advance
the epoch, keep serving. Anything else is still a failure.

This is the fourth time in this work that one outcome has been covering two
situations, and the third for a transport specifically. The pattern is worth
stating plainly: a peer leaving is something a server does every day, and code
that cannot say so ends up ending itself.

<!-- END IMPORTED BODY -->
