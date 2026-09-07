---
id: legacy-active-0479
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: f50288c85d9c51ecf7f6d0ce21594883f4928b8b
committed_at: 2026-08-21T07:23:07-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# An update arrived in front of the answer

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14547–14572. The heading has no date. Its first recorded addition is commit
`f50288c85d9c51ecf7f6d0ce21594883f4928b8b` (2026-08-21T07:23:07-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Publishing the committed topology worked -- `committed_snapshot_published
transaction=2 topology_epoch=2 transport_published=true` -- and immediately
broke the client that had just asked for it:
`Codec(InvalidEnum { field: "message_kind", value: 66 })`. Kind 66 is
`OutputV1Snapshot`, and the client was reading the frame after its proposal as
an outcome, kind 68.

The publication went out before the settlement. A snapshot is an unsolicited
update; an outcome is the answer to a request. Sending the update first put a
frame the client was not waiting for in front of the one it was, and the client
read the next frame as an outcome regardless of what it was.

Both halves were wrong and both are fixed. The settlement now goes first and
the snapshot after it, which is the order a request and an update belong in.
And the reference client consumes updates while it waits rather than tripping
over them, because an update can arrive at any moment and a client that can
only survive quiet is not a client that survives a desktop.

Three runs, three different failures, each one a consequence of fixing the
last: publish nothing and a restarted policy reasons about a desk that is gone;
publish at the wrong moment and the client that asked cannot read the answer;
publish correctly and it is the client's turn to be strict about frames it did
not expect.

<!-- END IMPORTED BODY -->
