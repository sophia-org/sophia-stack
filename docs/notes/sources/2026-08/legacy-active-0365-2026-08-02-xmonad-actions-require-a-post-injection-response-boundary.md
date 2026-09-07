---
id: legacy-active-0365
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "architecture"]
---
# 2026-08-02: xmonad actions require a post-injection response boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11201–11232. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The physical Firefox rerun rendered the browser at its exact 1276-by-1422
  tile and advanced to verifier stage 5 of 8. This physically confirms the
  exact-window Present-before-core `ConfigureNotify` fix. The remaining failure
  was `Super+Space`: action 3 committed as WM transaction 6 with four surfaces,
  `moved_surfaces=0`, and `configure_deliveries=0`.
- Engine correctly owned the physical chord and delivered only opaque action 3
  to policy. The defect was inside the xmonad compatibility adapter. It queued
  existing-node root `ConfigureNotify` reconciliation and private Mod1+Space in
  one response collection. The old Tall requests satisfied the expected-window
  set, and the 80 ms quiet fence could close the transaction before xmonad's
  post-key Mirror requests arrived.
- The bridge now drains and discards the complete pre-action reconciliation,
  validates that the supervised WM registered the profile chord, then injects
  the private key press/release and wake event and requires fresh WM activity.
  This remains a policy-adapter concern: X Authority still owns real X clients
  and Engine still owns physical input and opaque action authorization.
- The strengthened real-xmonad run first failed that grab validation and exposed
  an older core-wire defect. `GetKeyboardMapping` carries `firstKeyCode` and
  `count` in its request body, while the fake server read the header padding as
  `firstKeyCode`; its saturating exclusive range also serialized 247 mappings
  after advertising 248. Correct body parsing and complete inclusive keycode
  serialization let unmodified xmonad resolve Space and register Mod1+Space.
- A hermetic fake WM delays its Mirror requests beyond the former quiet period
  and proves the action returns only the post-injection geometry. It resolves
  Space through the full 248-entry keyboard mapping before registration. A
  second fixture omits `GrabKey` and proves fail-closed behavior. The real
  unmodified-xmonad smoke now requires the exact three-window Tall-to-Mirror
  transition, preserving a runnable reference boundary for future batching or
  event-loop optimizations.

<!-- END IMPORTED BODY -->
