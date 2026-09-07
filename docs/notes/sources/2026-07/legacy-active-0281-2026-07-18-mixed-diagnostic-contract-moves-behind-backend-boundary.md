---
id: legacy-active-0281
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-07-18: Mixed Diagnostic Contract Moves Behind Backend Boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8892–8903. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The native mixed-export completion record and its reduced evidence schema now live in
backend-live beside the native scanout diagnostic that produces them. The CLI only downcasts
the backend error, prints the reduced record, and applies command-level pass criteria; it no
longer defines a renderer\/scanout result type inside session supervision. A backend regression
freezes the exact schema. The rebuilt guarded X13 diagnostic still exported one CPU and one
DMA-BUF layer and retired all sources, fences, and transactions. This removes one CLI-specific
dependency that pinned the remaining neutral visual-control implementation to
`live_session.rs`.


<!-- END IMPORTED BODY -->
