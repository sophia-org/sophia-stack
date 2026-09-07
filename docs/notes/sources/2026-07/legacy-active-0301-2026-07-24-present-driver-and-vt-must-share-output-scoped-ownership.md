---
id: legacy-active-0301
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "architecture"]
---
# 2026-07-24: Present Driver And VT Must Share Output-Scoped Ownership

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9337–9357. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical run still showed only the hardware cursor. Output-correlated
logs proved the async coordinator retired the primary independently, but the
Present driver retained a second global in-flight early return. The secondary
output therefore continued to veto mixed composition after the coordinator had
correctly admitted it.

The same run left typed `ll` in tty3's input queue, where it appeared after
greetd returned. The Kitty launcher saved and restored KD and termios state but
never entered KD graphics or raw/no-echo mode. That left the console line
discipline active underneath native scanout.

The Present driver now consumes the same tested output-state reduction as the
service coordinator and blocks only on primary in-flight or cleanup state.
After the independent emergency guard is armed and immediately before starting
Sophia, the launcher switches to KD graphics and `stty raw -echo`; its existing
cleanup restores the exact saved KD and termios state on normal, failed, signal,
and emergency exits. Regression tests retain the guard-before-takeover order
and exact restoration commands.

<!-- END IMPORTED BODY -->
