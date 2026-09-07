---
id: legacy-active-0341
date: 2026-07-28
recorded_date: 2026-07-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-28: standalone Vulkan isolated unresolved software Present

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10694–10712. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first standalone `vkcube --wsi xcb` run passed natural-layout admission,
  configure, and focus, then timed out at `no_visual_detail`. Vulkan selected
  llvmpipe. Sophia selected a 500-by-500 `XPixmap` candidate while recording
  zero DMA-BUF registrations and zero Present submissions, so no renderable
  storage could reach composition or KMS.
- A raw X pixmap is not a renderer buffer. The X authority now materializes
  regular software pixmaps into immutable CPU snapshots at Present time. It
  also retains MIT-SHM pixmap bindings and snapshots client-owned shared pixels
  at the same transactional boundary; DRI3 remains the zero-copy path.
- Unresolved pixmaps now fail closed instead of becoming false
  `PresentedBuffer` evidence. Complete-presentation semantics travel as a
  passive, protocol-neutral surface observation independent of CPU or DMA-BUF
  storage, preserving the admission reducer's distinction between a submitted
  frame and an accumulated backing image.
- The whole-pixmap SHM copy is the bounded correctness fallback. Damage-scoped
  persistent mappings remain an explicit post-proof optimization.

<!-- END IMPORTED BODY -->
