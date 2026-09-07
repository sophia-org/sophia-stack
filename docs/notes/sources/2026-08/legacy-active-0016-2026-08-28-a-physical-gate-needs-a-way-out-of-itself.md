---
id: legacy-active-0016
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-28: a physical gate needs a way out of itself

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 554–576. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Running the latency harness left the operator with no working keyboard or
  mouse until the run ended. Sophia is handed only the uinput virtual device,
  so it never reacts to the real seat, but it does take DRM master: the console
  is behind its output, the session ignores physical input by design, and there
  is nothing to press. Before the wait was bounded that state was permanent;
  bounding it only shortened the wait to ninety seconds.
- The xmonad runner has always armed an emergency input guard for exactly this,
  and the latency harness armed nothing. It now arms the same guard on the real
  seat before any sample takes the display, and aborts the run when the operator
  presses Ctrl-Alt-Backspace, terminating the proof and its children rather than
  waiting out the deadline. As in the xmonad runner, arming requires one press
  up front, so the escape is proven to work before the display is taken.
- The guard listens without stealing: there is no `EVIOCGRAB` on the libinput
  path, so arming it does not itself deprive the console of keys. That mattered
  to check, because a guard that grabbed the seat would have made the symptom
  worse while appearing to address it.
- The general point is worth keeping. A gate that takes the display owes the
  operator an interrupt, and every physical harness here should be read for
  whether it has one. Bounding a wait fixes a hang; it does not give a person
  sitting in front of a black screen anything to do.

<!-- END IMPORTED BODY -->
