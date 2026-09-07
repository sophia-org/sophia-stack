---
id: legacy-active-0176
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-26: Managed X11 Mapping Requires Pre-Pixel Admission

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6042–6074. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The repeated blank `vkcube` frame was an ordering defect, not evidence that the
application required a hard-coded floating or fixed-size rule. The live X
frontend fulfilled `MapWindow` immediately and emitted `MapNotify`/`Expose`
before the blind WM had an opaque node to manage. Sophia could create that node
only after observing a pixel-backed transaction, while the application was
already reconfiguring its swapchain from the initial 500-by-500 window to the
tiled allocation. The latest trace registered buffers and fences but retired no
cube Present at the accepted layout.

X server map-redirection semantics supplied the useful reference model: keep a
policy-managed window unmapped while policy decides its geometry, then configure
and map it as one admitted transition. Sophia implements that invariant through
its own protocol-neutral boundaries. The frontend emits
`SurfacePresentationIntent`; Engine retains passive admission facts; the WM
plans an opaque bufferless node; and `AdmitSurface` configures and maps only
after proposal validation. Matching authority pixels, not the control
acknowledgement, establish committed visual truth. No X server code or
application-specific fact enters Sophia.

The same audit found an overly broad Present barrier. Any pending layout had
blocked every queued Present, including unrelated stable surfaces. Each
submission now carries an immutable disposition: immediate, staged for one
layout epoch, or rejected for a known size mismatch. The bounded scheduler can
continue unrelated work, shares one immutable CPU-layer batch, and releases
staged submissions when the epoch resolves. Wrong-size pending pixels update
only the safe-observation record and cannot leak into the visible layer table.

Reducer, wire, admission, WM pre-pixel planning, and mixed Present scheduling
tests cover the new ordering. Default `vkcube --wsi xcb` remains the physical
AMDGPU proof; until that retained run succeeds, the roadmap item stays open.

<!-- END IMPORTED BODY -->
