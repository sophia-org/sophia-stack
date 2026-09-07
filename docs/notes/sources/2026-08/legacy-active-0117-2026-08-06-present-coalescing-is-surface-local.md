---
id: legacy-active-0117
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-06: Present coalescing is surface-local

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3871–3901. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Native-chrome attempt `0003` applied and rendered the combined policy, but its
sequence driver correctly rejected the resize boundary. The preceding
two-surface epochs armed exact Present candidates for both Kitty surfaces while
only one candidate per epoch reached native retirement. The other surface kept
an older Engine committed extent. When combined chrome restored that extent,
the layout coordinator suppressed its Configure as already committed and
produced a one-surface epoch with clipped pixels.

The production Present scheduler was coalescing all runnable queued work into
one newest transaction. This crossed surface identity: releasing two staged
Presents, or receiving the second surface after the epoch had committed, could
reject the first surface even though both carried independent visual debt. It
also contradicted the architecture rule that unrelated surfaces remain
independently runnable.

Runnable coalescing now keeps the newest transaction per surface. Same-surface
overload remains bounded, while distinct surfaces retain FIFO order and exact
retirement ownership. Regressions cover both two-surface release in one epoch
and the observed ordering where one staged Present becomes runnable before the
second surface arrives. The native-chrome verifier now requires both distinct
armed candidates to retire at their exact extents before the next policy
generation may advance.

Installed commit `6a5bc833` passed native-chrome archive `0004`. Ring-wide,
frame-only, and combined policy each delivered two Configures, armed two exact
surface candidates, and retired both before advancing. The archive records 14
routed physical keys, two connected outputs, normal logout, clean native drain,
and no protocol, submission, retirement, or cleanup debt.

<!-- END IMPORTED BODY -->
