---
id: legacy-active-0561
date: 2026-08-29
recorded_date: 2026-08-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-29: sharing a renderer thread costs nothing measurable

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17437–17459. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Thirty-five sessions with both outputs of one card on one renderer thread:
`status=passed`, 245 presses, full chain p99 23 ms against the 34 ms budget,
queue dwell 1 ms, dwell-to-submit p99 7 ms, submit-to-page-flip p99 18 ms. No
stall retries were consumed and no session stalled at all.

Against the dedicated baseline of the same evening -- p99 24 ms, dwell-to-
submit 7 ms, submit-to-page-flip 18 ms -- every figure agrees within a
millisecond, including dwell-to-submit, which is the only stage a shared
queue could plausibly have cost: it is the window in which a sibling's render
could sit ahead of this output's. Four runs at one thread now agree with each
other and with the two-thread baseline.

That is the measurement the promotion deliberately left owed. The shared
worker was promoted on archive `0003` for correctness, not for cost, and
until now nothing had measured what an ordinary session would pay for it. The
answer is nothing, and the record says so with the thread count printed
beside the latencies rather than remembered.

Whether sharing becomes the default is a product decision and remains open.
What is no longer open is whether it would cost latency to make it one.

<!-- END IMPORTED BODY -->
