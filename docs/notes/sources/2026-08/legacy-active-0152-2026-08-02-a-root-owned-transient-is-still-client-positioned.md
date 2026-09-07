---
id: legacy-active-0152
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-02: A root-owned transient is still client-positioned

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4880–4901. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical Firefox run proved control-time refocus and reached the real
popup. Clicking `Open proof dialog` then blanked the owner. The trace showed
the popup entering ordinary WM admission: xmonad tiled it with the three
existing application surfaces, Firefox did not produce any of the four exact
requested extents, and each preserved-layout timeout restarted and reseeded
the bridge. The restart fix prevented the earlier `UnknownSurface` exit, but
could not correct the popup's wrong presentation role.

`WM_TRANSIENT_FOR` has two independent facts: the property marks a transient,
and its window value may resolve to an Engine surface owner. Sophia had
conflated them by classifying a window as client-positioned only when owner
resolution succeeded. ICCCM group transients legitimately point at the root,
which has no application surface, so the hint was reduced to no owner and the
popup was incorrectly promoted into blind WM admission. Window state now
retains transient-hint presence separately from the optional reduced owner.
Root-owned and otherwise unresolved transients remain client-positioned while
publishing no false owner edge; deleting the property restores normal policy
management. A mapped root-transient wire regression covers both transitions.
A fresh physical popup run remains required.

<!-- END IMPORTED BODY -->
