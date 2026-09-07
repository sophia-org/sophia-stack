---
id: legacy-active-0424
date: 2026-08-14
recorded_date: 2026-08-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "security"]
---
# 2026-08-14: ordinary terminal output requires lossless authority backpressure

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12853–12872. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Diagnostic mirror attempt `0004` on signed source `d8b5e861` proved the
  PolyRectangle and fatal-cleanup changes: both heads bootstrapped, joined and
  retired repeated generations, and native suspend drained with zero abandoned
  scanouts. The remaining failure was xterm status 84, reported as a fatal X
  connection I/O error after `ll` produced a burst of terminal drawing updates.
- The production frontend still used the fail-fast observation emitter. Filling
  its 256-batch queue converted a temporary Engine scheduling delay into a
  client-local failure and closed the X connection. This is the same bounded
  transport overload previously exposed by the terminal benchmark, but normal
  interactive shell output cannot be made safe by pacing a proof workload or
  enlarging an eventually finite queue.
- Production now preserves bounded memory and each client's ordered visual facts
  by retaining one current observation per blocked worker and pausing that
  connection's X11 request dispatch until Engine drains capacity. Concurrent
  clients may interleave, as they could before this change. Shutdown explicitly
  cancels that wait. The nonblocking emitter remains available for probes that
  intentionally require a fail-fast `Backpressure` result.

<!-- END IMPORTED BODY -->
