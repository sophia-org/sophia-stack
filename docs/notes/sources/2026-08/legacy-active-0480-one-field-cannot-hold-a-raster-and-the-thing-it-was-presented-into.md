---
id: legacy-active-0480
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: d702937b4b145e4acfdeb0fbbc4208a656479ece
committed_at: 2026-08-21T10:21:19-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# One field cannot hold a raster and the thing it was presented into

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14573–14623. The heading has no date. Its first recorded addition is commit
`d702937b4b145e4acfdeb0fbbc4208a656479ece` (2026-08-21T10:21:19-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Three fixes for `SourceSizeMismatch` addressed real defects and left the crash
standing, because each corrected a *copy* of a number whose original was
invented. The original is here:

```rust
// runtime/render_resources.rs, the DRI3 present path
let target_content_size = Size { width: record.geometry.width,
                                 height: record.geometry.height };  // the window
// pixmap_size was measured thirty lines earlier and discarded
```

That value becomes `SurfaceContentSet::logical_extent`, and `singleton` copies
it into `pixel_size` as well, so a transaction asserts a raster of whatever
size the window happens to be. The engine commits the assertion verbatim
(`authority_transaction.rs`), the head plan reads it as the texture extent, and
the renderer compares it against the buffer it was handed and ends the session.

The model already forbade what was being asserted. `SurfaceContentSet::new`
validates `pixel_size == ceil(logical_extent * density / 1000)`, so a 1280x1440
raster under a 1920x1080 extent is not expressible at any density -- and
`singleton` was constructing `Self { .. }` directly, so the one check that would
have caught it never ran. It builds through `new` now.

What made the obvious fix fail twice is that the same conflation is load-bearing
somewhere else. The resize gate asks "did the client fill what it was asked to
fill?" and answered it by comparing the registered buffer against the declared
extent -- which works only while the declaration is the window. Declaring the
pixmap instead makes that comparison vacuous, and a stale buffer reads as
satisfied: the gate opens early, quietly, which is worse than the crash.

So the transaction states both facts. `content` is the raster and spans what it
says it spans; `presentation_extent` is what the authority filled. They are
equal for a client that has answered its configure, differ for one that has
not, and for an authority projecting a descendant content window onto a larger
policy-managed surface the second is that descendant -- which is why an inset
present still proves its outer extent while a stale present does not. The gate
now compares a measurement against a measurement.

A stale present therefore commits truthfully and is placed and scaled into the
surface geometry for a flip or two, which is what an X server does with a
mismatched Present. Present feedback is untouched: it is bound to the
submission lifecycle, not to the commit.

Two things found on the way. `present_pixels_conflict_with_requested_sizes` had
been dead since it was written -- referenced only by its own test -- and is
deleted. And `sampling_class` compares densities only, so a raster that does
not span its geometry is still reported `Exact`; that is a separate defect,
now on the roadmap, and the mixed gate reads that field.

<!-- END IMPORTED BODY -->
