---
id: legacy-active-0017
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-08-28: the latency harness trusted the session to exit

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 577–604. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Repeated latency runs appeared to lock up the console. The harness bounds
  every wait it makes except one: after triggering injection it calls
  `wait "$PROOF_PID"` with no deadline, trusting the session to exit on its
  own. `SOPHIA_LIVE_SESSION_RUNTIME_MSEC` does not provide that exit -- with an
  input proof requested, `global_runtime_deadline_ends_session` returns false,
  so the global deadline only bounds startup and never ends a running session.
  A proof that starts but never completes therefore runs forever and the
  harness waits forever, with the operator's TTY apparently dead.
- The wait is now bounded by `SOPHIA_INPUT_LATENCY_PROOF_TIMEOUT_SECONDS`,
  ninety seconds by default, and the sample fails by name rather than hanging.
  The exit trap also terminates a live proof and injector, so interrupting a
  run at the console no longer leaves a session holding the GPU.
- Terminating means terminating the tree. The session starts children, so the
  terminator signals children first, waits, then escalates to KILL; a
  terminator that only asks politely would hang exactly as before against a
  session that is wedged rather than slow.
- The self-test for this took three attempts and is worth recording, because
  the first two passed while proving nothing. The first had only the parent
  ignore TERM: its child `sleep` died on TERM and took the parent with it, so
  the escalation never ran and removing the KILL path still passed. The second
  made the tree properly TERM-immune but then waited on it unbounded -- the
  very defect under test -- so a broken escalation hung the check instead of
  failing it, which reads as a slow pass. The check now makes every level
  ignore TERM and bounds its own wait, and removing the KILL escalation fails
  it in seconds with the reason named.

<!-- END IMPORTED BODY -->
