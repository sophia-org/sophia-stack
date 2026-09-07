---
id: legacy-active-0277
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "tooling"]
---
# 2026-07-18: Backend Owns Present Resource Admission

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8834–8842. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`LiveProductionPresentFeedbackCoordinator` now consumes the backend-owned production batch
directly to register DMA-BUF sources and fences and to process source/fence releases. The CLI
visual wrapper no longer clones file descriptors or sequences presentation-resource lifetime
admission. The full offline all-feature suite passes; behavior is covered by the immediately
preceding guarded native mixed and strict QEMU evidence.


<!-- END IMPORTED BODY -->
