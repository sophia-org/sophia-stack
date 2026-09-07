---
id: legacy-active-0208
date: 2026-07-19
recorded_date: 2026-07-19
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-07-19: Milestone 8 Close, Sequence, And Soak Results

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7081–7101. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Firefox's intermittent XCB abort was an output-order race: asynchronous writers
could snapshot sequence N, wait behind a reply for N+1, then emit an event
carrying N. Request-sequence publication and every protocol, control, and input
event snapshot are now serialized by the output socket lock.

Close routing selects an exact `WM_DELETE_WINDOW` target, its nearest
protocol-advertising ancestor, or the only unambiguous protocol window. A
client with no cooperative target follows the bounded terminate path. The soak
also established bounded launch/focus settlement, close retries during busy
layout proposals, stale-surface WM resynchronization, and explicit discard of
the terminal logout chord's undeliverable release batch.

The final unattended QEMU run lasted 1,891,936 ms and completed 22 terminal,
Firefox, and GTK launcher cycles with 66 closes, 11 recovered bridge restarts,
all six Firefox semantic stages, zero unexpected protocol errors, no pending
WM/action/input state, and clean application, frontend, namespace, Xauthority,
and native presentation teardown. Three consecutive mixed-application runs
also passed before the soak.

<!-- END IMPORTED BODY -->
