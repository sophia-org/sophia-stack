---
id: legacy-active-0322
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-07-25: Startup Evidence And Input Waits Follow Their Owners

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10066–10087. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first clean run after restoring fail-safe composition lifetime survived
201 mixed frames and completed with balanced callbacks and cleanup. Its strict
verifier still found two evidence and latency defects.

Per-output synchronous-modeset records were printed before
`LiveProductionVisualRuntime` initialized native scanout, while every head
still carried an empty initial-modeset state. The later aggregate
`output_baseline_ready` record observed the correct state, but the detailed
records had already been skipped. Native heads now retain the exact initial
submission identity, and the CLI emits each detailed record at the same
readiness transition as the aggregate record.

The physical input worker dispatch gap remained one millisecond, but an event
could enter its queue immediately after the owner drained input. The owner then
waited as long as 25 ms for X authority work before a composition taking as
long as 75 ms, producing 120 ms of measured queue dwell. Physical-input
presence now selects the existing one-millisecond owner wait budget. Cursor
and control work retain that same budget, while sessions without those
latency-sensitive sources keep the 25 ms idle wait.

<!-- END IMPORTED BODY -->
