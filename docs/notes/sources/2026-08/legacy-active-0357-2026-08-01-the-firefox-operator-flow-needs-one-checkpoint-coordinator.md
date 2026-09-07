---
id: legacy-active-0357
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "tooling"]
---
# 2026-08-01: the Firefox operator flow needs one checkpoint coordinator

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11020–11046. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next physical run validated the descendant projection fix. Firefox
  surface 8388611 repeatedly retired a 1280-by-1040 frame at toplevel position
  2,16 with unit scale, and the offline page completed loaded, keyboard, and
  clipboard stages. This replaces the prior global-origin child evidence and
  proves ordinary browser keyboard input reaches the correctly placed visual.
- The PRIMARY step did not describe a compositor failure. The middle button
  landed at the minimum screen edge, outside the toplevel's two-pixel chrome
  clearance, and was explicitly suppressed as `no_target`. A later physical
  wheel reached the X route, but the page correctly could not advance past the
  missing ordered PRIMARY stage. The page now tells the operator to keep both
  middle-click and wheel input well inside the colored client area.
- The two independent Kitty probes also each announced the next global action
  as soon as their own A2 or B2 checkpoint completed. Completing B2 before A2
  therefore launched and closed the second Firefox; completing A2 then
  launched an unintended third browser. Each probe now publishes private
  checkpoint markers; Kitty B waits for A1 before the first launch, both probes
  wait at the later barriers, and launch/logout authority belongs only to Kitty
  B. An executable concurrency regression feeds both clients through all three
  barriers and requires exactly two Firefox instructions and one logout
  instruction from the coordinator.
- The run exited normally with clean Firefox status and renderer teardown, but
  its deliberate fail-closed verifier rejected three of eight browser stages,
  zero selection conversions, and three launches. It is diagnostic evidence,
  not a Milestone 10 pass; the synchronized physical workflow must be rerun.

<!-- END IMPORTED BODY -->
