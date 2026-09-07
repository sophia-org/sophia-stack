---
id: legacy-active-0352
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-01: Firefox smooth wheel requires an explicit XI2 path

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10911–10936. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Core Button4-Button7 records reached Firefox's exact GTK content window and
  decoded correctly in Xlib, but the local page never observed a DOM `wheel`
  event. Firefox links `gdk_disable_multidevice` and leaves GTK on its legacy
  core-device manager unless `MOZ_USE_XINPUT2=1` is set.
- The X `QueryExtension` reply was verified against the installed Xlib/XCB
  protocol headers: `present`, major opcode, first event, and first error occupy
  bytes 8 through 11. An experimental shifted layout made XKB unusable and was
  reverted before the final implementation.
- Sophia now advertises XI2 2.1 master-pointer horizontal and vertical valuator
  plus scroll classes, retains cumulative v120 positions in the X frontend,
  encodes them as XI2 motion valuators, and resolves selections through the
  target window's ancestor chain. Engine routing remains protocol-neutral.
- Firefox's XI-only key selection exposed a separate bounded-backpressure bug:
  the input writer waited five seconds for a core key selection even when an
  XI2 key event was selected. XI selection now satisfies writer readiness, with
  a regression proving that it bypasses only the legacy startup wait.
- The QEMU harness isolates the Firefox surface before both PRIMARY middle-click
  and wheel stages. The clean instrumentation-free run completed all eight
  Firefox stages and emitted
  `sophia_qemu_firefox_m8 schema=3 status=scroll_complete source=wheel axis_route=true keyboard_fallback=false`,
  followed by normal application, authority, renderer, KMS, input, and VM
  teardown. Chromium remains an independent compatibility follow-up because it
  is not installed on the current host.

<!-- END IMPORTED BODY -->
