---
id: legacy-milestone-0029
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-08-16 Multi-Monitor Per-Head Composition Foundation

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 1020–1044.

<!-- BEGIN IMPORTED BODY -->

- [x] Replaced primary-sized mirror composition with one immutable logical
  scene fanned out into a native-size `HeadCompositionPlan`, damage ledger,
  renderer target, framebuffer/import owner, KMS request, callback lane, and
  retirement owner for every opaque physical head. Mirror retirement joins the
  required head set without exposing connector identity to Engine policy or the
  WM.
- [x] Added prepare-all presentation cohorts, callback-only shutdown drain,
  partial-submit poisoning, retryable cleanup, head-loss failure, and typed
  forced-detach evidence. Startup, resume, CPU, retained, DMA-BUF, compositor,
  cursor, and damage paths use the common per-head lowering seam.
- [x] Added live `sophia_output_v1` topology proposals for independently chosen
  modes, scales, transforms, positions, and mixed mirror/extended membership.
  Candidate and rollback resources are prepared before ordered per-card apply;
  published Engine, X, WM, pointer, and input state changes only after every new
  logical output reaches its first-presentation barrier.
- [x] Added bounded authority-owned `SurfaceContentSet` variants and
  `SurfaceRasterRequirements`. X Authority retains a canonical drawable plus
  derived density stores and can replay clear/fill, line, rectangle,
  ImageText8/PolyText8, and same-drawable `CopyArea`; selection reports exact,
  downsampled, and upsampled paths honestly.
- [x] Retained the physical failure archaeology and implementation decisions in
  `docs/research-log.md`. Active work moved to the critical path in `todo.md`;
  `docs/multi-monitor-composition.md` remains the normative contract.
<!-- END IMPORTED BODY -->
