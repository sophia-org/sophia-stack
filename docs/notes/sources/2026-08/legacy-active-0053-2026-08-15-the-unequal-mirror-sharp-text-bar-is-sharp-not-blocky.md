---
id: legacy-active-0053
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: the unequal-mirror sharp-text bar is sharp-not-blocky

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1643–1658. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first tty4 run on the head-identity commit proved the rekeyed path end
  to end (head-keyed readiness, joined generations, active sharp downscale,
  zero fallbacks) and failed only at the operator's visual confirmation: the
  1080p head's terminal text is soft. That softness is the mathematical floor
  of fractionally downscaling 1px bitmap glyphs, not a filter defect.
- Decision: the gate's `scaled_text=sharp_not_blocky` confirmation means what
  it says — no blocky resampling artifacts — and is the honest bar while
  client content is a single raster. Native-sharp client text on a 0.75x head
  is out of reach for sampling, for per-head composition (client rasters stay
  single-variant), and for integer-scale authority variants alike; it would
  require glyph rasterization at fractional density, which is deliberately
  not on the current roadmap. Operators who want pixel-exact text on the
  smaller head can configure a same-mode 1920x1080 mirror instead.

<!-- END IMPORTED BODY -->
