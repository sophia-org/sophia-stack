---
id: legacy-active-0169
date: 2026-07-28
recorded_date: 2026-07-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-28: Visible Vulkan Diagnosis Starts With One Natural-Size Client

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5820–5844. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical xmonad run after evidence-ranked admission produced no
visible change: default vkcube still opened a blank bordered surface. Further
changes made only inside the combined Kitty/xmonad path would not distinguish
the X11 Present/rendering fault from compatibility-bridge layout behavior.

Sophia now has a dedicated single-client production profile. It launches
`vkcube --wsi xcb` directly, omits Kitty, xmonad, xmobar, and the X11 WM
compatibility bridge, and uses the external reference WM's new generic
`natural` layout policy. That policy sees only an opaque layout node, preserves
its natural allocation, centers it within output bounds, and emits no policy
resize. It is usable for any single-purpose session; neither its reducer nor
Engine branches on vkcube identity.

The profile deliberately retains policy-managed deferred mapping, the X
authority's DRI3/Present stream, exact visual-candidate admission, renderer
composition, and KMS page-flip retirement. It therefore draws a useful fault
boundary: a blank standalone window localizes the defect below the xmonad
bridge, while a visible cube localizes the remaining defect to full-desktop
policy/configure integration. The strict verifier requires one
PresentedBuffer candidate, exact armed/presented/retired identity, nonzero
scanout pixels, normal logout, and zero live presentation resources. Physical
evidence is pending.

<!-- END IMPORTED BODY -->
