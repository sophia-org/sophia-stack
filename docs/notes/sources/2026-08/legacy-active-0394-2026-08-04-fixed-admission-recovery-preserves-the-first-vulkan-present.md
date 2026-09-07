---
id: legacy-active-0394
date: 2026-08-04
recorded_date: 2026-08-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11"]
---
# 2026-08-04: fixed admission recovery preserves the first Vulkan Present

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11957–11989. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Installed release `f007757a` preserved nonzero Present timestamps but did
  not restore vkcube animation. Three independent 500-by-500 launches followed
  the same path: the first Present selected the admission pixels, the blind-WM
  layout timed out, rollback drained that Present, and a CPU snapshot later
  made one static frame visible. No vkcube transaction reached native
  retirement.
- Live process inspection showed that vkcube had not crashed. Its main thread
  and FIFO WSI queue thread were parked on condition variables, its X event
  thread was waiting for another event, and its X socket had no unread data.
  The intervening 300-by-300 GLX workload retired 851 frames, isolating the
  failure to Vulkan's first-Present lifecycle rather than KMS, composition, or
  general DRI3 progress.
- Mesa's X11 WSI records each FIFO Present by serial and waits for that exact
  completion before queuing another image. XLibre reports Skip only after a
  pixmap or window ceases to exist; an ordinary non-flippable pixmap is copied
  and completed. Sophia instead destroyed a still-coherent admission Present,
  reported Skip, and displayed its copied pixels through a second transaction.
  Correct timestamps could not repair that split lifecycle.
- Layout rollback now reconciles each staged Present with Engine's fixed
  recovery extents. A managed resize or mismatched source remains stale and is
  rejected. An admission source whose descriptor exactly matches its recovery
  extent stays layout-fenced. It is released only after the recovered surface
  enters the committed presentation projection, and its previous generation
  is rebased to any CPU snapshot committed during admission. The preserved
  DMA-BUF can then retire normally through KMS and produce one coherent
  Complete/Idle lifecycle.
- Crate-boundary regressions require exact extent matching, continued deferral
  while the surface is absent, generation rebasing at visibility, normal
  eligibility afterward, and rejection of a one-pixel mismatch. The packaged
  physical vkcube rerun remains the acceptance boundary.

<!-- END IMPORTED BODY -->
