---
id: legacy-active-0065
date: 2026-08-12
recorded_date: 2026-08-12
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "security"]
---
# 2026-08-12: Output power is a separate authority; reservations mostly were not missing

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1943–1981. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The output-authority ledger row names four things beyond the topology machine:
  test, apply, rollback, reservations, and a separate power authority. Working the
  last two showed one was genuinely absent and one was mostly already built.
- **Power is now its own authority.** `OutputPowerState` holds per-output levels for
  the outputs the desktop currently has. The reason it is separate from enablement
  is that blanking a screen and removing a monitor are different facts: a dark
  output keeps its bounds, work area, and surfaces, and policy must not see the
  transition at all, while a disabled output leaves the complete snapshot and forces
  a relayout. The temptation to merge them is not hypothetical — atomic modesetting
  powers a head down by clearing the CRTC's `ACTIVE`, the very property that
  disables one. That is a fact about the commit, and letting it leak upward would
  make "blank the screen" and "unplug the monitor" the same operation to every layer
  above.
- Two rules fall out of admission. A topology change keeps the level of every
  surviving output, so a mode change on one monitor cannot relight another that an
  idle policy powered down; and an output that leaves keeps no level, so a
  reconnected monitor cannot inherit a stale one and come back dark for no traceable
  reason. Both are the same reconciliation, and both are tested.
- **Reservations were largely present.** Work areas are already re-projected from
  the new output rects inside the topology publication that swaps the outputs, so
  geometry and work area commit together on the hotplug path. What was missing was
  any test pinning it. The plan assumed a gap that the code had already closed;
  reading the runtime path before writing the fix is what turned a rewrite into a
  regression test.
- **One fail-open edge, documented rather than fixed.** Reservations are
  root-relative, so shrinking an output can leave one outside the new root. Such a
  reservation is filtered *before* reduction, so reduction succeeds and reports the
  full output as available — silently releasing it. Between the mode change and the
  reserving client republishing, policy may lay out under a bar that still occupies
  pixels. The fail-closed path exists immediately next door: a reduction returning
  `None` makes callers preserve the previous work area. The pure reducer cannot
  distinguish a malformed reservation from one invalidated by a geometry change,
  because it holds no previous state, so closing this belongs at
  `SurfaceOutputReservationState`. It changes behavior for every bar, which is why
  it is recorded as a decision to take rather than taken at the end of a long
  session.

<!-- END IMPORTED BODY -->
