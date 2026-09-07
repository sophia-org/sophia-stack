---
id: legacy-archive-0019
date: 2026-07-07
recorded_date: 2026-07-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-07: Current Local XLibre Facts

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 341–356. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Local source tree: `/home/niltempus/src/xserver`.

Observed facts:

- Xnamespace exists and is enabled by the `namespace` build option.
- Selection names are rewritten per namespace.
- A draft/runtime `X-NAMESPACE` management protocol skeleton exists.
- XComposite and Damage exist as likely render handoff points.
- Input delivery still flows through legacy `XYToWindow`, sprite trace, grabs,
  focus, and DIX delivery.

Implication: rendering has a plausible existing seam. Compositor-routed input
needs explicit XLibre protocol/server work.

<!-- END IMPORTED BODY -->
