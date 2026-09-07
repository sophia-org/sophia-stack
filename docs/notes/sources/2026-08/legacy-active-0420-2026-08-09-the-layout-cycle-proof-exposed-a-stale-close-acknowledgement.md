---
id: legacy-active-0420
date: 2026-08-09
recorded_date: 2026-08-09
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "validation"]
---
# 2026-08-09: the layout-cycle proof exposed a stale close acknowledgement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12757–12781. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first layout-cycle replacement run used X keycode labels for an evdev
  binding table. `Super+F` therefore committed the browser operation instead of
  fullscreen. The guide now uses the actual retained bindings: `Super+Y` for
  fullscreen, `Super+N` for layout cycling, `Super+I` for minimize, and
  `Super+R` for restore.
- The accidentally launched Chromium client could not initialize its GPU path
  and left a presented surface after its X Authority route disappeared.
  Pointer buttons remained fail-closed during the unresolved focus handoff.
  Closing that stale surface returned `UnknownSurface`; Sophia treated the
  expected teardown race as fatal even though the client was already gone.
- A rejected `CloseSurface` acknowledgement with `UnknownSurface` is now
  retired as a stale target, like `ClientGone`. Other command kinds still
  reject `UnknownSurface` as an error, so configure and focus failures cannot
  be hidden. Focused queue tests cover both sides of that distinction, and the
  complete `sophia-cli` suite passes.
- The corrected installed run on Sophia `09337bb2` passed the bounded physical
  policy gate and was archived as promotion record `0003`. The trace commits
  fullscreen and layout-cycle actions before the supervised restart, loads and
  reconciles the nonempty checkpoint in epoch 2, then commits fullscreen,
  layout-cycle, focus, minimize/restore, and output actions afterward. Exact
  text and pixel evidence passed, all native ownership drained, and all 20
  session controls completed without rejection, timeout, or cleanup debt.

<!-- END IMPORTED BODY -->
