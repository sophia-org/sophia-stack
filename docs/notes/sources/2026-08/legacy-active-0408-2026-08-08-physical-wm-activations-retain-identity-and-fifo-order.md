---
id: legacy-active-0408
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-08: physical WM activations retain identity and FIFO order

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12452–12475. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The production live-WM owner queue had contradicted `PolicyLifecycle.tla` by
  treating an opaque action token as an idempotency key. A second physical
  activation of the same registered action was discarded whenever an equal
  request was pending or in flight. Tokens identify policy operations; they do
  not identify physical activation instances.
- The existing sixteen-entry owner bound now retains every admitted action in
  FIFO order. An in-flight request counts against the same bound. Saturation
  remains fail-closed because the Engine shortcut reducer has already consumed
  the chord; the session reports a saturating rejection count and limits
  per-event rejection diagnostics to sixteen records.
- Retaining multiple actions required more than deleting the duplicate check.
  Each action is rebuilt at the transport head against the latest committed
  workspace and layout snapshot while preserving its minted transaction and
  queue position. Thus the second activation observes the state committed by
  the first instead of carrying the stale snapshot captured at ingress.
- Scene refresh and completed pointer-gesture requests retain their existing
  selective duplicate reduction. The completion ledger now reports
  `action_ordered` and keeps `action_coalesced=0` as an explicit compatibility
  assertion. Focused regressions cover equal FIFO values, capacity including
  in-flight work, dequeue-time state rebasing, and verifier rejection of any
  nonzero action-coalescing count.

<!-- END IMPORTED BODY -->
