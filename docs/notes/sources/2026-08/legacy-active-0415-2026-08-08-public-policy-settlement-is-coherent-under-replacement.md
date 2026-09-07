---
id: legacy-active-0415
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-08: public-policy settlement is coherent under replacement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12602–12638. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Scenario-driven code analysis found two coupled recovery defects in the
  first live Hagia path. Process replacement could discard a staged reducer
  successor while frontend layout settlement still owned its identity, and
  the prepare hook promoted reducer authority before the matching layout
  commit. The latter exposed a transient last-good disagreement and could
  survive a transport failure.
- Restart now terminates the old transport, makes it unavailable, forces the
  exact pending layout through its ordinary timeout/abort reducer, and admits
  a new connection epoch only after settlement ownership clears. Prepare now
  performs non-mutating staged revalidation; the owner-loop commit advances
  reducer and layout authority together.
- Terminal configuration, projection, and session-operation outcomes retain
  one owner-side deferred command when the capacity-one worker channel is
  busy. An old-epoch command is discarded on confirmed transport loss and can
  never cross into the replacement peer.
- `PolicySettlementRecovery` exhaustively checks 224 distinct states to depth
  36. Its invariants cover coherent last-good serials, failure preservation,
  old-owner clearance, and at-most-once terminal delivery. A temporary
  prepare-time promotion produced the expected `LastGoodIsCoherent`
  counterexample before the corrected model passed.
- Sophia now lowers one completed Engine-owned pointer gesture to a final,
  bounded interaction cause. Hagia validates phase, target generation,
  capability, output, and region before storing private floating geometry.
  Hagia regressions also retain repeated action identity, opaque session
  operations, atomic cross-output movement, output return generation, and
  reconnect behavior.
- `tools/check_policy_client_matrix.sh` completes one offline matrix across the
  Rust reference, independent C client, Hagia, and X11 bridge. Packaging can
  include an explicitly supplied Hagia binary and a separate installed login
  profile. A phase-anchored fault after the second submitted Hagia projection
  passed bounded restart, startup, session-health, and layout-health checks.
  Exact live injection after owner-side staging, the physical installed
  workload, shared behavioral freeze corpus, default promotion, and API-v7
  removal remain deliberately open.

<!-- END IMPORTED BODY -->
