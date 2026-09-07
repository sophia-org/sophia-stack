---
id: legacy-active-0368
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-02: WM reseed must replay an uncommitted admission before relayout

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11290–11316. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The physical run after truthful X11 lifecycle delivery showed that
  `Super+F`, the session action, and Firefox process launch all succeeded.
  Firefox's first policy-managed surface timed out while proving its initial
  layout, which correctly restarted the speculative xmonad bridge. The restart
  then queued a relayout from only the last committed workspace. That state did
  not yet contain Firefox, so transaction 6 committed four visual layers but a
  policy projection of only the two existing Kitty surfaces. The later manage
  retry timed out and withdrew Firefox, making a working `Super+F` chord appear
  to launch nothing.
- A speculative bridge cannot recover an uncommitted `ManageSurface` through a
  committed-state relayout: the manage request is the state transition that
  registers the opaque surface in the WM workspace. Restart recovery now
  replays the oldest pending admission before considering an ordinary relayout.
  The choice is an allocation-free session reducer with crate-boundary tests,
  leaving X11 lifecycle in X Authority and visual proof in Engine.
- Schema-3 reseed evidence distinguishes `manage`, `relayout`, and `none`, so a
  future queue/batching optimization can preserve the ordering contract. A
  fresh physical Firefox launch remains the acceptance proof.
- Full-suite validation also exposed four stale wire-fixture assumptions left
  by the preceding lifecycle change: unselected create/configure/map events,
  Expose after a non-admission configure, and notification after a no-op
  configure. The fixtures now select the events they require, verify shared
  map state through `GetWindowAttributes`, bound blocking reads, and require
  silence for a no-op. The complete 166-case X11 wire target passes.

<!-- END IMPORTED BODY -->
