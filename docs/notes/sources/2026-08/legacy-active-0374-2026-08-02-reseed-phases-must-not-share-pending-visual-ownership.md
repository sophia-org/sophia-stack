---
id: legacy-active-0374
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-08-02: reseed phases must not share pending visual ownership

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11434–11462. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The immediate live repeat proved `Super+F`, action commit, process launch,
  surface discovery, and xmonad's two-phase request order all succeeded.
  Firefox still failed admission after a second restart. The first reseed
  relayout armed Firefox's selected 1280-by-1040 Present even though its WM
  request contained only the two committed Kitty nodes; the queued manage
  response then had no quarantined transaction left to stage and timed out.
- The leak occurred after the WM boundary. `proposal_from_response` applied a
  valid response to `planning_layers()`, which appended every unresolved
  surface independently of the response's candidate `WmWorkspaceState`.
  Committing phase one also discarded the excluded surface's unmanaged/retry
  bookkeeping. X Authority had supplied correct pixels and configure
  acknowledgements, while the fresh xmonad smoke correctly modeled policy
  ordering but did not exercise this visual-admission lifecycle.
- WM response projection now begins with committed layers and adds only
  planning surfaces assigned by that response's candidate workspace state.
  Committed relayout therefore leaves Firefox's admission state, retry count,
  and pre-admission authority group untouched; the following manage response
  alone releases and arms the exact candidate. Unmanaged admission ownership
  survives unrelated commits until candidate assignment, withdrawal, or
  removal.
- The retained lifecycle regression reproduces two committed surfaces, one
  pending PresentedBuffer, a recovery extent and standing target, phase-one
  relayout, and phase-two manage through exact retirement. The physical
  verifier accepts direct admission for future optimization; when a restart is
  observed it requires committed-layout/manage ordering, forbids phase-one
  arming, requires replay arming and retirement, and rejects a second restart.

<!-- END IMPORTED BODY -->
