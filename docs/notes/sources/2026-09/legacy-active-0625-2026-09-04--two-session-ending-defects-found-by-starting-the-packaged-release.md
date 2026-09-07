---
id: legacy-active-0625
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "tooling"]
---
# 2026-09-04 — Two session-ending defects found by starting the packaged release

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20099–20136. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installing and starting `0.1.0-417e97d2e25b` ended twice before reaching a
desktop, and neither cause was visible from the failure it printed.

`DeviceConfigurationFailed` came from libinput, not from output. Pointer
preferences are applied to every device reporting a pointer capability, and any
error failed the whole seat. libinput answers `Unsupported` for a knob a device
does not have and `Invalid` for a value it refuses; only the second is a
configuration failure. This seat has a composite HID exposing joystick, keyboard
and mouse on one interface with no acceleration profile, so `accel-profile
"flat"` ended every login for a preference that device was never able to hold.
The error named neither the device nor the setting, which is why finding it took
a source trace rather than a log line.

The second attempt reached the compositor and died on `Sophia rejected Hagia's
policy configuration`, repeated until the supervisor gave up. Six bindings
spelled their triggers as X11 keysyms — `bracketleft`, `comma`, `minus` and
their siblings — where the shortcut authority resolves the characters
themselves. `resolve_public_shortcuts` answered "shortcut trigger has no evdev
identity" and rejected the configuration. Hagia had validated that the trigger
used permitted characters and never that it named a key, so `hagia config check`
reported the profile as valid every time, and the same six were in the shipped
default, which would have failed the packaged-promotion session identically.

Both are fixed. `Unsupported` is counted and skipped, the count is reported
beside the input pipeline, and a fatal configuration names the setting it
refused. Hagia carries the bindable trigger names as data mirrored from Sophia's
table and refuses an unbindable chord at `config check`.

A third finding was not a defect: the requested output topology was validated
against the kernel and then declined, because the executor startup runs has no
apply by design. The apply path already existed as `sophia native-topology
apply` uses it; the session simply never called it. It now runs after scanout is
presenting, since a modeset needs a framebuffer sized for the mode it sets. A
refusal there is logged and the session continues on the topology already on
screen.

<!-- END IMPORTED BODY -->
