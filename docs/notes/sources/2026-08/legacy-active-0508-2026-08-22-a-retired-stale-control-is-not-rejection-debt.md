---
id: legacy-active-0508
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-22: a retired stale control is not rejection debt

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15561–15589. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Signed source `f085774a7bf755b0ecd4b97d9396112db5950a65` produced and
independently re-verified mirror archive `0005`. The following mixed run,
`/tmp/sophia-mixed-output-centered-20260822-164837.log`, proved the late pixel
correction: all three heads retained nonzero exports, their asynchronous
submissions, callbacks, and retirements balanced, and native completion
verified three heads. It also exercised the pinned-Present deferral before
cleanly draining native ownership.

The process still returned an error. A short-lived surface had disappeared
before `PublishMetadataRule` transaction 2 reached the frontend. The owner
classified its `UnknownSurface` acknowledgement as `stale_target_retired`, as
designed, and continued. At shutdown, however, the session-control aggregate
reported 19 enqueued, 19 dispatched, 18 delivered, and one rejected. Its old
terminal rule required dispatched to equal delivered and rejected to remain
zero, so the expected retirement could never drain cleanly.

The queue now classifies acknowledged stale targets in its own `stale_retired`
bucket. It still returns the rejected outcome to the owner, which means only
`ClientGone`, or `UnknownSurface` for close and metadata publication, can take
the nonfatal retirement path. Every other rejection remains terminal.
Completion schema 2 requires `dispatched = delivered + stale_retired`, no
pending work, and zero true rejections, timeouts, or unexpected replies. The
external regression includes the physical `19/19/18/1` shape, and evidence
readers accept old schema-1 archives while enforcing the new balance for
schema 2. This is a local executable correction; the signed physical sequence
must run again before either open promotion row advances.

<!-- END IMPORTED BODY -->
