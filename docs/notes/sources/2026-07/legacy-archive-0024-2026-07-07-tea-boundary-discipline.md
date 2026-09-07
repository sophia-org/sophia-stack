---
id: legacy-archive-0024
date: 2026-07-07
recorded_date: 2026-07-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-07-07: TEA Boundary Discipline

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 403–414. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Sophia will use The Elm Architecture where it matches the authority boundary:
policy components consume snapshots/events, update private policy state, and
emit command packets. This is the default style for Sophia WM, portals, and
session policy.

Sophia Engine is not a global TEA application. It is the compositor authority
and must stay performance and security centric: owned tables, typed IDs,
generation checks, spatial indexes, damage queues, renderer systems, and
auditable hot paths.

<!-- END IMPORTED BODY -->
