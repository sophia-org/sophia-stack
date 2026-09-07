---
id: legacy-active-0159
date: 2026-07-31
recorded_date: 2026-07-31
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation", "architecture"]
---
# 2026-07-31: Synchronized input latency uses an end-to-end and stage contract

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5167–5189. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The former physical gate required full-chain p95 below one 17 ms refresh. A
randomly phased input event can spend nearly that entire interval waiting for
the next synchronized page flip after useful work has completed, leaving an
unrealistic sub-millisecond p95 allowance for input delivery, client response,
composition, and submission. The aggregate bound therefore encouraged tearing,
VRR, or workload-specific bypasses before the normal synchronized path was
otherwise ready.

The physical contract now requires full-chain p95 below two configured refresh
periods and independently fails when maximum queue dwell exceeds 1 ms,
dwell-to-submit exceeds one refresh, or submit-to-page-flip exceeds one
refresh. The first draft used half a refresh for dwell-to-submit, but physical
evidence showed that this interval also includes the external client's response
rather than only Sophia-owned work. One refresh is the meaningful correctness
boundary: it rejects an additional processing frame, while the aggregate bound
still rejects two complete stages at their limits. The two-refresh bound is
exclusive; stage bounds are inclusive. The reporter emits the refresh, derived
aggregate budget, every stage budget, and named failed gates under schema 2.
Existing immutable schema-1 evidence remains valid historical evidence and is
not rewritten.

<!-- END IMPORTED BODY -->
