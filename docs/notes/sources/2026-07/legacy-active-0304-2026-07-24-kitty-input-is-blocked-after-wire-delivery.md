---
id: legacy-active-0304
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-24: Kitty input is blocked after wire delivery

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9479–9497. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`x-authority-kitty-input-smoke` now reproduces the interactive failure without
owning a VT. It launches Kitty 0.48.0 with `--config NONE`, uses the routed
frontend and render node, waits for two DRI3/Present submissions, snapshots the
client-visible XKB map, focuses the mapped surface, and injects `ll` plus
Return. The gate only passes when the proof shell writes exactly `ll` and Kitty
submits another Present.

The retained diagnostic boundary is narrower than the earlier hardware
hypotheses: keycode 46 resolves to `l`; Kitty's mapped window selected core
KeyPress/KeyRelease; focus control is delivered; all six events are flushed to
the client socket; and Present Complete/Idle routing succeeds. The shell still
receives zero bytes and Kitty submits no post-input frame, while the existing
xterm input smoke passes the same brokered route. Physical libinput discovery,
console echo, Kitty configuration, and xmonad are therefore not the current
root boundary. Promotion remains blocked on Kitty consuming the X11 stream
across Present synchronization.

<!-- END IMPORTED BODY -->
