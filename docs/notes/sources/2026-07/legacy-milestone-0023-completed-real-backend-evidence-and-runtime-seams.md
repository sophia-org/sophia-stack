---
id: legacy-milestone-0023
date: 2026-07-09
recorded_date: 2026-07-09
date_basis: first-heading-commit
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
date_commit: 6125320e1155d430ec82008ec22bdd9d57735a25
committed_at: 2026-07-09T19:43:12-04:00
---
# Completed Real Backend Evidence and Runtime Seams

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 511–545.
Date from the first addition of this heading in commit `6125320e1155d430ec82008ec22bdd9d57735a25`
(2026-07-09T19:43:12-04:00); it does not date every event or later edit.

<!-- BEGIN IMPORTED BODY -->

- [x] Added `live-session-composition-smoke`, composing the Sophia X Authority
  Present-pixmap socket path, bounded authority batch intake, runtime commit
  projection, renderer-live frame-target observation, rendered primary-plane
  scanout submit, deterministic page-flip retire, and cleanup-drained reporting
  into one non-destructive reduced evidence line.
- [x] Proved libinput-shaped input polling, native page-flip retirement, and
  rendered scanout submit can share one runtime tick.
- [x] Added a runtime-owned readiness gate so concrete libinput dispatch runs
  only after the session loop observes reduced readiness.
- [x] Kept physical input intake separate from routed-input transformation with
  reduced `PhysicalIntakeOnly` runtime evidence.
- [x] Preserved deterministic queued poller tests as the default validation
  path while keeping native libinput behind optional feature tests.
- [x] Collapsed one-shot helper calls into a reusable session loop owner.
- [x] Fed reduced input, page-flip, and scanout facts through one bounded tick
  budget.
- [x] Kept real file-descriptor readiness outside Sophia Engine state via a
  reduced readiness collector.
- [x] Recorded opt-in real GBM/EGL validation with reduced draw, presentation,
  and frame-target allocation evidence.
- [x] Defined renderer-private GBM/EGL frame-target lifecycle states: created,
  retained, resized, invalidated, and retired.
- [x] Added reduced runtime observations for frame-target lifecycle and
  allocation without implicit native allocation during ticks.
- [x] Added the first reduced KMS scanout target report and derived page-flip
  readiness from it.
- [x] Preserved CPU fallback and degraded GPU paths while scanout matures.
- [x] Added a fake live compositor loop smoke covering input polling, authority
  transaction intake, WM policy, renderer target observation, frame commit, and
  reduced page-flip observation.

---

<!-- END IMPORTED BODY -->
