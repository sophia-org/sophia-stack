---
id: legacy-active-0560
date: 2026-08-29
recorded_date: 2026-08-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-29: the same stall, the other exit

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17410–17436. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The shared-mode run reached thirty-four of thirty-five samples. The retry
absorbed a stall at sample five exactly as intended, and then sample
thirty-four ended the run with the same fault and seven retries unspent.

The classifier was right and unconsulted. A stall that kills a session before
it asks for physical input takes an earlier exit -- the one that waits for
readiness, sees the process gone, and fails with "exited before requesting
physical input" -- and that path knew only about the cursor-permission retry.
Sample five's stall landed after readiness and was redone; sample
thirty-four's landed at startup, on head one with zero retirements, and was
not. The budget was never the constraint; the wiring was.

Both failure paths consult the classifier now and share one budget, because
they are one fault: a display that stopped completing flips, wherever in the
session it stopped. The self-test asserts that both consult it, which is a
structural check rather than a behavioural one and is the only kind that
would have caught this -- the classifier itself was correct and its own tests
all passed while a run died.

The thirty-three completed samples continue to say sharing costs nothing:
105 presses at one renderer thread, full chain p99 22 ms against the 34 ms
budget, dwell-to-submit p99 7 ms, submit-to-page-flip p99 18 ms. Three
partial runs now agree within a millisecond of each other and of the
dedicated baseline. Only the sample count has kept a full run from saying so.

<!-- END IMPORTED BODY -->
