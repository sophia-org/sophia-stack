---
id: legacy-active-0223
date: 2026-07-12
recorded_date: 2026-07-12
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-12: Native Wayland Replaces The Kitty Compatibility Runtime

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7459–7483. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Sophia's production Kitty path now terminates a private Wayland socket through
the Smithay-backed Sophia Wayland Authority. Engine input routes and layer
records are protocol-neutral; keyboard focus and pointer hit-testing remain in
Engine, while the authority translates accepted routes into `wl_keyboard` and
`wl_pointer` delivery. A real Kitty 0.47.4 process completes the headless smoke
with `DISPLAY` removed, changing nonzero SHM frames, and no X server process.

The installed launcher now uses the native Wayland/KMS session and retains the
independent Ctrl-Alt-Backspace recovery interlock. XLibre is excluded from the
production dependency graph and launcher; its frozen crate, CLI, patches,
scripts, fixtures, and notes live under `research/xlibre`.

The native-scanout session advertises a bounded single-plane linear/implicit
XRGB8888/ARGB8888 DMA-BUF subset. Accepted buffers cross the renderer boundary
as owned descriptors. Their experimental native import/presentation route is
now gated by the controlled first-frame/lifetime proof; arbitrary Kitty buffers
need GPU composition before they can enter this route. It is not yet recorded
as passing hardware evidence. Wayland
presentation and buffer-release feedback must remain withheld until the
matching KMS submission is observed as presented. The next evidence gate is the
controlled proof, followed by text, navigation, pointer, resize, sub-100 ms
presentation, clean exit, and TTY recovery in real Kitty.

<!-- END IMPORTED BODY -->
