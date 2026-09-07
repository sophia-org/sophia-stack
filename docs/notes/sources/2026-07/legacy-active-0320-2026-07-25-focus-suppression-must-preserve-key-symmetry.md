---
id: legacy-active-0320
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-07-25: Focus Suppression Must Preserve Key Symmetry

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9996–10037. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical run with asynchronous focus controls completed cleanly, and
all 28 controls were delivered without rejection or timeout. Keyboard hardware
also remained active. The remaining apparent keyboard loss was an input-state
ordering defect: a key press could reach the old X client immediately before a
focus handoff, while its physical release arrived during the deliberately
suppressed Engine/frontend focus mismatch. The WM observed that release, but
the X client did not, leaving keys such as Super logically pressed.

The live input boundary now retains a fixed-capacity data record of key presses
actually delivered to each surface, seat, and device. Before a focus,
clear-focus, VT, seat-release, or logout handoff, it sends a release for every
record owned by the old client and includes those releases in normal delivery
accounting. A later physical release without a matching delivered press is
suppressed instead of being sent to the new client. Surface removal clears any
remaining record and updates the local modifier reducer, preventing stale
state from crossing a client exit.

Completion evidence reports peak pressed-key depth, synthetic releases,
suppressed orphan releases, surface-removal cleanup, and final debt. The
four-Kitty verifier requires final pressed-key debt to be zero.

The close-window physical proof refined the ordering requirement. X authority
owns one XKB reducer per seat, not one per surface. Clearing two local records
when Meta-Shift-C removed its surface left Meta and Shift pressed in that
seat-wide reducer, so the replacement Kitty inherited modified input. Control
dispatch is now held behind the exact synthetic-release delivery IDs. A close
or focus request cannot reach its X control writer until every preceding
release has been acknowledged by X authority. Completion and the physical
verifier require both the pressed-key ledger and this release barrier to be
empty, with no keys abandoned during surface removal.

Client-initiated exits add a different boundary: a terminal may destroy its
surface in response to Return before the physical Return release arrives.
There is then no live target for an X event, but the seat-wide XKB reducer must
still observe the release. Routed input now distinguishes ordinary
Engine-selected delivery from a state-only seat update. Surface removal emits
state-only releases for its residual key records; X authority updates XKB
without resolving a surface or emitting an event to the newly focused client.
This preserves global keyboard state without inventing an application target.

<!-- END IMPORTED BODY -->
