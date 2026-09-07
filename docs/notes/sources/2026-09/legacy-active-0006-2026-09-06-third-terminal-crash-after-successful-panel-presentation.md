---
id: legacy-active-0006
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "shell"]
---
# 2026-09-06: third-terminal crash after successful panel presentation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 175–220. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical session ran Sophia `4eb1136a` with Hagia `875c8c2` at
10:41:40 UTC. Evidence is preserved in `/tmp/sophia-third-terminal-4eb1136a`
(session/lifecycle/launch logs and installed manifest). Both Quickshell surfaces
retired on their respective outputs, terminal content stayed at height 1390,
and the two action-launched terminals reached `status=admitted`. The previous
invisible-panel and first-frame supersession failures did not recur. The
built-in 14-pixel indicator and 32-pixel X11 panel were both visible; this is
not full panel interaction or scrolling acceptance.

The session then failed with `retired Present lost its staged renderer
snapshot`. Layout 18 moved surface 6291470 from x=1919 to x=3187 on output 1,
whose right edge is x=2560. The final queued frame was 203 / scene 1636; its
logical checksum matched the preceding retained scene. The fatal lacked the
surface, transaction and image identities, so the log cannot independently
identify its missing image.

A deterministic reproduction using those rectangles exposes an admission gap:
old bounds keep output 1 in the repaint cohort, while Engine clips the current
surface entirely out of that head. Policy correctly excludes output 2. Neither
physical head of a mirrored output then lowers the candidate DMA-BUF, so no
renderer snapshot can be staged, but the runtime still assigned the repaint a
copy-Present retirement. This is consistent with the physical failure and is a
reachable source-level reproduction of its missing-snapshot condition.

Present admission now checks the lowered physical frames for the exact candidate
DMA-BUF image. If no head captures it, Sophia queues the clearing frame as an
ordinary repaint and returns Skipped feedback through the existing rejection
path, releasing ordered surface-content ownership. Partially visible windows
still present normally. The check runs after clipping, scaling and translation;
raw rectangle overlap cannot substitute for actual frame membership. Unexpected
snapshot failures remain fatal and now name transaction, surface, image, output
and frame. The submitted owner remains available to bounded cleanup if snapshot
promotion fails.

Focused tests cover the old-bounds-only repaint, both mirror heads, partial and
one-pixel visibility, and wrong-image rejection using the real Engine planner
and renderer lowering. The next physical check is ordinary terminal insertion
and navigation with the panel running, including scrolling an older window out
of view and back. No comparison campaign or unrelated milestone reset is needed.

Validation: `cargo xtask check` passed with isolated temporary/config directories
(2,438 Rust test executions, Clippy, layout debt, reader and archive checks).
Physical acceptance of this additional repair remains pending.

<!-- END IMPORTED BODY -->
