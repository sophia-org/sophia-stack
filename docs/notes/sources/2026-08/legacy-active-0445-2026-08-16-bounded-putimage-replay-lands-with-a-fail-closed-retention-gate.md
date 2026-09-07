---
id: legacy-active-0445
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-16: bounded PutImage replay lands with a fail-closed retention gate

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13333–13364. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Accepted core and MIT-SHM `PutImage` now retains its own bounded pixels in the
  X Authority semantic journal and replays them per density class. This closes
  the boundary that attempt `0019` diagnosed; the physical gate must still be
  re-run to convert it into promotion evidence.
- Decision: retention is gated on an unconditional write rather than on the
  canonical writer's looser behavior. Only tight ZPixmap depth-24/32 rows with
  no left padding, drawn through GXcopy with a full visible plane mask and no
  clip rectangles, are retained; everything else poisons the journal with the
  `unsupported_put_image` cause. The canonical writer ignores the graphics
  context entirely, so a looser gate would also have been internally consistent.
  The strict gate was chosen because a retained command must reproduce the
  canonical drawable on replay, and a conditional write cannot promise that.
  The cost is a possible false negative if a real client uploads through a
  clipped or non-copy context.
- Consequence for the next gate run: a 750-density head that still reports
  `cause=unsupported_put_image` means the gate is too strict for the traced
  client and should be relaxed toward canonical semantics, whereas
  `cause=unsupported_cross_drawable_copy` means the next replay slice is
  required. The cause is therefore the first thing to read from a failed run.
- Replay projects the retained 1x pixels rather than resampling the canonical
  store, so ordering against later text and copy commands is preserved. Each
  destination pixel is a per-channel rational area average over the source
  pixels it covers, which keeps fully covered pixels exact and blends only at
  boundaries. At unit density the projection degenerates to full coverage of one
  source pixel, so a derived 1x store would reproduce the canonical bytes.
- Sampled fallback is now cause-classified and coalesced per surface, emitting
  the first occurrence and each subsequent power of two with a cumulative count.
  The cause stays authority-private; Engine still observes only
  `SurfaceContentFidelity`.

<!-- END IMPORTED BODY -->
