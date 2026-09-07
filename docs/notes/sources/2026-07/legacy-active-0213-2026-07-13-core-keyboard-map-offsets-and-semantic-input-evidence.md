---
id: legacy-active-0213
date: 2026-07-13
recorded_date: 2026-07-13
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-07-13: Core Keyboard Map Offsets And Semantic Input Evidence

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7191–7216. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The X13 stability workload exposed a false-positive input proof: xterm visibly
echoed repeated `^@` control notation, never printed its `received:` result, but
the session passed because fourteen events flushed and later pixels changed.
The `GetKeyboardMapping` decoder read the request padding byte as
`first_keycode` and the real first-keycode byte as `count`. Xterm therefore
cached keysyms for keycodes 0 through 7 while Sophia delivered normal core
keycodes such as 39 for `s`; Xlib translated every delivered key to NUL.

The decoder now reads the protocol body fields at bytes 4 and 5. Both wire byte
orders have regression coverage, and the real-xterm input smoke requires its
shell to receive exactly `sophia`. The live proof likewise uses an owner-only
result channel and emits schema 11 only after exact terminal bytes, flushed
delivery tokens, changed focused-surface pixels, and presentation all agree.
Pixel change alone is no longer input evidence. Kitty remains a separate
Wayland client proof; its X11 mode needs modern extension coverage beyond this
core-keyboard regression.

On X13, the standalone real-xterm smoke and all ten native repetitions reported
exact six-byte `sophia` receipt with no `^@` substitution. Nine native runs
exited cleanly. The tenth emitted complete schema-11 presentation and cleanup
records, then glibc reported `corrupted size vs. prev_size` during process
teardown. That preserves the allocator lifetime issue as a separate unresolved
bug; it does not weaken the now-semantic keyboard result.

<!-- END IMPORTED BODY -->
