---
id: legacy-active-0344
date: 2026-07-29
recorded_date: 2026-07-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-29: retained repaints must reuse renderer image generations

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10756–10776. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The latest physical GLX run completed two mixed Presents and routed the
  predecessor Idle correctly. A focus-border repaint then recreated the current
  DMA-BUF's EGLImage and GL texture. The draw blocked for 10.4 seconds in the
  frame completion barrier before AMD reported a guilty-context hard recovery;
  Sophia aborted with status 134 and the GLX client lost its X connection.
- The static gears were therefore not a GLX bootstrap, input, KMS callback, or
  TTY-recovery failure. Only the first two frames retired before a generic
  compositor-only repaint repeated a live import.
- Mixed layers now carry opaque renderer-image generations. A bounded
  renderer-private slot table imports one EGLImage/texture per generation,
  validates the complete DMA-BUF identity on hits, and reuses the texture for
  focus, chrome, workspace, and retained-damage repaint.
- Replacement retirement evicts the predecessor while its native context is
  current before triggering the idle fence. Context recreation and normal
  shutdown clear residency before presentation leases are released.
- Mixed and CPU presentation now reports X Present `Copy`; `Flip` is reserved
  for future direct scanout. Reduced metrics and the GLX reporter require cache
  hits, reject mismatch/capacity debt, and preserve post-KMS cadence evidence.

<!-- END IMPORTED BODY -->
