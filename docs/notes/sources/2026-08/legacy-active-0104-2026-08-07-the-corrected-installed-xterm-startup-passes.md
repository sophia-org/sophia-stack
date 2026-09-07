---
id: legacy-active-0104
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "tooling"]
---
# 2026-08-07: The corrected installed xterm startup passes

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3402–3419. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The installed `0.1.0-56dad4de8b5f` xmonad profile starts xterm without the
predecessor's `CreatePixmap(depth=1)` failure. Both native outputs present,
xmobar reduces their work areas, and xmonad commits the xterm at
`2556x1422_2_16` inside the `2560x1426_0_14` primary work area, including its
two-pixel Engine frame. Stable mixed scanout makes the session ready in 314 ms
with zero X11 protocol errors.

The same run quiesces cleanly for a switch to tty2, restores retained content
on resume, and commits `Super+Shift+Q` through the blind WM path. Presentation
drains with no abandoned scanout, session health and layout health are clean,
all native and X11 ownership reaches zero, the packaged normal-lifecycle
verifier passes, and KD mode plus termios are restored exactly. A host-level
check finds no Sophia, xmonad, xmobar, or xterm residue. This closes the narrow
startup regression only; the remaining focused application, layout, color,
automated-cycle, and soak gates stay open.

<!-- END IMPORTED BODY -->
