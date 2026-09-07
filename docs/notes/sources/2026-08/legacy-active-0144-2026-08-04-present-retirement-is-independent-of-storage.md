---
id: legacy-active-0144
date: 2026-08-04
recorded_date: 2026-08-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-04: Present retirement is independent of storage

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4618–4649. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The fresh installed xmonad/vkcube run disproved the remaining DMA-only
assumption. Engine selected exact transaction 626, surface 6291456, CPU buffer
28 as `PresentedBuffer`, but admission committed it as `cpu_snapshot` without
native retirement. The vkcube main and WSI queue threads then remained blocked
on their FIFO Present wait. In the owner batch that released transaction 626,
an unrelated Kitty DMA-BUF Present selected the GPU production path. That path
scheduled the DMA-BUF group but did not register the separate software-Present
group, so no Complete or Idle event could unblock vkcube.

XLibre's copy path copies the pixmap and sends Idle plus clocked Complete after
the target MSC. Yserver independently preserves the same Idle/Complete
lifecycle when its scheduler chooses copy instead of flip. Storage selects the
composition mechanism; it does not erase Present identity or retirement.

Sophia now carries the exact transaction/surface/target-buffer key and source
extent on software Present submissions. Any Engine-selected `PresentedBuffer`
enters `AwaitingRetirement`, including a CPU materialization. A mixed owner
batch registers its separate software groups before the DMA-BUF group drives
the shared native frame; submission and page-flip settlement then route
clocked Copy/Idle feedback and expose the exact software retirement to layout
and admission. Only a non-Present `BackingSnapshot` may use immediate CPU
admission. Same-group storage ambiguity fails closed.

The Rust regressions cover exact CPU admission fencing, software feedback
retirement, source extent, and intake cardinality. `AdmissionRecovery.tla`
explores both DMA and CPU storage for the same Present lifecycle, and the
physical verifier accepts either exact storage identity while rejecting any
retirement bypass. Offline gates pass; a fresh installed physical run remains
the milestone gate.

<!-- END IMPORTED BODY -->
