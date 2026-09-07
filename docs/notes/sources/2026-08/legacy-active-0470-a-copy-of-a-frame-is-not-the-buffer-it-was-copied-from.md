---
id: legacy-active-0470
date: 2026-08-19
recorded_date: 2026-08-19
date_basis: first-heading-commit
date_commit: a158d2a507af8a9994b0cac99a51cd04fb83a09d
committed_at: 2026-08-19T06:57:59-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# A copy of a frame is not the buffer it was copied from

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14262–14295. The heading has no date. Its first recorded addition is commit
`a158d2a507af8a9994b0cac99a51cd04fb83a09d` (2026-08-19T06:57:59-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The enriched error answered its question on the first run that hit it:
`SourceSizeMismatch { surface: 2097166, handle: 8, planned: 2560x1440, held:
1280x1440 }`. The mirror surface, its committed buffer grown to the full mirror
width by the layout that followed the topology change, and the compositor
holding a 1280x1440 copy of the frame that had retired moments earlier.

Both records were accurate about different objects. A head plan measures the
surface's committed buffer -- `variant.pixel_size` from the committed content
set. A retained source hands over the compositor's own renderer image, a copy
of an earlier generation, carried under the identity of that committed buffer
because that is what the plan will look for. `retained_composition_source_set`
pairs them: identity from `committed`, pixels from `displayed_surfaces`.
Lowering then compared a measurement of the client's buffer against the size of
the compositor's copy and ended the session when they differed.

They agree only while a surface has not resized, which is why the check
survived this long, and the comparison was never meaningful for that source
kind: a mirror member already draws every retained image at a size of its own,
and placement carries a copy to its head at whatever size the head wants. The
check now applies to CPU and DMA-BUF sources, where the plan measured the very
buffer being handed over and a difference means the wrong one arrived, and not
to renderer images, whose size is their own fact. During a resize the head
therefore shows the previous frame at the new geometry for one flip, until the
Present carrying the new buffer lands -- which is what a resize looks like
everywhere.

Worth noting how this became reachable. Composing the scene for every output
is what let the extended surface present at all; with two surfaces presenting,
a repaint can now find a retained surface whose committed content is newer
than any composition. The defect was always there. Nothing had been able to
reach it.

<!-- END IMPORTED BODY -->
