---
id: legacy-active-0349
date: 2026-07-31
recorded_date: 2026-07-31
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-07-31: one latency sample must produce one input frame

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10847–10865. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The retry-capable physical run completed all 20 independent samples with
  exact text, kernel page-flip timestamps, clean teardown, and no retries.
  Damage-only reuse held maximum native upload to 2 ms, but full-chain p95 was
  22 ms against the 17 ms refresh budget.
- The injector's two-millisecond delay after every key transition stretched
  `sophia\n` across roughly 28 ms. Sophia correctly rendered and submitted
  intermediate terminal states. On the two p95-tail samples, the last routed
  press then arrived while one of those earlier states already owned the next
  page flip, adding 8–9 ms between queue dwell and final-frame submission.
- The physical gate now emits the same exact press/release sequence with zero
  spacing. Events still enter through uinput, the normal threaded libinput
  worker, X delivery, xterm, CPU composition, atomic KMS submission, and kernel
  retirement. Coalescing the bounded burst into one visual transaction removes
  an unrelated earlier frame from the isolated input-to-photon measurement;
  exact text, changed pixels, event flush, and exact-frame correlation remain
  mandatory.

<!-- END IMPORTED BODY -->
