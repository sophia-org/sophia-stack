---
id: legacy-active-0396
date: 2026-08-04
recorded_date: 2026-08-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "tooling"]
---
# 2026-08-04: installed recovery keeps the deadline outside Engine

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12022–12043. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The development launcher already placed Sophia in a fresh process group and
  kept its optional wall-clock deadline in the outer shell. That topology is
  the correct recovery boundary: the deadline can terminate the complete
  graphical group, while the surviving launcher still owns the saved keyboard,
  KD, and terminal state. Duplicating this timer in Engine would make recovery
  depend on the component being contained.
- A daily desktop cannot carry a finite wall-clock lifetime. The immutable
  release now adds a separate `Sophia Recovery Proof` greetd entry whose thin
  installed wrapper alone selects a 45-second deadline. The normal xmonad,
  Kitty, and Firefox entries leave the deadline unset.
- Watchdog containment is not a clean logout. Its verifier requires visible
  startup readiness, one exact process-group deadline, an armed but untriggered
  local guard, restored VT state, and an installed status-124 display-manager
  handoff. The recorder archives this evidence separately from status-zero
  login cycles and status-130 graceful emergency-chord runs.
- Staged-install regressions require the wrapper, operator recorder, desktop
  entry, current-release symlinks, and rollback survival. Fixture regressions
  reject a changed deadline, local-chord substitution, graceful-shutdown
  relabeling, a missing status-124 diagnostic, and a normal lifecycle handoff.

<!-- END IMPORTED BODY -->
