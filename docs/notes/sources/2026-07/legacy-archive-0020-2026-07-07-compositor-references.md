---
id: legacy-archive-0020
date: 2026-07-07
recorded_date: 2026-07-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-07: Compositor References

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 357–372. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Niri is a reference, not a base. Use it for Rust/Smithay compositor mechanics:
backend structure, frame clock, KMS/libinput patterns, renderer integration,
transaction timeouts, and headless or visual test ideas.

Do not fork niri for Sophia. Niri's central state combines compositor and WM
policy, while Sophia needs an external WM policy process.

Picom is the X-side reference. Use it for XComposite/Damage handling, X window
tree mirroring, top-level/client detection, layer snapshots, render command
planning, and damage calculation across buffered layouts.

Do not copy picom's process architecture. Picom renders back into X. Sophia
should hand X-derived layer snapshots to a compositor that owns scanout.

<!-- END IMPORTED BODY -->
