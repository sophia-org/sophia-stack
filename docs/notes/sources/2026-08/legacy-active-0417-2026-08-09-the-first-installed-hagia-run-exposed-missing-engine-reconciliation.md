---
id: legacy-active-0417
date: 2026-08-09
recorded_date: 2026-08-09
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "tooling"]
---
# 2026-08-09: the first installed Hagia run exposed missing Engine reconciliation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12669–12693. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Installed Hagia attempt `0001` reached both physical outputs and admitted
  Kitty, then failed readiness at `layout_pending`. The immutable archive shows
  a public-policy transaction expecting `2560x1440` while Engine retained the
  coherent `1323x1424` admission extent. The transaction timed out, startup
  failed closed, all processes drained, exact TTY state returned, and the
  ledger classified the run as failed rather than erasing it.
- The API-v7 compatibility path reconciled every blind-WM transaction through
  `LayoutEpochCoordinator` after priming an Engine-owned recovery extent. The
  public-policy path primed the same extent but staged Hagia's unreconciled
  proposal directly into both the reducer successor and frontend layout. It
  therefore asked the client for pixels that contradicted Engine's temporary
  constraint and could not commit the startup frame.
- Public proposals now pass through the same protocol-neutral constraint
  reconciliation before reducer staging. Reconciled outer geometry and any
  explicit content-size request remain one candidate, while an omitted public
  content request stays omitted. Focused regressions reproduce the physical
  `2560x1440` to `1323x1424` correction and guard optional-size semantics.
- Kitty launch now disables its remembered OS-window size for deterministic
  managed-session admission. The bounded live Hagia regression observes a
  real constraint adjustment, reaches readiness, survives populated-checkpoint
  replacement, and ends with no layout, protocol, application, or cleanup
  debt. A fresh installed physical run remains the acceptance proof.

<!-- END IMPORTED BODY -->
