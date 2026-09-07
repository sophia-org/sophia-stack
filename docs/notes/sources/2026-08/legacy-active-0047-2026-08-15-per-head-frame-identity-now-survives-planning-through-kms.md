---
id: legacy-active-0047
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: per-head frame identity now survives planning through KMS

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1492–1512. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The per-head lowerer produced the right native geometry, but its passive backend
envelope retained only the opaque head and logical checksum. A queued worker
result could therefore no longer prove which Engine scene generation, committed
target generation, or `Fit`/`Cover`/`Exact` mapping had produced its pixels.
That omission weakened stale-plan rejection precisely where an IPC topology
change may replace a target while older renderer work is still completing.

`LiveProductionHeadCompositionFrame` now carries the scene generation, target
generation, and protocol-neutral mapping in addition to head and logical
checksum. Admission validates the complete unique head set against the current
`HeadRenderTarget` set before mutating any exporter, and topology candidate and
rollback pools validate the same identity against their respective plan side.
Stable plan and queue evidence records preserve the chain without exposing DRM
identity. The mirror physical verifier accepts a generation only when each head
has one native-size plan, one matching queued frame, the expected sampling
density, and strict plan-before-queue-before-KMS ordering. Deterministic negative
fixtures cover missing heads, stale target generations, mapping/extent mismatch,
and late queue evidence.

<!-- END IMPORTED BODY -->
