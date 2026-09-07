---
id: legacy-active-0504
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security"]
---
# 2026-08-22: a departed surface does not turn disclosure into authority

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15452–15470. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Signed source `e0f43071103febb40ea16c948a9a16f4230df430` produced and
independently re-verified mirror promotion archive `0001`. The following
three-head mixed run then stopped during startup when a short-lived proof client
closed its X window before X Authority received the broker's
`PublishMetadataRule` control. The frontend returned `UnknownSurface` in two
milliseconds; the owner treated it as a policy rejection and ended an otherwise
recoverable session.

This is the same bounded-handoff distinction already made for closing a departed
surface. A disclosure rule has no effect without its exact surface, and the
broker retires the admitted surface when the frontend's removal batch arrives.
`UnknownSurface` is therefore a stale completion for
`PublishMetadataRule`, while `AuthorityRejected`, malformed rule identity,
timeouts, and failures for live targets remain fatal. The session-control
regression names that one additional stale pair and continues to reject
`UnknownSurface` for focus.

<!-- END IMPORTED BODY -->
