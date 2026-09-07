---
id: legacy-active-0342
date: 2026-07-28
recorded_date: 2026-07-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-28: software Present feedback must follow composed KMS retirement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10713–10733. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first materialized software-Present run made the llvmpipe cube visible,
  but it remained on its first frame. The log contained one authority visual
  transaction and a retired nonzero CPU scanout, while Present Complete, Idle,
  and idle-fence-trigger counts all remained zero. The client was correctly
  waiting for permission to reuse its software pixmap.
- Presentation lifetime is now independent of storage kind. A software
  Present carries only its transaction, surface, and optional acquire/idle
  fence handles across the passive authority and production records. It owns
  no fabricated DMA-BUF handle.
- The production runtime registers that lifetime beside the CPU transaction,
  marks it submitted only when the composed primary frame reaches native KMS,
  and emits Complete followed by Idle after the matching page-flip retirement.
  Headless composition settles at its deterministic submission boundary.
- Focused regressions require source-free presentation retirement, actual idle
  fence signaling, authority-to-production observation preservation, and
  Complete-before-Idle routing. The physical verifier now rejects a visible
  but static software frame by requiring at least three authority frames and
  positive Complete, Idle, and idle-fence evidence.

<!-- END IMPORTED BODY -->
