---
id: legacy-active-0506
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-22: mirror coalescing cannot erase a Present owner

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15499–15528. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Signed source `b21b7692df82096e24ccd652d293c5f0527517d3` produced and
independently re-verified mirror promotion archive `0003`. The following mixed
run proved the chrome correction: the first surface presented at 2556 by 1436
inside its 2560-by-1440 outer allocation, admission recovery cleared, and the
public policy committed the two-output topology.

The run then ended at a strict ownership check. Present transaction 574 owned
mirror frame 77, but neither head had submitted it when the committed topology
queued ordinary frames 78 and 80. Per-head pacing correctly coalesced to the
newest scene and released frame 77. The Present scheduler, however, still held
transaction 574 in its rendering state. When retained frame 80 reached KMS,
the runtime refused to attribute that unrelated frame to transaction 574 and
ended the session with `native output submission does not match its Present
ownership`.

Weakening that check would give protocol feedback to pixels that did not earn
it. Native scheduling instead defers an ordinary successor while the active
generation is an unsubmitted Present. The deferred slot is latest-wins, so
ordinary scene churn remains bounded. Once the primary head submits the Present,
its page flip owns logical completion; the deferred successor may then become
active, and a slower secondary may skip directly to it under the existing
last-head lifetime rules. A passive regression retains frame 77 and transaction
574 and requires deferral before primary ownership and admission afterward.

This changes the executable and remains local evidence. Mirror archive `0003`
does not promote the successor, and the mixed run did not reach its archive or
the Hagia and broker gate.

<!-- END IMPORTED BODY -->
