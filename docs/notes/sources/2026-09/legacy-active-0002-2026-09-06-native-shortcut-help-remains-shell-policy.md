---
id: legacy-active-0002
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "shell"]
---
# 2026-09-06: native shortcut help remains shell policy

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 47–92. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Implement the Triad-style helper as revision 3 read-only shell reference sheets,
with separate catalog and presentation capabilities and a 256-binding bound.
Narthex owns display text, grouping, ordering and style; Session supplies only
active binding facts, and Engine owns geometry, GPU/CPU composition and input
capture proven against the exact retired frame. Reference slots carry no action
capability. The overlay preserves application focus, work area and WM/camera
state. The generic contract and lifecycle are recorded in
[shell-reference-sheets.md](../../../shell-reference-sheets.md).

Keep the bundled JetBrains Mono NL as the shared presentation default. Compare
Triad `fb8fb27` with that same font; preserve its colors, spacing, border and
title, adding pages instead of truncating the configured bindings. Private
Narthex `hotkey-overlay { skip-at-startup #false; }` selects once-per-login help;
only that file is mounted into the shell domain. The session does not parse it.

The rendered regression exposed an existing CPU fallback omission: ARGB text
buffers were silently skipped. The fallback now blends them, while native GPU
composition retains its cached texture path. Tests also distinguish a new
read-only sheet from an older retired sheet with identical geometry and no
activation targets. Physical acceptance is pending a coordinated installation;
the live session has not been reloaded during implementation.

The text cache now admits 1,024 entries within the existing 16-MiB ceiling:
a full 64-row page has 129 text nodes and exceeded the previous 128-entry
ceiling, causing cyclic eviction on every repaint. A regression requires the
second full-page raster pass to add no misses. Long labels are clipped within
their column instead of pushing later bindings off screen. The 36-row visual
fixture matches Triad's 472×615 panel at (404,52) on a 1280×720 output with the
same font; glyph-edge antialiasing remains renderer-specific.

Your personal binding is staged in `/tmp/hagia-shortcut-help.kdl`; the installed
older parser cannot yet consume it. `/tmp/install-sophia-shortcut-help.sh` checks
that config, installs the coordinated signed build, then activates it only if
the current profile still matches the reviewed source. Source commits and the
normal session acceptance remain separate from this implementation.

Validation complete: `cargo xtask check` passes with 2,453 Rust test executions
and no failures (one pre-existing ignored test), both standalone Nim gates pass,
and Sophia, Hagia and Narthex release builds succeed. Formatting and diff checks
are clean. Evidence is retained in `/tmp/help-final-check.log`,
`/tmp/help-hagia-full.log`, `/tmp/help-narthex-final.log`, and the
`/tmp/{sophia,triad}-reference.png` comparison. No live reload or install was
performed.

<!-- END IMPORTED BODY -->
