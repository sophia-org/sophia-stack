---
id: legacy-active-0257
date: 2026-07-17
recorded_date: 2026-07-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-07-17: Input Delivery Settlement Restores Bounded Sessions

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8432–8459. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The default xterm QEMU gate exposed a second post-input stall after keyboard and
pointer evidence had already succeeded. Phase tracing proved cursor composition,
KMS submission, and page-flip retirement all returned. The loop instead kept
`input_delivery_wait_started_at` populated after the exact key deliveries
settled. Because ordinary proof sessions advance `--max-ticks` only outside an
active delivery wait, a successful input proof made the session immortal; GTK
was unaffected only because its application-specific proof exits immediately.

Delivery settlement now consumes the wait timestamp exactly once. Later pointer
or emergency batches start their own bounded delivery wait and clear it after
settlement, while the initial key-flush record remains tied to the complete
14-event sequence. A regression covers the consume-once transition. The QEMU
verifier now recognizes current schema 14 and validates either native CPU export
mode: zero GL resources for the preferred direct linear GBM write, or exactly
one reusable GL target/pipeline per output for the fallback. Mixed counters,
recreation, missing uploads, latency violations, and cleanup debt still fail.

The rebuilt X13-hosted image passed every unattended profile. The strict
two-xterm run completed 300 ticks in 6,971 ms with two CPU layers, 8 ms input
presentation, 11 ms maximum composition, 40 submissions, 38 retirements, and
zero cleanup debt. Classic and confined GTK completed normally with exact
stdout, `first_error=none`, pointer selection, and clean two-output retirement.
The emergency profile armed and triggered Ctrl-Alt-Backspace, flushed all five
routed deliveries, and shut down cleanly in 187 ms.


<!-- END IMPORTED BODY -->
