---
id: legacy-active-0293
date: 2026-07-22
recorded_date: 2026-07-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11"]
---
# 2026-07-22: Xmonad TTY3 Requires Independent Local Recovery

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9112–9138. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical native-xmonad TTY3 operator attempt produced blank scanout.
Sophia did not provide VT suspend/resume, the wrapper had no independent input
guard or explicit KD/termios restoration, and its documented recovery path
incorrectly depended on switching to another TTY. The operator had to reboot,
which also erased the `/tmp` launcher log.

The xmonad wrapper now applies the established guarded TTY lifecycle before
graphics takeover: one Ctrl-Alt-Backspace chord must arm the independent input
guard, a second chord stops the supervised session even if Sophia input routing
is wedged, and cleanup restores KD mode, termios, and keyd. Guard and recovery
records are durable under the user's state directory. Ctrl-Alt-Fn remains
unsupported until Engine owns a correct VT/DRM suspend-and-resume boundary.

The guarded rerun proved DRM ownership, two-output presentation, WM bridge
startup, and complete emergency restoration, but also retained
`startup_apps=0` and no live `key_observed` marker after Super-Enter. The normal
TTY wrapper had omitted `--session-start=terminal`. Independently, a blank
normal session represented its missing primary child as already exited; the
post-exit proof guard then compared two absent surfaces and suppressed the
entire physical-input poll, including global WM shortcuts. The session now
distinguishes full application routing, shortcut-only polling, and complete
suppression. Empty desktops admit emergency and registered WM chords without
delivering ordinary keys or pointer events to an unfocused client, and the
physical launcher starts Kitty by default.

<!-- END IMPORTED BODY -->
