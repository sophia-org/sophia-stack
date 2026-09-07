---
id: legacy-milestone-0017
date: 2026-07-14
recorded_date: 2026-07-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-07-14 Portal Broker And X11 Clipboard

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 389–411.

<!-- BEGIN IMPORTED BODY -->

- [x] Split sanitized policy requests from single-use, generation-bound grants
  with bounded pending/active state, deadlines, completion, expiry,
  disconnect/executor revocation, and broker-restart invalidation.
- [x] Added owner-only bounded broker IPC, persistent lifecycle across client
  connections, a default-deny deterministic policy provider, correlated
  payload frames, and fail-closed disconnect handling.
- [x] Implemented ordinary same-namespace `SelectionRequest`, restricted
  `SelectionNotify` SendEvent, owner replacement `SelectionClear`, and
  per-connection sequence/routing behavior.
- [x] Implemented cross-namespace `CLIPBOARD`/`PRIMARY` through an
  authority-private source proxy. The broker sees only sanitized namespace,
  target, size, generation, grant, and payload values; Engine and policy never
  receive XIDs or atoms.
- [x] Implemented `TARGETS`, `UTF8_STRING`, and bounded UTF-8 `text/plain`.
  Denied, stale, expired, disconnected, unsupported, and executor-failure
  paths produce normal `SelectionNotify(property = None)`.
- [x] Deterministic and socket tests prove same-namespace ownership and
  handoff, a complete broker/source-proxy/target-property transfer, stale
  generation, expiry, disconnect, default/capability denial, and executor
  failure without granting general cross-namespace resource visibility.

<!-- END IMPORTED BODY -->
