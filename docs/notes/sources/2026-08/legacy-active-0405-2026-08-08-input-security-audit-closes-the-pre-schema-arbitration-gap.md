---
id: legacy-active-0405
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security"]
---
# 2026-08-08: Input security audit closes the pre-schema arbitration gap

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12322–12382. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first target model overstated its proof. Its release action guarded out
  stale activation, its disclosure action guarded out missing grants, its
  two-scene bound hid A→B→A handle reuse, and the pacing model recorded string
  tags rather than actual stream values. Cancellation could also become
  disabled at a lifetime counter bound. Those models established their own
  construction, not the advertised adversarial properties.
- The audit also found contract gaps outside those abstractions: a recipient
  shell could declare its own coordinate grant; targets lacked presented
  ownership, occlusion, and overlap admission; disclosure was not correlated
  to a live seat/device/contact; output and authority epochs were absent; and
  no rule prevented a frontend grab from carrying application coordinates over
  privileged shell or lock pixels.
- Current application routing is not already harmonized with the shell rule.
  Production input layers derive from committed surfaces, ordinary events are
  re-hit-tested before namespace-local grab lookup, one stalled client queue
  can fail the frontend service, and XID reuse recreates `SurfaceId` with
  generation one. These are runtime debts, not changes made in this
  documentation/formal pass.
- The corrected contract makes application grabs Engine-visible
  profile-scoped route leases. A secure transition revokes them immediately;
  a normal move outside admitted application scope waits for frontend release
  acknowledgement before shell capture. Fresh application and shell selection
  must use the applicable last-presented snapshot before the routes coexist.
- Coordinate disclosure now requires a capability issued by independent
  session or portal policy and bound to authority/session, target generation,
  output/region, seat/device class, precision, rate, expiry, and revocation
  epoch. Normal visual removal becomes effective on presentation; policy or
  security revocation is immediate and discards queued old-epoch data without
  sending a final value to the revoked endpoint.
- Targets are admitted only inside their authority's presented visual
  allocation after occlusion and deterministic overlap ordering. Identity is
  monotonically generational across authority sessions, and capture includes
  the initiating device/contact. Precommitted visual alternatives share fixed
  bounds and meaning; no concrete variant wire node is ratified.
- `TargetResolvedInput` now retains immutable scene history, uses three scene
  generations, models multiple ordered targets and independently issued local
  grants, and records attempted release/disclosure facts for falsifiable
  invariants. `TargetInputPacing` represents optional values so zero and
  no-motion completion are valid, reserves final-boundary capacity, models
  paced flush and fail-closed recipient epochs, and distinguishes normal from
  security cancellation. `InputAuthorityArbitration` separately covers
  presented selection, profile-scoped leases, release acknowledgement,
  reserved shortcuts, secure preemption, and old-epoch queue quarantine.
- Temporary negative controls produced the intended counterexamples: routing
  against committed state violated `CapturedTargetsArePresented`; recyclable
  generations violated `GenerationsNeverRecycle`; missing grant checks
  violated `CoordinatesAreAuthorizedAndLocal`; wrong-device activation
  violated `ActivationsMatchCapturedPresentedTarget`; bypassed topmost or
  output-loss cancellation violated `CapturedTargetsArePresented`; a fabricated
  final value violated `FinalValuePrecedesNormalBoundary`; removing drain
  fairness violated the pacing temporal property; widening a grab violated
  `ApplicationLeasesAreProfileScoped`; and retaining queued input through a
  secure transition violated `SecurityStateHasNoCaptureOrQueuedInput`.
- With the pinned TLA+ Tools 1.7.4 jar, the clean target model completed
  exhaustively across 5,518,840 distinct states to depth 20, pacing completed
  across 19,200 states to depth 14 including its fairness property, and
  arbitration completed across 26,560 states to depth 21. The complete pinned
  Sophia TLA gate passed with all three models registered.

<!-- END IMPORTED BODY -->
