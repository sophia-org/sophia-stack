---
id: legacy-active-0553
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-28: the latency row closes

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17144–17172. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Physical run `20260828T231430Z` on source `96b00d0d`: thirty-five sessions,
two hundred forty-five independent presses, zero page-flip stalls,
`status=passed failed_gates=none`. Full chain p99 24 ms against the 34 ms
two-refresh budget; queue dwell 1 ms; dwell-to-submit p99 7 ms against one
refresh; submit-to-page-flip p99 18 ms against one refresh plus the named
jitter millisecond. The M14 one-in-flight and refresh-relative latency row
is complete.

The day's arc is worth one paragraph. The row opened blocked on a readiness
predicate that could never fire, and closed with a measurement that means
what it says. In between: a readiness deadlock unsatisfiable by
construction, a completion invariant that refused a blank second monitor, a
verifier fifteen days behind its own session line, two verifier clauses
describing a retired renderer, a text proof that refused the user's own
NumLock, a harness that hung and then a harness that could not be escaped,
nine kernel-withheld vblanks attributed to amdgpu DCN3.2 from both sides of
the descriptor, a correlation measuring the wrong page flip, readiness
accepting a pre-content picture, and a reporter gating a percentile
contract on one press's vsync phase. Every fix carries a regression that
reproduces the recorded defect shape, and the QEMU session gate -- dark
since July -- now guards the whole path headlessly.

Dwell-to-submit at 7 ms p99 also answers the parked question: the
owner-thread evidence composite does not need to leave the native critical
path for this row. That optimization stays unqueued until a measurement
asks for it. The shared renderer worker is the next row.

<!-- END IMPORTED BODY -->
