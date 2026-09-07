---
id: legacy-active-0373
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "policy"]
---
# 2026-08-02: fresh-WM recovery must restore committed state before admission replay

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11401–11433. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next physical run loaded the focus synchronization change and still
  placed Firefox in the lower-right pane. Transaction 5 timed out and restarted
  the complete bridge/xmonad process. The fresh peer was seeded with only the
  pending Firefox `ManageSurface`; transaction 6 forced Firefox focus in
  Sophia, but transaction 7 introduced the two previously committed Kitty
  nodes and returned Kitty as xmonad's actual master/focus.
- That replay also projected the Engine's temporary 1280-by-1040 recovery
  extent as fixed `WM_NORMAL_HINTS`. Xmonad's standard `manage` path classifies
  a newly seen fixed-size window as floating. Focus synchronization cannot
  restore tiling membership or admission order after those facts have already
  been lost. The prior real-xmonad smoke reused one process where Firefox was
  already tiled, while its fixture reduced a click to `SetInputFocus`; neither
  represented a fresh StackSet.
- Restart reseed is now ordered: committed policy-managed surfaces are queued
  as a relayout first, and the unresolved admission is queued behind it. The
  relayout derives membership from committed `WmWorkspaceState`, so the pending
  surface cannot leak into the seed. Stable opaque-surface order reconstructs
  the committed xmonad admission sequence before Firefox is managed.
- Declared client constraints and Engine recovery constraints now have distinct
  projections. Blind-WM nodes carry only declared constraints and declared
  resizability; Engine transaction reconciliation continues to apply the
  effective recovery extent to geometry and configure sizes. Intrinsically
  fixed clients still cross the WM boundary unchanged, while a temporary
  recovery fence can no longer change xmonad policy identity.
- The real-xmonad smoke now destroys its first runtime, starts a genuinely
  fresh second xmonad, seeds two committed opaque nodes, replays the third
  admission, and requires that third node to remain tiled, focused, and master
  on the following relayout. Engine, session, and planner regressions lock the
  declared/effective split and the two-phase reseed order without adding an X11
  or WM wire extension.

<!-- END IMPORTED BODY -->
