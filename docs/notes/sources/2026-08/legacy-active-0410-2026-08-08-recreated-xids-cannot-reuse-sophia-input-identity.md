---
id: legacy-active-0410
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-08: recreated XIDs cannot reuse Sophia input identity

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12499–12520. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- X wire decoding necessarily represents the client's current resource by its
  raw XID, but CreateWindow had also projected every Sophia `SurfaceId` with
  generation one. Destroying and recreating the same XID could therefore make
  an old deferred input route resolve to the replacement.
- Each admitted X11 client now owns a private generation ledger keyed by its
  resource index. A CreateWindow request receives the next candidate Sophia
  identity, and the ledger advances only after dispatch accepts that creation.
  Rejected creates can retry the same candidate; stale, skipped, or exhausted
  generations fail closed.
- Investigation exposed the other half of the ABA path: DestroyWindow removed
  the Engine surface but left its frontend route registered until disconnect.
  Every successful response carrying removed surfaces now deletes those exact
  surface/window and routed-input entries before later requests are observed.
- Unit coverage checks monotonic admission and overflow. A socket lifecycle
  regression observes create generation one, DestroyWindow retirement, and
  same-XID generation two. A frozen-input regression proves an old thawed route
  is discarded while fresh input to the replacement still routes normally.
  Engine-visible grab leases, slow-client queue isolation, output-local pointer
  domains, and focus-handoff revalidation remain separate work.

<!-- END IMPORTED BODY -->
