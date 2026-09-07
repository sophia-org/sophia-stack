---
id: legacy-active-0536
date: 2026-08-26
recorded_date: 2026-08-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-26: a totals check no correct run could satisfy

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16470–16502. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first signed attempt at the shell archive was a correct session. Zero
protocol errors, the complete `Super+P` proof including an inert click on the
retained pixels, nonzero presentation on both outputs, exact text, clean
teardown. It was refused at the last check: `action 30 was committed 1 times but
the guide never asked for it`.

Action 30 is `sessionBrowser`. One `Super+B` emits two records six lines apart --
`physical_action_committed action=30` and `session_action_committed
action=LaunchBrowser` -- and the guide's browser step waits on the second. The
totals check builds its expected set from `wait_for_action_count` lines alone, so
30 could never enter it while every run commits it once. The check would have
failed a perfect run every time, and this was the first real run it had seen.

That is the failure its own comment predicts: a verifier that restates an
expectation becomes a second owner of the same fact, and the file had already
been corrected once for drifting from a run it had never seen. Reading the
expectations out of the guide was the right design; the guide simply did not
state this one in the primitive that is read. Fixed where the fact is owned --
the browser step now waits on the action count as well -- rather than by teaching
the verifier a name-to-id mapping it would then own alone. The matcher fixture
needed the same line, which is the honest cost of a fixture that stands in for a
run.

The refused evidence was replayed against the corrected guide and passed, so the
defect was proven to be the check rather than the session before a second rig
session was spent. The archive still came from a fresh run: the refused evidence
names a commit whose guide lacks the fix, and an archive that cannot re-verify
against its own commit is not evidence. Signed archive `0006` followed, on a
Sophia binary whose digest is identical to the refused run's, since the fix
touched only the guide and its fixture.

<!-- END IMPORTED BODY -->
