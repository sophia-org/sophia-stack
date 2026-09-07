---
id: legacy-active-0371
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11"]
---
# 2026-08-02: recovery-hint release must preserve xmonad window identity

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11356–11378. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next physical run confirmed the standing-target fix: Firefox's exact
  1276-by-1422 frame retired, `reason=standing_target_presented` cleared the
  temporary extent, later frames used complete clips, pointer focus handoffs
  succeeded, and the browser proof reached 7/8. The constraint relayout then
  moved the newly focused Firefox from xmonad's full master column into the
  lower-right slave pane.
- The move was not a layout-order or pixel regression. Releasing the temporary
  fixed extent changed the bridge's synthetic `WM_NORMAL_HINTS` profile. The
  bridge represented that mutable property change as `DestroyNotify` followed
  by a new `MapRequest`; xmonad consequently discarded the surface's focus and
  stack identity before remapping it.
- XLibre's core property path delivers `PropertyNotify` under
  `PropertyChangeMask`, and yserver's independent encoder confirms the exact
  32-byte core event layout. The private xmonad bridge now updates the stored
  manage profile in place, emits `PropertyNotify` for `WM_NORMAL_HINTS`, and
  retains the synthetic window ID. A reducer regression covers fixed and
  released profiles without destroy/map events. The real-xmonad smoke changes
  an already focused recovery surface back to resizable and requires the same
  master/focus stack afterward, preserving a seam for later property-routing
  optimization.

<!-- END IMPORTED BODY -->
