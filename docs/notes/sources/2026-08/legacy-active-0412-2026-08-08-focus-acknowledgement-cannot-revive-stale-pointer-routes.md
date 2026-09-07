---
id: legacy-active-0412
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-08: focus acknowledgement cannot revive stale pointer routes

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12539–12555. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The bounded pointer-focus handoff previously released its complete buffered
  sequence when frontend-applied focus equaled the originally requested
  surface. It did not independently confirm that the surface and every queued
  target still belonged to the current interaction projection and frontend
  route table.
- Release now requires each exact generational `SurfaceId` to remain both
  renderable in the last-presented input snapshot and owned by a current X
  client route. Removal, generation replacement, or route loss cancels the
  handoff atomically before any delivery token is minted.
- The protocol-neutral handoff reducer accepts an authority-supplied membership
  predicate and has a regression proving generation-one buffered input cannot
  release after only generation two remains. The live owner reports stale
  cancellation separately from timeout and capacity cancellation. A security
  authority epoch remains part of the larger Engine-visible grab-lease slice.

<!-- END IMPORTED BODY -->
