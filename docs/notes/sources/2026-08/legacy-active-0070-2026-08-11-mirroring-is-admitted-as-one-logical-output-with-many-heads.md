---
id: legacy-active-0070
date: 2026-08-11
recorded_date: 2026-08-11
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "tooling"]
---
# 2026-08-11: Mirroring is admitted as one logical output with many heads

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2120–2169. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Native output mirroring was an open product call the port ledger refused to
  leave implicit. The decision is to implement it before the freeze rather than
  reject the port obligation. Triad's own contribution here is only a repudiated
  screencopy prototype — an ordinary fullscreen client whose documentation
  forbids citing it as output-cloning evidence — so nothing is being ported; this
  is new capability.
- The shape is forced, not chosen. Modeled as **one logical output backed by N
  connectors**, mirroring is invisible to policy: `SnapshotOutput` carries no
  connector identity, mode, scale, or enabled flag, so there is no
  `sophia_wm_v1` wire risk and no competition for the pre-freeze window. The
  alternative shape, two logical outputs sharing surfaces, is inexpressible: it
  violates one-output-per-surface and raises `DuplicateSurface`. Recording this
  matters because the inexpressible shape is the more intuitive one.
- Joint multi-head retirement changes multi-output and buffer-lifetime
  semantics, so the standing rule to extend the bounded visual-retirement model
  first applies. That pulls Milestone 14's first roadmap item into Milestone 13.
  `VisualRetirement.tla` now carries a head layer beneath output retirement: a
  logical output retires only when its last head flips, the framebuffer stays
  leased until then, and one output in the checked configuration is a two-head
  mirror group so a single run exercises joint retirement within a group and
  independent retirement between groups. 112,252 distinct states, depth 19.
- The ratified presentation invariant is narrowed rather than dropped.
  Retirement is joint within a mirror group and independent between groups; the
  unit of retirement is still one logical output, distinct outputs still retire
  on their own page-flip timelines, and Sophia still claims no globally
  simultaneous multi-output retirement instant.
- Head loss fails the candidate closed. A connector that disappears mid-flight
  drops its lease without counting as a flip, and a surviving-head topology is a
  new candidate rather than a salvaged one. Head loss sits outside the fairness
  assumption, because nothing guarantees a connector disappears.
- Two negative controls were required to get the invariant set right, and the
  second reversed a wrong conclusion. Treating any flipped head as retiring the
  whole output violates `CommittedAfterExactRetirement`, which initially looked
  like proof that a head-level commit invariant would be redundant. It is not:
  making loss silently shrink a group corrupts the `RetiredOutputs` definition
  itself, so the invariant expressed through that definition goes blind while
  `MirrorGroupCommitsOnlyWithEveryHead`, which names the heads directly, still
  fails. An invariant stated through a derived operator cannot check that
  operator. A separate release-safety invariant was dropped as genuinely
  redundant, since `ReleasedResourcesAreTerminal` already states the condition
  directly on head-scoped `inFlight` with no definition in between.
- Mirroring is same-mode only. No plane scaling exists anywhere in the tree, so
  mismatched modes must fail closed at reconcile time rather than silently
  letterbox. Implementation work remains: lifting singular per-output connector
  selection to a set, sharing the currently exclusive buffer lease, allowing N
  heads per logical rect, exempting mirror-group members from overlap rejection,
  and a physical gate on two same-mode heads.

<!-- END IMPORTED BODY -->
