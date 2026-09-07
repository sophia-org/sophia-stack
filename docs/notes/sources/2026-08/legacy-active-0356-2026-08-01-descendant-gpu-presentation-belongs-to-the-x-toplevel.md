---
id: legacy-active-0356
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-01: descendant GPU presentation belongs to the X toplevel

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10994–11019. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next physical run retired Firefox's 1280-by-1040 DMA-BUF frames instead
  of rejecting them, but retained each frame as child surface 8388621 at global
  origin. Its managed parent occupied the left 1276-by-1422 pane. The child
  therefore rendered outside the parent placement, overlapped Kitty's hit-test
  region, and a browser click transferred focus and later key delivery to
  Kitty surface 6291472.
- X child geometry is relative to its parent. The X authority already flattened
  descendant software drawing into the root-child presentation surface, but
  standard DRI3 Present bypassed that reduction. DMA-BUF Present now walks the
  retained X hierarchy, targets and advances the managed toplevel transaction,
  translates damage by the accumulated child offset, and exports that reduced
  offset to the protocol-neutral live scheduler. Engine and the renderer still
  receive only surface, geometry, buffer, and damage facts.
- The renderer now intersects a mismatched source with both the surface clip
  and the actual pixel-sized target. A 1280-by-1040 frame in the 1276-by-1422
  pane therefore remains unscaled at the toplevel origin and clips to
  1276-by-1040 rather than claiming a 1276-by-1422 clip. Retirement evidence
  reports unit scale from source-versus-target size, independent of clipping.
- Focus and input now operate on the same managed surface that owns the visual
  transaction; the X input writer remains responsible for selecting the exact
  descendant window. Focused hierarchy and clipping regressions plus the full
  offline all-feature workspace gate pass. A fresh physical Firefox workflow
  remains required before Milestone 10 advances.

<!-- END IMPORTED BODY -->
