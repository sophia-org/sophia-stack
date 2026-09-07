---
id: legacy-active-0025
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-08-27: the proof terminal and the launch terminal are one application

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 859–896. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The second native attempt reached the session, presented, and accepted the
  typed phrase at 34 of 34 events. Both `Super+Return` presses then worked
  exactly as specified -- admitted, committed, `LaunchTerminal`, started,
  surface observed -- and each new window vanished immediately. The operator saw
  a shortcut that did nothing; the evidence shows two terminals reaching
  `surface_observed` and then `normal_exit_after_surface` with exit status 0.
- The startup terminal runs the guide as its command, and one application id
  carries one argument list, so every terminal the workflow launched ran its own
  copy of the guide. Each copy found an evidence file that already satisfied
  every wait it had -- the phrase completes before the first launch, by the
  guide's own ordering -- ran to the end, and exited. A terminal exits when its
  command does.
- Splitting them into two applications is the obvious repair and Sophia refuses
  it. A normal session with a physical text proof requires the terminal action
  to name its single startup application (`startup_terminal` in
  `PersistentXtermSessionConfig::from_args`) and otherwise rejects the whole
  argument set as proof-only. The third attempt ended there, before any window
  appeared. The rule is deliberate: the proof types into the session's terminal,
  so the terminal it proves must be the terminal the session actually launches.
- The differentiation therefore belongs inside the command, not in the session
  model. The gate creates a claim file per run; the guide claims it with
  `set -C` and any instance that cannot claim it `exec`s an ordinary shell. One
  application, one argument list, and only the startup terminal drives the
  proof.
- Three argument-validation failures in a row cost three physical attempts, and
  nothing offline reaches the real parser: `sophia-live-session` validates its
  arguments only on the way to taking DRM, and there is no check-only entry
  point. The matchers now restate the two rules that were violated -- an input
  proof needs a bounded runtime, and the terminal action must not be overridden
  -- and test the stand-down behaviourally. A check-only argument mode would
  retire those restatements and is worth considering on its own merits.
- The run ended by VT switch rather than logout and is not promotion evidence.
  Nothing in the WM, shell, frame-slot, or presentation path was implicated:
  layout committed eighteen times, focus reconciled fourteen, and the two stale
  responses recovered.

<!-- END IMPORTED BODY -->
