---
id: legacy-active-0447
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-16: scanout cloning decided as plan-equivalence, geometry only

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13406–13442. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Research question: whether Sophia should adopt the macOS mirroring shape —
  a logical-monitor model on top, opportunistic scanout cloning underneath,
  render copy as the universal fallback with automatic invisible switching.
  Mutter is the reference for the logical model (three-level configuration,
  mirror sets constrained to a common mode, so it never needs the strategy
  switch). Weston's DRM backend is the reference for the mechanics
  (atomic-first, `TEST_ONLY` probing, opportunistic plane promotion per
  element, not per head). Neither implements the automatic arbitration; that
  arbitration is the part Sophia would add.
- Decision: adopt the shape, post-promotion. Clone eligibility is equivalence
  of `HeadCompositionPlan` records within one logical output — the whole
  record except head identity and content checksum — plus identical modes
  including refresh and single-card membership. The decision is derived from
  existing passive data, lives in the backend, and is invisible to every
  policy surface.
- Decision: the content checksum is excluded from the predicate. A checksum
  only exists after composition, but the strategy must be chosen before it;
  and once cloned there is one composition, so no second checksum exists to
  compare. Content-inclusive eligibility could therefore promote cloning but
  never detect the need to demote it. Content identity is instead guaranteed
  by compositor determinism given geometry-equivalent plans over one scene
  generation — the same invariant the exact-density gates already rely on —
  and that guarantee is proven once, with evidence, by a dual-render audit in
  the clone gate rather than checked in production.
- Decision: any field later added to the plan record is equivalence-relevant
  by default. Unconsidered head-local state then disables cloning silently
  instead of wrongly preserving it; fail closed points in the safe direction
  without anyone remembering the clone path exists.
- Ordering consequence: the unequal-mode mirror gate exercises exactly the
  configuration cloning can never serve, so proving per-head render copy
  first is the load-bearing half of the macOS design, not a detour. The
  optimizer may decline a clone, never a topology, and every eligibility
  input changes only through `OutputTopologyTransaction`, so switching is
  configuration-cadence by construction.

<!-- END IMPORTED BODY -->
