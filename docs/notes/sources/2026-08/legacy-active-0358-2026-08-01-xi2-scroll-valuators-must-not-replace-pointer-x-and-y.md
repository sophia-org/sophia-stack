---
id: legacy-active-0358
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-08-01: XI2 scroll valuators must not replace pointer X and Y

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11047–11068. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The synchronized physical Firefox run reached the deterministic loaded,
  keyboard, clipboard, pointer, and PRIMARY stages. A real wheel event then
  crossed libinput, was observed and routed by Engine, and received an X input
  delivery acknowledgement, while the page never completed its DOM `wheel`
  stage. Firefox presentation continued retiring at the correct toplevel, so
  this isolated the failure after compositor hit testing and before browser
  event handling.
- Sophia described horizontal and vertical scrolling as XI2 valuators 0 and 1
  and set those same bits in XI2 motion events. Xorg reserves valuators 0 and 1
  for relative pointer X and Y; its input-test device places relative
  horizontal and vertical scrolling on valuators 2 and 3. A scroll class names
  an existing valuator rather than replacing the pointer coordinate axes.
- The X frontend now reports four relative pointer valuators, associates the
  preferred horizontal and vertical scroll classes with axes 2 and 3, and sets
  those same valuator-mask bits in XI2 motion events. Cumulative v120 positions
  and legacy Button4-Button7 emulation remain X-frontend state; Engine input
  packets remain protocol-neutral. Wire regressions parse the complete pointer
  class topology and cover simultaneous two-axis value ordering. The physical
  Firefox DOM stage remains the acceptance proof.

<!-- END IMPORTED BODY -->
