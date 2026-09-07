---
id: legacy-active-0309
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-07-24: Physical Promotion Needs Action And Output Evidence

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9627–9648. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The original physical xmonad verifier proved that xmonad committed some layout
and focus work, that two Kitty processes started, and that the session retired
native frames. Those aggregate facts could not distinguish the requested
focus, layout, workspace, close, click-drag, and two-output workflow from a
shorter interaction that happened to increment the same counters.

The live-session boundary now records the opaque action number after a
physically initiated WM proposal commits. This remains application- and
protocol-neutral: the Engine sees the same opaque policy action and no Kitty
identity. The physical verifier requires the focus-next, next-layout,
workspace-away, workspace-return, and close actions, two pointer-button
transitions, two terminal launches, two native outputs, and an independently
retired page flip on each output. Fixture mutations prove that missing
workspace, output-retirement, cursor, or click-drag evidence is rejected.

These records prove that the requested control path committed; they do not by
themselves prove that a hidden surface received no routed input. That
isolation claim remains a physical gate until delivery evidence correlates
input with the focused visible surface before and after workspace changes.

<!-- END IMPORTED BODY -->
