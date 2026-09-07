---
id: legacy-active-0350
date: 2026-07-31
recorded_date: 2026-07-31
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-31: unchanged secondary outputs must not be recomposed

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10866–10886. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first zero-spacing physical archive proved zero libinput queue dwell in
  all 20 samples, but failed at 29 ms p95 and 30 ms maximum. The isolated input
  frame consistently spent 11–14 ms before KMS submission, then up to 16 ms
  waiting for the kernel page flip. Exact text, clocks, cleanup, and renderer
  health all passed; native upload remained bounded at 3 ms.
- `LiveProductionCpuScene::frames_for_outputs` rebuilt the diagnostic frame for
  every non-primary output after each primary composition. On the physical
  topology that allocated, cleared, drew, and exactly scanned an unchanged
  1920x1080 CPU frame on the owner path for every xterm update.
- The scene now retains immutable secondary frames by output index and complete
  `HeadlessOutput` descriptor. Primary recomposition clones the retained frame;
  output removal, reorder, size, scale, or identity change invalidates it.
  Regression coverage proves the same pixel allocation survives primary
  recomposition and is replaced after a descriptor change.
- The focused dual-output QEMU GBM/KMS path passed exact input and pointer
  pixels, damage-only buffer reuse, kernel page-flip correlation, and clean
  teardown with 2 ms maximum composition and 1 ms maximum upload. Physical TTY3
  p95 remains the authoritative promotion gate.

<!-- END IMPORTED BODY -->
