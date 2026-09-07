---
id: legacy-active-0338
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "tooling"]
---
# 2026-07-26: declared constraints must fence blind-WM proposals before configure

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10635–10652. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The traced fixed-extent run completed cleanly and proved that all Present
  rejection paths emitted Complete/Skip, Idle, and actual xshmfence signals.
  Vkcube registered three DMA-BUF images and six fences but stopped submitting
  after Sophia initially configured its fixed 500x500 surface as a 1276x1422
  tile and later recovered it.
- `WM_NORMAL_HINTS` is advisory and xmonad's default tiled layout does not
  enforce it. Treating an external WM proposal as authoritative client size
  therefore lets WM policy violate application constraints.
- `LayoutEpochCoordinator` now reconciles content geometry and configure sizes
  against Engine-owned declared constraints before control delivery. Placement
  remains WM-selected, but constrained extents are clamped and kept inside the
  output work area. Impossible constraints are rejected explicitly.
- This is protocol-neutral Engine policy: it applies to every WM bridge and
  fixed/min/max constrained surface, with no Vulkan, vkcube, or xmonad identity
  in the decision.

<!-- END IMPORTED BODY -->
