---
id: legacy-active-0362
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-01: Firefox resize completion belongs to page-flip retirement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11125–11156. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The latest physical launch disproved both earlier geometry hypotheses.
  Firefox first retired its 1280-by-1040 admission frame, then produced an
  exact 1276-by-1422 candidate for the xmonad tile. A layout epoch cannot stage
  that candidate unless its observed pixels exactly equal the configured
  target, so mapped `ConfigureWindow` denial and configure delivery were no
  longer the failing boundary.
- Sophia nevertheless logged the layout epoch as committed before the native
  page flip completed. Later same-surface backing transactions then advanced
  the Engine generation, and the already prepared Present transactions were
  correctly rejected as `RejectedStaleSurface` at retirement. The white or
  clipped Firefox window was therefore a split logical/visual commit, not a
  missing browser resize.
- DMA-BUF resize observations now arm a bounded `(transaction, surface)` visual
  candidate instead of updating committed layout size. The standing target
  remains authoritative until an exact successful native retirement; old-size
  Presents remain rejected during that interval. The production runtime owns
  a bounded per-surface content fence while a Present is asynchronous, defers
  only later authority groups touching that surface, and rebases/releases them
  after either successful or controlled rejected retirement. Surface removal,
  native detach, failed submission, and shutdown have explicit non-deadlocking
  cleanup paths. Other surfaces continue independently.
- Runnable regressions prove multiple surfaces can share one layout transaction,
  mismatched or wrong-transaction retirements cannot clear a candidate, a
  resize Present reaches Engine generation 2 before its deferred update reaches
  generation 3, removals bypass the fence, and shutdown discards its backlog.
  The physical verifier now requires ordered `visual_armed` and matching
  `visual_committed` evidence and rejects a stale outcome between them. Offline
  targeted and all-feature live-session tests pass; the physical Firefox rerun
  remains the acceptance boundary.

<!-- END IMPORTED BODY -->
