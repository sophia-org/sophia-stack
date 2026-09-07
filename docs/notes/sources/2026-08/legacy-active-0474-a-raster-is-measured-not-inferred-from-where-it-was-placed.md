---
id: legacy-active-0474
date: 2026-08-20
recorded_date: 2026-08-20
date_basis: first-heading-commit
date_commit: 110399d7ecae4b17d905dd57eb2d27a9f3da0a04
committed_at: 2026-08-20T19:06:48-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# A raster is measured, not inferred from where it was placed

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14378–14418. The heading has no date. Its first recorded addition is commit
`110399d7ecae4b17d905dd57eb2d27a9f3da0a04` (2026-08-20T19:06:48-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Three runs of the member-optimized gate produced three different fatals, and
the third named the real one: `SourceSizeMismatch { surface: 2097166, handle:
7, planned: 1920x1080, held: 1280x1440 }` -- both records describing the same
DMA-BUF. `held` was the imported buffer. `planned` came from the committed
content set, and that number had never been measured:

```rust
/// Builds a committed state around a single canonical raster whose
/// pixels span the geometry, which is every current producer's shape.
content: SurfaceContentSet::singleton(source, Size {
    width: geometry.width, height: geometry.height,
})
```

The stated invariant is true only of a producer that has answered the configure
that moved it. Between the configure and the redraw -- which for an X client is
a round trip through its own event loop -- the surface is placed at one size and
drawn at another, and the record asserted the placement as though the client had
reported it. Everything downstream inherited the invention: the head plan's
`source_pixel_size`, the sampling classification computed from it, and the
lowering check that compares a plan against the buffer it was handed.

`LayerSnapshot` and `OutputFrameSurfaceState` now carry `source_size`, and it
is filled from whoever measured the raster: the buffer registry in the live
session (`live_transaction_observed_size`, which already reported the physical
size when a client attached an old buffer under a new declared extent), the
committed content set in the backend projection, and the head binding in the
composition plan. Where a layer names no raster at all -- planning surfaces,
templates -- the field carries the geometry, because nothing samples it.

The mitigation that was on the table is now unnecessary, and would have been
wrong. Dropping the DMA-BUF size check would have removed the only consumer
that noticed the invention while leaving the sampling classification quietly
wrong. With the size measured, that check compares two independent
measurements of one buffer, which is exactly what it should have been doing.

One site keeps deriving size from geometry deliberately: `SurfaceSnapshot`,
where the raster is an X window's own pixmap and the two genuinely coincide.

<!-- END IMPORTED BODY -->
