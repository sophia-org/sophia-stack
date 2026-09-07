---
id: legacy-milestone-0011
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-07-24 Physical Kitty Recovery And Input Promotion

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 265–292.

<!-- BEGIN IMPORTED BODY -->

- [x] Added a guarded two-output Kitty-only TTY3 launcher with independent
  Ctrl-Alt-Backspace recovery, exact KD/keyboard/termios restoration, display
  manager handoff, libinput `seat0` discovery, and bounded startup readiness.
- [x] Established the direct Mesa GLX, DRI3/Present, mixed-composition,
  page-flip feedback, and classic hardware-cursor path required by real Kitty.
- [x] Fixed Present identity, selection, retirement, multi-output scheduling,
  cursor ownership, focus, event-selection, and TTY keyboard-ownership races
  found by repeated physical attempts.
- [x] Added a strict real-Kitty input gate requiring exact shell input and a
  later Present rather than treating socket writes as proof.
- [x] Found the final keyboard failure in X11 extension event allocation:
  advertising GLX event base zero made libX11 install GLX converters over core
  KeyPress and KeyRelease. Traditional extension ranges are now non-core and
  mutually disjoint.
- [x] Retained a passing physical TTY3 run with visible Kitty, routed keyboard
  and pointer buttons, 13 ms maximum cursor motion-to-submit, status-zero Kitty
  exit, clean protocol health, and normal originating-TTY restoration.
- [x] Made display-manager takeover wait for all graphical clients, ignore
  zombies, and terminate only exact lingering graphical PIDs before failing
  closed.

Commit `eb509a6` is the promotion point. This milestone proves the physical
Kitty-only session; it does not promote the physical xmonad/Firefox desktop.

---

<!-- END IMPORTED BODY -->
