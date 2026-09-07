---
id: legacy-active-0106
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-07: Auxiliary pixmap depths are separate from TrueColor visuals

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3436–3459. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first `199fa11d` installed-session start reached both outputs, xmonad, and
xmobar, then xterm exited on `CreatePixmap(depth=1)` with `BadValue`. The
TrueColor closure had incorrectly reduced all core pixmaps to the two color
visual depths. This is a setup/authority defect, not an Engine rendering or WM
policy failure.

XLibre's `ProcCreatePixmap` admits depth 1 as the core bitmap case independently
of the screen's ordinary visual depths and derives other nonvisual depths from
its pixmap-format table. Yserver advertises the conventional
1/4/8/15/16/24/32 storage formats, retains pixmap depth, and exercises depth-1
masks and stipples. A real xterm smoke confirmed that depths 4 and 8 are probed
before its depth-1 mask. Sophia now uses that shared bounded format family for
setup and request validation, while only depths 24 and 32 have TrueColor
visuals. Pixmap records retain depth and return it through `GetGeometry`;
auxiliary pixmaps never become XRGB Engine surfaces. Both byte orders prove the
exact setup catalog, creation and geometry for every retained depth, and
rejection of depths outside the catalog.

The failed installed release remains useful evidence but is not a promotion
candidate. The corrected successor above supersedes it for all remaining live
gates.

<!-- END IMPORTED BODY -->
