---
id: legacy-active-0389
date: 2026-08-03
recorded_date: 2026-08-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering"]
---
# 2026-08-03: retained recovery pixels must remain live

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11805–11846. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first normal login through installed release `0.1.0-09113a7da149`
  completed all eight lifecycle records, returned through the display-manager
  handoff with exit status zero, restored the original KD mode and termios,
  left no graphical process, and emitted no failure diagnostic. The session
  reached clean protocol, layout, presentation-resource, and frontend cleanup
  summaries.
- Ordinary use nevertheless exposed a visual liveness failure. A 300-by-300
  GLX surface launched from Kitty displayed one `glxgears` frame and then
  remained static. The surface retired exactly one Present while Firefox
  retired 1,400; the completion summary recorded 188,148 controlled Present
  rejections and two recovered WM layout timeouts.
- The GLX surface had been admitted at its coherent 300-by-300 recovery extent
  while retaining a 1276-by-709 blind-WM target. After the first admission
  frame retired, `present_layout_disposition` compared every newer
  300-by-300 buffer only with the standing target and returned
  `RejectLayoutMismatch`. Immediate skipped feedback let the client run
  unpaced, but no newer GLX buffer could reach Engine preparation or native
  retirement.
- XLibre/Xorg does not discard `PresentPixmap` solely because its pixmap and
  current window extents differ: the non-flip path copies the applicable
  region and settles the request. Yserver independently selects its Copy path
  when a flip is unavailable and keeps Present scheduling separate from its
  ordered Present/core `ConfigureNotify` delivery. Sophia retains its stronger
  atomic geometry rule by displaying these updates only at the already
  coherent recovery geometry; neither reference requires publishing the
  outstanding target before matching pixels exist.
- Engine now owns a protocol-neutral extent classifier with four results:
  unconstrained, exact expected target, explicitly retained recovery extent,
  and mismatch. The live presentation gate schedules newer buffers in the
  retained-recovery class immediately while leaving the standing target,
  committed geometry, and exact-target retirement rules unchanged. Unrelated
  extents still fail closed, and X authority continues to own X11 Present and
  ConfigureNotify semantics without gaining layout policy.
- The crate-boundary regression repeats retained recovery classification,
  proves that it does not discharge either the extent or target, accepts the
  exact target separately, rejects an unrelated extent, and rejects the old
  extent again once recovery is cleared. All Engine layout-epoch tests and the
  all-feature workspace suite pass. A packaged physical GLX rerun remains the
  acceptance boundary.

<!-- END IMPORTED BODY -->
