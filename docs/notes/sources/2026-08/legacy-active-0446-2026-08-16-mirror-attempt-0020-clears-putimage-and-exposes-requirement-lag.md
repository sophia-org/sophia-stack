---
id: legacy-active-0446
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-16: mirror attempt 0020 clears PutImage and exposes requirement lag

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13365–13405. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Signed source `f5b3b4855ccea02cc463812e5494a1e7d10d9bbb`, binary sha256
  `ade158fa…`, ran the unequal-mode mirror gate on DP-1 2560x1440 and DP-2
  1920x1080. Both heads composed, queued, submitted, and retired. The run failed
  visual confirmation: DP-2's text was fuzzy rather than sharp.
- The `PutImage` replay slice worked. The evidence contains zero
  `unsupported_put_image` records, so the journal retained xterm's startup
  upload and stayed replayable through later text, line, and scroll traffic.
  That was the attempt `0019` blocker and it is closed.
- The failure moved one layer out. Every raster requirement reported
  `cause=stale_dependency`, and no derived store was ever built. Because
  `satisfy` never ran, the surface's required-class set stayed empty, only the
  canonical 1x variant was ever published, and both heads selected
  `density_millis=1000`. DP-2 therefore downsampled the canonical handle 98
  times, which is exactly the observed fuzziness.
- Diagnosis: Engine builds each requirement from its committed scene
  (`raster_requirements.rs` `reconcile`), while X Authority admits it only when
  the requested generation equals the window's current generation. The authority
  advances a generation per draw and Engine commits at frame cadence, so a
  continuously drawing client leaves the authority permanently ahead. The lag is
  structural, not a race, which is why the failure was total rather than
  intermittent.
- Telemetry defect found in the same pass: generation mismatch and extent
  mismatch both reported `stale_dependency`, so the evidence alone could not say
  which check fired. They are now distinct causes, `stale_content_generation`
  and `logical_extent_mismatch`, and a stale generation logs the authority's
  observed generation beside the requested one.
- A deterministic regression now reproduces the field condition: draws advance
  the authority past a requirement built against an older committed generation,
  the cause is `stale_content_generation` with observed above requested, and the
  identical requirement rebuilt at the current generation is satisfied. That
  isolates the lag as the sole remaining blocker without needing hardware.
- Open decision: whether requirement admission should keep exact-generation
  equality. `apply_surface_raster_requirements` builds a complete replacement
  transaction and advances the generation rather than amending committed
  content, so satisfying against current state and reporting the generation
  actually produced may be the correct invariant. The alternative is retaining
  per-generation journal state so the authority can replay a requested
  generation exactly.

<!-- END IMPORTED BODY -->
