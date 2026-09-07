---
id: legacy-active-0026
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-08-27: an input proof needs a startup budget, not a session lifetime

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 897–923. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first native gate attempt never reached a window. Sophia refused its
  arguments with `input proof flags require --max-runtime-ms or --max-ticks for
  a bounded proof`, restored greetd, and returned the TTY exactly. No DRM work
  happened, so this is launcher evidence rather than a session result.
- The switcher gate satisfies that rule through
  `live_session_persistent_hardware_proof.sh`, which turns
  `SOPHIA_LIVE_SESSION_RUNTIME_MSEC` into `--max-runtime-ms`. The native gate
  runs its session through the ordinary `hagia` runner instead, and the runner
  passes no runtime bound because ordinary sessions have no lifetime. The bound
  had no owner on the new path.
- The bound is a startup budget, not a session lifetime, and that is why it is
  compatible with ending on a normal logout:
  `global_runtime_deadline_ends_session` returns `!input_proof_requested`, so
  with a proof requested the deadline never ends the session. It bounds the wait
  for the first focused terminal frame the proof types into, and fails closed
  with a named error if that frame never arrives. Everything after the phrase is
  owned by narrower stage deadlines, so the operator's logout remains the only
  thing that ends the run. The gate passes 660000 ms through
  `SOPHIA_HAGIA_NATIVE_STARTUP_BUDGET_MSEC`.
- `check_hagia_native_matchers.sh` now refuses a gate that requests an input
  proof without a runtime bound. It restates a rule that already lives in
  `PersistentXtermSessionConfig::from_args`, which is duplication worth keeping:
  nothing else offline reaches the real argument parser, and the omission cost a
  physical attempt to discover.

<!-- END IMPORTED BODY -->
