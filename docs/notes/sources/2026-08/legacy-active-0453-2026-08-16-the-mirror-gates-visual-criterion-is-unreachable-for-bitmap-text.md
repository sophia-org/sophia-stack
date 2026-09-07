---
id: legacy-active-0453
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-16: the mirror gate's visual criterion is unreachable for bitmap text

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13614–13653. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Attempt `0025` on `31fed705` carried the corrected coverage weighting and
  looked unchanged to the operator: still fuzzy, still out of focus. The
  telemetry was clean — DP-2 selected `density_millis=750 sampling=exact` on
  the held frame, zero sampled fallback, zero stale responses, the burst
  committed in 9 cycles.
- Measurement rather than judgement settled it. The final composed frames
  report `nonzero_rgb_pixels=10036` on DP-2 at 1920x1080 against `9668` on
  DP-1 at 2560x1440: the smaller head lights more pixels than the larger one
  while showing the same content, about 1.85 times the ink density. Crisp
  content scaled down lights fewer pixels. Every stem is being spread across
  neighbours as partial coverage, which is fuzziness measured directly.
- Cause, and it is not a defect: 0.75 is close to the worst ratio for a
  fixed-cell bitmap font. A six-pixel cell becomes 4.5 pixels, so alternate
  characters straddle pixel boundaries, and a one-pixel stem covers three
  quarters of a pixel. No weighting recovers a stem that never occupies a whole
  pixel; thresholding one crisp produces the blocky result the same criterion
  rejects. Fuzzy and blocky are the only outcomes available for this content at
  this ratio.
- Retracted: an earlier suggestion to vendor additional fixed font sizes so a
  derived store renders at a size matching its density. It cannot work for a
  terminal at 0.75, because the advance is 4.5 pixels and no integer-width font
  sits on that grid without drifting across the line or changing the column
  count. Per-density font selection helps at integer ratios only.
- Retained as deterministic evidence rather than argument: replay keeps a
  one-pixel line fully lit at 0.75 where area-resampling the canonical raster
  reaches no fully lit pixel at all, and replayed versus resampled 6x13 glyphs
  land within a few levels of each other. Both comparisons use the same area
  rule, so they isolate the source of the pixels rather than the choice of
  filter. That is the architecture's benefit and its boundary, stated as a
  measurement.
- Two structural conclusions. A mirror always resamples for at least one head,
  because one client renders one raster at one size; the unequal-mode mirror is
  therefore the least favourable demonstration of per-head composition. And
  exact replay needs semantic content, which modern toolkits do not provide —
  they rasterize text themselves and upload pixels. Visual acceptance of
  native-density rendering moves to the extended topology, where a window is
  rendered at its own head's density and nothing is resampled.

<!-- END IMPORTED BODY -->
