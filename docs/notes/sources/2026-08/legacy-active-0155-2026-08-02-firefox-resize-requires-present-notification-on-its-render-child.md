---
id: legacy-active-0155
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-02: Firefox resize requires Present notification on its render child

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4957–4986. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The physical three-window trace separated the remaining short black Firefox
frame from Engine resize admission. Xmonad repeatedly requested a
1276-by-1422 left pane, but Firefox submitted only 1280-by-1040 PresentedBuffer
candidates. Engine correctly preserved the retired 1280-by-1040 recovery frame
and kept the standing resize target pending; the committed launch frame carried
no visible browser pixels, so the safe result was a short black pane rather
than a falsely stretched or partially committed browser.

The missing transition was in the X frontend. Firefox presents from a
descendant render window. Sophia accepted the descendant's client-controlled
`ConfigureWindow`, updated its geometry, and sent core `ConfigureNotify`, but
did not send the Present `ConfigureNotify` selected on that exact child.
XLibre confirms that Present wraps the screen `ConfigNotify` hook and therefore
notifies every matching subscriber before core event delivery. Yserver provides
an independent native-Rust confirmation: its Firefox investigation found Mesa
retaining the old swap-buffer size and rendering blank until the same Present
notification was emitted on real window reconfiguration.

Present configure delivery therefore remains X-authority policy. The frontend
now notifies every Configure-mask subscriber for the exact reconfigured window,
uses each receiving client's sequence, preserves Present-before-core ordering,
and suppresses the Present event for failed or no-op geometry requests. The
Engine's protocol-neutral standing-target, visual-evidence, recovery, and
retained-frame rules remain unchanged. Focused socket regressions cover a
Firefox-shaped child resize, cross-client subscription routing, no-op and mask
filtering, and Engine-originated configure ordering. A new physical run remains
the acceptance boundary.

<!-- END IMPORTED BODY -->
