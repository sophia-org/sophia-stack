---
id: legacy-active-0436
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy"]
---
# 2026-08-15: the supervised public WM owns the live output role

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13154–13171. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Native public-policy startup now binds the exclusive `sophia_output_v1`
  endpoint inside the private policy directory, authorizes the exact supervised
  WM PID, and advertises only that endpoint through `SOPHIA_OUTPUT_SOCKET`.
  Static and legacy policy paths receive no physical-output capability.
- The optional transport service and `LiveOutputAuthorityOwner` are retained by
  the live WM session. Complete proposals are polled in a bounded owner turn;
  validate-only proposals settle through the resolver and topology reducer,
  while apply proposals fail explicitly at preparation rather than mutating a
  subset of native state.
- A supervised WM restart disconnects the old output peer, abandons its active
  and queued proposals, advances the connection epoch, and authorizes the new
  PID before it can reconnect. The published topology survives the role handoff.
  This closes the supervision boundary; renderer-target preparation, live KMS
  apply/rollback, runtime rebuild, and first-presentation publication remain the
  ordered cutover.

<!-- END IMPORTED BODY -->
