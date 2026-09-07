---
id: legacy-active-0353
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-08-01: the physical Firefox gate must carry its own operator state

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10937–10957. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The retained physical Firefox launcher described only the older eight-stage
  browser exercise, while its verifier first invoked the broader xmonad TTY3
  verifier. That unrelated verifier required a startup-Kitty exit, desktop
  relaunch, workspace-empty input, a VT round trip, pointer-edge traversal, and
  three terminal launches that the Firefox instructions never requested. A
  correct Milestone 10 run therefore could not satisfy its stated verifier.
- The Milestone 10 runner now gives exactly two Kitty processes independent
  A1/A2/A3 and B1/B2/B3 prompts. Each accepted token changes only a redacted
  title length, allowing the owner loop to record bounded checkpoints before
  Firefox, after normal `Ctrl+Q` exit, and after the restarted browser is closed
  through xmonad. Both terminals retain their visible checkpoint history.
- The offline Firefox page now displays its current next action. The physical
  verifier orders a routed axis event between the PRIMARY and DOM scroll
  stages, orders the physical layout action before the resize stage, requires
  the normal-close/restart/WM-close sequence, and retains the existing strict
  protocol, input-drain, native-presentation, frontend, authority, guard, and
  TTY restoration checks. Mutation fixtures reject missing wheel, Kitty
  retention, forced-close, and cleanup evidence.

<!-- END IMPORTED BODY -->
