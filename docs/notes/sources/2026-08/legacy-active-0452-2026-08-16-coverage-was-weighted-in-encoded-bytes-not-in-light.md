---
id: legacy-active-0452
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-16: coverage was weighted in encoded bytes, not in light

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13579–13613. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Attempt `0024` on `2273b550` completed without a crash and validated the
  whole pipeline. Merging engaged as designed — 76 authority batches became 9
  compositions, `merged_batches=68`, `max_merge_run=48` — and the raster
  round-trip closed: a requirement raised at content generation 48 was answered
  from generation 72, accepted, with zero sampled fallbacks and zero stale
  responses. The final held frame on DP-2 selected `density_millis=750
  sampling=exact` from a real second variant. The gate still failed visual
  confirmation: the operator described the text as fuzzy, like out of focus,
  soft with grey-edged glyphs rather than blocky or stair-stepped.
- That description is antialiasing, not a selection or pipeline defect, and it
  led to a real bug. `blend_copy_pixel` mixed gamma-encoded channel bytes
  arithmetically, and `blit_projected_image` averaged them the same way. Half
  coverage of white over black therefore produced about 128, roughly a fifth of
  the intended luminance rather than half. Every stroke narrower than a pixel
  was uniformly under-weighted, which is exactly what reads as out of focus.
- Fix: weight coverage as light. Components are squared before mixing and
  square-rooted after, in integer arithmetic, so replay stays bit-reproducible;
  a true transfer function needs a power the platform may round differently,
  which would put deterministic pixel gates at risk. Full and zero coverage map
  to the endpoints exactly, so canonical-density text remains bit-identical to
  the 1x drawable.
- Open, and larger than the bug: the gate proves per-head density with xterm at
  `-fn 6x13`, a fixed bitmap font. At 750 density its stems are 0.75 pixels
  wide, so no stem can land on a whole pixel and some softness survives any
  correct weighting; binary thresholding instead would produce the blocky
  result the same criterion rejects. For a bitmap font, exact-density replay
  and downsampling are also nearly the same operation, so the architecture's
  advantage is small for precisely this content. Vector content re-renders
  crisply at a native density and would demonstrate it plainly. If the
  corrected weighting does not reach the visual bar, the choice is between
  vendoring additional fixed font sizes so a derived store renders at a size
  matching its density, and changing what the gate measures.

<!-- END IMPORTED BODY -->
