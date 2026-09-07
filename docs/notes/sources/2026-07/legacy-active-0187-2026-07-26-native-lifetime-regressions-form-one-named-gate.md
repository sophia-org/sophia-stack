---
id: legacy-active-0187
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-07-26: Native Lifetime Regressions Form One Named Gate

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6345–6367. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The remaining Milestone 9.1 regression item did not require a new lifecycle
owner. Its constituent failures already reduce through focused deterministic
tests at the boundaries that own them:

- CLI resize coordination tests compensating rollback, abandoned-pixel
  fencing, disconnect cleanup, and stale prepared-frame settlement.
- Backend startup tests output/target size mismatch, replacement, stale target
  allocation removal, and reduced target retirement.
- Backend presentation and scanout tests accepted replacement, stale callback
  rejection, cleanup retry, final displayed-owner cleanup, and repeated
  retirement as a no-op.
- Renderer lifetime tests stale CPU retirement and reusable DMA-BUF retirement
  without exposing renderer-native handles.

Validation now gives these tests one named command set. This preserves DRY
ownership: rollback remains transaction policy, target replacement remains
backend readiness, and native resource destruction remains KMS/backend state.
The gate closes the deterministic Milestone 9.1 item, but it does not replace
the same-commit physical xmonad run, whose balanced complete-target,
frame-surface, page-flip, and teardown counts remain authoritative.

<!-- END IMPORTED BODY -->
