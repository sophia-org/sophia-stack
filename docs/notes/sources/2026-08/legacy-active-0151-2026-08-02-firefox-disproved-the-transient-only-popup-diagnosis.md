---
id: legacy-active-0151
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-02: Firefox disproved the transient-only popup diagnosis

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4849–4879. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The physical run from `d38a217c` used a freshly built binary and still admitted
Firefox surface `8388650` as a fourth policy-managed tile. That is decisive:
the live popup did not produce a valid `WM_TRANSIENT_FOR` reduction, so the
root-transient fix was correct protocol handling but was not the observed
Firefox fix. The ensuing four-surface resize epoch timed out with the popup at
zero committed size, restarted xmonad, retried, and then entered a repeating
single-Firefox resize loop. The GDK thaw warning occurred immediately after
the admission control, consistent with the blanked browser frame.

The missing authority input is EWMH `_NET_WM_WINDOW_TYPE`. EWMH requires this
pre-map functional hint to influence WM behavior, but Sophia stored the
property without reducing it and the blind WM cannot inspect application X11
properties. X Authority now decodes the ordered ATOM list, skips unknown
extension types, keeps `NORMAL` policy-managed, and reduces dialog/menu/
utility/splash/popup-like types to `ClientPositioned`. Replacement and deletion
publish role snapshots, and a redacted trace records the live reduction. A
wire regression sets an unknown preferred type followed by `DIALOG` before
map, proves immediate client-positioned mapping, and proves deletion restores
normal policy.

There is also a recovery-loop bug independent of the hint. A timed-out
admission can safely publish retained pixels at its initial extent, but the
retirement path immediately cleared that recovery extent while its original
WM target was still outstanding. The automatic relayout therefore drove the
same failed resize again. Recovery extents now remain pinned until the standing
target actually commits; the retained surface can stay visible and a future
explicit optimization may retry convergence without blanking the committed
layout. A fresh physical run remains the acceptance boundary.

<!-- END IMPORTED BODY -->
