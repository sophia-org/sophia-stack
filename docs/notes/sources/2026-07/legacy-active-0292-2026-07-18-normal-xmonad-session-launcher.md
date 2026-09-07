---
id: legacy-active-0292
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "shell"]
---
# 2026-07-18: Normal xmonad Session Launcher

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9094–9111. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Milestone 7 is archived with its unattended QEMU gate retained as a frozen
regression. Its verifier now has a positive fixture plus negative mutations and
fails explicitly when any guest failure marker is present.

`sophia-live-session --session-mode=normal` now owns a bounded application
registry with explicit startup and named-action mappings. Applications are
spawned without a shell in dedicated process groups; normal exit is nonfatal,
and shutdown sends TERM to the group before a bounded KILL fallback. The
operator launcher selects xmonad compatibility policy, native Sophia WM policy,
or no external WM without placing a WM identity in Engine.

The first Milestone 8 QEMU gate starts one registered xterm, survives an
intentional bridge/xmonad restart with layout preserved, launches a second
registered xterm through xmonad's terminal action, closes it, logs out, and
retires two-output native presentation without cleanup debt. The frozen
Milestone 7 two-xterm gate also passes against the same build.
<!-- END IMPORTED BODY -->
