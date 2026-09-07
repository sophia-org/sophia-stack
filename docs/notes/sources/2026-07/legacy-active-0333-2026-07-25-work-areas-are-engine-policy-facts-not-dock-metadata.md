---
id: legacy-active-0333
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-07-25: Work Areas Are Engine Policy Facts, Not Dock Metadata

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10477–10508. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The corrected mixed renderer made xmobar and Kitty visible together, then
exposed that Kitty still began at the root origin beneath the bar. Rendering
was healthy: the run completed 141 mixed exports with matching target,
pipeline, and frame-surface lifetimes and no native failure. The missing
mechanism was layout reservation, not another renderer special case.

The X frontend now decodes exact CARDINAL `_NET_WM_STRUT_PARTIAL` and legacy
`_NET_WM_STRUT` properties in client byte order. Valid partial data takes
precedence; legacy data is the fallback. The authority emits complete,
bounded `SurfaceOutputReservations` replacements keyed by Sophia `SurfaceId`.
Atoms, XIDs, dock types, titles, classes, and application identity stay inside
the frontend. Malformed values remain legal X properties but produce no
reservation.

Engine owns the lifecycle table and pure work-area reducer. Only mapped
`ClientPositioned` surfaces are active. Replacement, deletion, unmap, and
surface removal update the table; same-edge depths take the maximum, different
edges combine, and partial root spans clip independently against each output.
An empty aggregate is rejected so the session preserves its last valid work
area. The full output remains available for composition and pointer hit
testing.

The live WM owner now stores reduced bounds in `WmWorkspaceState` without
changing workspace or focus state. Manage and relayout requests use that
stored rectangle, and the existing WM compatibility bridge forwards it as its
synthetic root bounds. Consequently the implementation is generic across
native WMs and legacy profiles; neither Engine, renderer, live session, nor
bridge contains an xmobar, xmonad, Kitty, or toolkit condition. Physical
bar-plus-Kitty geometry and lifecycle evidence remain the promotion gate.

<!-- END IMPORTED BODY -->
