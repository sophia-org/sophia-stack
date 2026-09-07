---
id: legacy-active-0142
date: 2026-08-04
recorded_date: 2026-08-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-08-04: delayed Presents reconcile resolved layout epochs

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4529–4564. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first installed run after grouped CPU content selected vkcube's 500-by-500
software Present, but no vkcube window became visible. The live trace showed
that application creation and pixel capture had succeeded. Its admission epoch
timed out while an existing Kitty DMA-BUF Present was staged; rollback rejected
the scheduler-owned Kitty Present, but later Kitty groups remained behind that
Present in `SurfaceContentStream`.

Those later groups retained `StageLayout { epoch: 2 }`. They reached the
Present scheduler only after epoch 2 had aborted, so the one-shot abort could
not see them. The scheduler treated the dead epoch as pending again. No future
commit or abort could release it, Kitty stopped supplying resize evidence, and
vkcube's valid admission transaction remained outside the visible workspace.
The new content stream exposed this latent time-of-classification error by
making the deferred ownership exact.

The Present scheduler now retains bounded outcomes for resolved layout epochs
and reconciles a delayed submission when it actually enters the queue. Work
from an aborted epoch receives ordinary controlled Skip/Idle settlement; work
from a committed epoch runs when its surface is already visible or waits only
for visibility. An outcome older than the bounded exact history fails closed
instead of recreating an epoch that cannot progress. Crate-boundary regressions
cover both delayed abort and delayed commit. A fresh installed vkcube run
remained the physical acceptance boundary.

Installed commit `663934ca` passed that boundary. The run reproduced the
important recovery shape: vkcube's first 500-by-500 software Present arrived
during epoch 2, the blind-WM resize timed out, and one staged Kitty Present was
aborted. Kitty then resumed native retirement, epoch 4 committed both visible
surfaces, and vkcube's exact transaction 574 retired on native frame 20. The
cube continued for 665 clocked software retirements with increasing kernel
MSC values. Normal logout reported 691 Complete events and 691 Idle/fence
signals, 132 native retirements, no protocol or native failure, no live
presentation resource, clean layout health, and clean frontend teardown.

<!-- END IMPORTED BODY -->
