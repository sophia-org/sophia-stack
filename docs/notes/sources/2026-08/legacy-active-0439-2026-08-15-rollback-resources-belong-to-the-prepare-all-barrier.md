---
id: legacy-active-0439
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-15: rollback resources belong to the prepare-all barrier

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13209–13227. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Preparing only candidate framebuffers is insufficient. Once card zero accepts
  a candidate, card one's refusal creates an immediate need for the old topology;
  attempting to allocate rollback resources at that point introduces a failure
  between mutation and recovery.
- A typed live resource cohort now requires exactly one candidate member and one
  rollback owner for every affected opaque head before it reports ready.
  Disabled candidates contribute a discovered connector/CRTC/plane property set
  rather than a fake framebuffer; previously disabled heads do the same on the
  rollback side. Duplicate, unknown, or wrong-disposition
  insertions return the supplied owner instead of dropping it.
- Card/head observations into `LiveOutputAuthorityOwner` now stage through a
  cloned reducer and publish the batch only if every member is accepted. Native
  execution therefore cannot partially advance protocol authority because of a
  malformed later member. The remaining work is the nonblocking renderer-worker
  driver that fills this cohort and transfers accepted owners into rebuilt live
  targets.

<!-- END IMPORTED BODY -->
