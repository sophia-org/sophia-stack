---
id: legacy-roadmap-0010
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 13.1 Ratify And Model The Boundary

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 584–605.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0009-milestone-13-public-policy-protocol-and-hagia.md).

<!-- BEGIN IMPORTED BODY -->

- [x] Reconcile the architecture, WM API, Hagia design, specification draft,
  and research log around one language-neutral policy protocol. Mark the
  current workspace-oriented Rust API v7 experimental and reserve
  `sophia_wm_v1` for the first stable public projection interface.
- [x] Add bounded `PolicyConnection` and `PolicyProjection` TLA+ models before
  changing production IPC or Engine policy state. Check negotiation,
  capabilities, transfer assembly, connection epochs, stale proposals,
  multi-output atomicity, focus, timeout, disconnect, restart, and
  last-committed projection preservation.
- [x] Map every model action to its owning Rust boundary. Preserve each
  implementation-relevant TLC counterexample as a deterministic Rust
  regression before correcting the implementation or model.
- [x] Audit retained Triad capabilities against Sophia, Hagia, River, and Niri.
  Keep spatial policy in Hagia; keep input, client settlement, rendering, and
  scanout in Engine; reserve separate session, shell, broker, and portal roles.
- [x] Extend the policy models for ordered action causes, policy-initiated
  reprojection, configuration generations, frontend settlement, reduced
  pointer interactions, and opaque session-operation outcomes before adding
  those transitions to the draft wire.

<!-- END IMPORTED BODY -->
