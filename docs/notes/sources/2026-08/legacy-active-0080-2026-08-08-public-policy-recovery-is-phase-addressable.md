---
id: legacy-active-0080
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "policy"]
---
# 2026-08-08: Public-policy recovery is phase-addressable

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2724–2749. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The previous Hagia restart smoke killed the client after it submitted a
projection. That proved supervision and eventual reseeding, but timing could
not establish whether the owner had only staged the reducer successor, had
installed a frontend layout, or had already queued the terminal outcome.

The live owner now admits one explicit bounded proof control with four named
boundaries: `proposal_staged`, `frontend_pending`, `prepared`, and
`terminal_outcome_queued`. The control requests the ordinary supervised
transport restart and is consumed once; it does not mutate reducer or layout
state directly. `tools/hagia_owner_settlement_fault_smoke.sh` ran every point
against real Hagia and Kitty. Both settlement-bearing cases recorded
`settlement_aborting`, exact layout timeout/abort, epoch-2 restart, later
startup readiness, and clean session/layout health. The staged and terminal
cases also restarted once and drained cleanly.

The complementary `PolicyOutputSettlement` model covers the remaining
topology mechanism before dynamic output ingress exists. An output loss or
identity return advances the canonical scene, a stale prepared candidate
cannot replace either half of the last-good reducer/layout pair, and return
increments the output generation. TLC explores 86 generated and 64 distinct
states to depth 13. Removing the final topology recheck produced the expected
seven-state stale-commit counterexample; suppressing generation advancement
produced an output-ABA counterexample in three states.

<!-- END IMPORTED BODY -->
