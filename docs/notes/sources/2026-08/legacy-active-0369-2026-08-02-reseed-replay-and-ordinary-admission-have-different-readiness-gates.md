---
id: legacy-active-0369
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-08-02: reseed replay and ordinary admission have different readiness gates

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11317–11333. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The immediate physical rerun falsified the first reseed fix. Its new evidence
  emitted `reseed_queued request=relayout`, followed by the same two-surface
  workspace projection and Firefox admission timeout. The selector had reused
  `next_unmanaged_surface`, which deliberately returns no candidate while any
  rollback extent is active. That is correct for scheduling a new admission,
  but wrong after restarting a bridge whose rejected `ManageSurface` must be
  replayed before recovery can settle.
- Restart reseed now selects the oldest known, nonterminal unmanaged surface
  independently of the rollback scheduling gate. Ordinary owner-loop admission
  remains blocked by rollback. One allocation-free reducer owns the shared
  known-surface/retry predicate, and a crate-boundary regression requires a
  first-retry Firefox-shaped candidate to remain replayable while rejecting a
  withdrawn or terminal candidate. Schema-3 evidence must now report
  `request=manage` on this recovery path; physical confirmation remains open.

<!-- END IMPORTED BODY -->
