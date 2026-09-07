---
id: legacy-active-0193
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy"]
---
# 2026-07-26: Pending Pixels Must Not Replace WM Geometry

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6581–6642. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The latest normal xmonad capture exposed a temporal geometry defect that the
four-Kitty end-state verifier did not cover. After ManageSurface transaction 3
committed the two-window layout, the new surface continued to present at its
admission staging position, `1280x1426_80_60`. The existing surface presented
at its correct right-hand tile. A later Super-J focus transaction moved the new
surface to `1280x1426_0_14` without a configure, after which the geometry
remained stable. The three-window sequence reproduced the same pattern.

A strengthened real-xmonad smoke now manages three opaque layout nodes
sequentially with the physical `2560x1426_0_14` work area. It requires the
exact full-height master and two `1280x713` stack panes. The smoke passes,
proving that xmonad and the compatibility bridge return every committed
placement.

The first correction made Present admission consume the same immutable Engine
presentation layout used for composition. This prevents a newly queued buffer
from missing reprojection merely because it did not exist at the beginning of
the cycle. A following physical run proved that correction for the second
window, then reproduced the defect with the third window on workspace 2.

That topology exposed the underlying resize-quarantine race. The third window
already had the requested full-height pixel size, while the two existing
windows needed resize redraws. An ordinary third-window Present arriving while
those redraws were pending was therefore an unrequested layout observation.
The pending-layout reducer replaced the whole proposed layer with that
observation, including its `(80,60)` staging geometry and old stack rank. It
preserved new pixels by silently discarding the WM placement.

Unrequested policy-managed observations now merge only authority-owned visual
content into an existing pending layer: source, damage, generation, identity,
opacity, crop, transform, and resize capability. WM-owned geometry and stack
rank remain from the proposal. Client-positioned surfaces explicitly retain
authority-owned geometry, so an xmobar update during a resize epoch is not
lost. Deterministic regressions cover both authority assignments.

The following 77-second physical run confirmed the authority split across
seven action-launched Kitty surfaces on workspaces 1 and 2. Each ManageSurface
commit moved exactly the projected surface count, and the new surface's first
retired Present used the work-area master geometry. Three-window layouts
presented one `1280x1426` master and two exact `1280x713` stack panes;
four-window layouts presented the master plus `1280x475`, `1280x475`, and
`1280x476` stack panes. No target used `(80,60)`, and every following layout
transaction reported `moved_surfaces=0`. The run balanced 524 mixed target,
pipeline, and frame-surface lifetimes, held input dwell to 11 ms and
submit-to-page-flip observation to 47 ms, recorded no unexpected protocol
error or WM degradation, and completed cleanly.

The work-area-aware four-Kitty verifier now checks this temporal boundary for
every action-launched surface it observes. It correlates launch observation,
ManageSurface commit, active-workspace projection, first retired Present, and
the following stability transaction. Mutation fixtures reject staging
geometry, mismatched pixel dimensions, and a second geometry change.

Focused borders remain a separate compositor-chrome concern. They must derive
from committed focus and committed surface geometry, enter the ordinary Engine
frame and damage lifecycle, and never compensate for or conceal a placement
error. The renderer-neutral chrome model in `docs/compositor-graphics.md`
already defines that boundary; implementation follows physical geometry
stability.

<!-- END IMPORTED BODY -->
