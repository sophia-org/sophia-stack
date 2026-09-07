---
id: legacy-active-0558
date: 2026-08-29
recorded_date: 2026-08-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "validation"]
---
# 2026-08-29: latency after the renderer restructure, and what it did not measure

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17353–17377. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Thirty-five sessions on `02d11e0f`: full chain p99 24 ms against the 34 ms
budget, queue dwell 1 ms, dwell-to-submit p99 7 ms, submit-to-page-flip p99
18 ms inside one refresh plus the named jitter millisecond, and no page-flip
stalls at all -- against nine the previous evening. The renderer restructure
cost nothing measurable.

What the run did not measure is worth as much as what it did. The harness does
not enable shared renderer workers, so those sessions ran a thread per head:
the numbers confirm the restructure did not regress the dedicated path, which
is what an ordinary session still uses while sharing stays opt-in, and they
say nothing about latency when two outputs share a queue. A second head's
render delaying this one is the single cost the shared worker could impose,
and nothing has measured it.

Two reports with the same latencies and different thread counts are different
measurements, and until now nothing in the record distinguished them. The
report is schema 5 and names the renderer-thread count it observed, reading it
from the sessions themselves rather than from what the harness was told to do;
mixed counts across a run report `mixed` rather than a number that would be
true of only some sessions. The harness takes `--shared` to measure the other
configuration, and records the choice in `source.env`. That pairing is the
prerequisite for deciding whether sharing becomes the default.

<!-- END IMPORTED BODY -->
