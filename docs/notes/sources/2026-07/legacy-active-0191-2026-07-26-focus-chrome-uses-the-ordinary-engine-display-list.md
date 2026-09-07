---
id: legacy-active-0191
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "shell"]
---
# 2026-07-26: Focus Chrome Uses The Ordinary Engine Display List

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6454–6490. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Sophia now has the first production compositor-owned visual: a minimal focused
surface border. Engine builds one bounded immutable display list from opaque
surface order, committed focus, and committed surface state. Four stable
inside-edge nodes follow the focused surface in that list. Node identity,
generation, geometry, and color drive deterministic old/new damage; unchanged
nodes produce no compositor damage. The external WM and X frontend receive no
chrome records, graphics parameters, renderer handles, or application facts.

The CPU reference path consumes the ordered surface and solid commands
directly, including clipping and exact XRGB byte output. The native mixed path
uses the same list to interleave CPU buffers, DMA-BUF buffers, and compositor
solids. EGL lowers each solid to a scissored opaque clear, avoiding texture
allocation in the frame hot path. A prepared GPU Present uses the candidate
surface state associated with those pixels; retained and CPU composition use
committed state, so a border cannot move ahead of matching client geometry.

The two-output QEMU xmonad gate passed with the new path. It observed four
border primitives on both focus targets. During the click-drag proof, WM focus,
Engine focus, and X frontend focus acknowledgment completed first; the retained
pointer records were then released, the new border frame was composed for that
same opaque target, and the following key reached it. The verifier rejects a
missing border, the wrong target, fewer than four primitives, or border
evidence that arrives after the following key. Physical DRM confirmation across
focus, resize, workspace, VT, and mixed CPU/DMA-BUF presentation remains open.

Border generation hashes only facts that change compositor pixels: geometry,
thickness, and color. Client buffer commits therefore do not create false
compositor damage or repeated evidence. Hiding focus clears retained
observation state so restoring the workspace proves a new border composition;
VT and native-recovery repaints explicitly re-emit the reduced border fact even
when geometry is unchanged. A dedicated physical verifier now requires those
workspace and VT sequences, two focus targets, a focused geometry-generation
change, nonzero mixed exports, and clean shutdown. Its pass and mutation
fixtures fail closed without claiming physical completion.

<!-- END IMPORTED BODY -->
