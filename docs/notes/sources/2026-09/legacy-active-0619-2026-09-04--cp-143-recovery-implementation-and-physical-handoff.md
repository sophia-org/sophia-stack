---
id: legacy-active-0619
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-09-04 — CP-14.3 recovery implementation and physical handoff

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19877–19926. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The row-10 failure is addressed at the native owner boundary. Session evidence
now retains retired scanout lifetimes across VT release/resume, rejected and
timed-out switches, startup recovery, topology rebuild, and final completion.
Counters sum disjoint lifetimes, maxima use maxima, current resource gauges stay
separate, and bounded cost populations merge before percentile calculation.
Failed drains and outstanding ownership remain sticky. The new versioned
`sophia_live_native_owner` record names each epoch and closure outcome; existing
summary schemas and WM/shell/output wire contracts are unchanged.

CPU visual progress resets native frame/submission baselines after observing the
old owner's final state, without dropping completed history or unbound scene
updates. Input-latency observations abandon pending cross-owner joins explicitly;
the Firefox pixel verifier also keys joins by owner lifetime. A later frame with
a recycled ID cannot complete an earlier owner's measurement.

The owner loop services input acknowledgments and runtime deadlines before seat
waits. Recovery rechecks the remaining budget after old-owner drain. Suspended
sessions can drain frontend removals, coordinator work and CPU lifecycle owners
through existing bounded quiescence. Native suspension is distinct from headless
presentation: software Presents are skipped and input projections stay revoked.
No new rendering owner is admitted during deadline/key drain or quiescence.

`NativeSessionLifecycle` was checked before the runtime edits: 330 distinct
states, with safety and deadline liveness satisfied. Its three negative controls
fail for discarded counters, forgotten failures, and shutdown requiring resume.
The counterexamples correspond to the diagnosed completion/lifecycle paths,
not a new speculative incident. The model and controls are wired into
`tools/check_tla.sh`; focused run logs are retained in
`/tmp/sophia-recovery/spec/output/`. Existing X authority shutdown models continue
to own detailed accepted-work settlement.

Verification: `cargo xtask check` passed, including 2,348 passing test executions,
reader/mutation fixtures, archived evidence checks, and render-node buffer-age
pixel equivalence. The focused software Present suite passes all eight tests,
including suspended presentation and revoked input. Firefox verifier fixtures
accept separate complete epoch joins and reject a cross-epoch retirement. The
native-session release build succeeded. Initial sandbox-only session tests could
not bind four role sockets; the full gate passed with the required local access.

The [two physical procedures](../../../native-recovery-canary.md) are prepared: Firefox
with VT return and new input/presentation before the deadline; Firefox suspended
through its deadline, with shutdown completing without reacquisition or watchdog.
Neither has been run. Stage 1's physical acceptance and CP-14.3 remain open. No
comparison row was rerun or modified; the existing row-10 partial and sealed
samples retain their original candidate identity. No live session was started or
installed by this implementation.


<!-- END IMPORTED BODY -->
