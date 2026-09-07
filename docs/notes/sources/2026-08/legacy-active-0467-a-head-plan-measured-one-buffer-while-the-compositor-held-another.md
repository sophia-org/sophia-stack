---
id: legacy-active-0467
date: 2026-08-18
recorded_date: 2026-08-18
date_basis: first-heading-commit
date_commit: b8dbf5b71f95cca697e409011f62eed2421832a1
committed_at: 2026-08-18T20:25:33-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# A head plan measured one buffer while the compositor held another

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14191–14214. The heading has no date. Its first recorded addition is commit
`b8dbf5b71f95cca697e409011f62eed2421832a1` (2026-08-18T20:25:33-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Composing the scene for every output moved the failure one layer down: the
session now dies at `SourceSizeMismatch(7)` while lowering a head
composition, immediately after the mirror heads plan cleanly. The extended
head is in the scene at last -- that is what makes this reachable -- and the
surface it must show has a size the compositor cannot supply.

The two records come from different places. A head plan takes each layer's
`source_pixel_size` from the committed content set, which a resize advances at
admission. The pixels come from `displayed_surfaces`, a retained renderer
image refreshed only when that surface's own Present retires. Between those
two moments the committed record names a buffer whose pixels nothing has
composed yet, and the retained image still holds the previous frame. Lowering
compares them and ends the session.

Which record is stale decides the fix, and the error could not say: it carried
a bare buffer handle. It now carries the surface, both sizes, and the handle,
because "7" cannot distinguish a mirror surface pinned to its produced pixels
by a recovery extent from an extended-output surface that answered a configure
the compositor has not drawn yet. Both are live candidates here and they want
opposite treatments -- one wants the plan to bind what can be drawn, the other
wants the head to keep its current frame until the client's Present lands.

<!-- END IMPORTED BODY -->
