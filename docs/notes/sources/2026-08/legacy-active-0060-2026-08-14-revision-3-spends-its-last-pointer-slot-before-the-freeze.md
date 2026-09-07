---
id: legacy-active-0060
date: 2026-08-14
recorded_date: 2026-08-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-14: revision 3 spends its last pointer slot before the freeze

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1796–1812. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The freeze analysis had already fixed four interaction kinds and four phases,
  but the code still exposed only move/resize and still rejected the proposed
  axis slot as reserved. Leaving the decision in prose would have made the frozen
  enum permanently smaller than the retained Triad input vocabulary.
- `PolicyInteractionKind` now fixes `Drag = 3` and `Scroll = 4`. The existing
  phases remain `Begin`, `Update`, `End`, and `Cancel`; a fifth kind or phase
  requires a new interface family after the freeze.
- `ProjectionRequest.reserved_cause` is consumed in place as
  `interaction_axis`: zero for move/resize/drag, one horizontal, two vertical.
  Scroll reuses X/Y as signed deltas and requires zero width/height. This changes
  no offset or payload size. The semantic packet carries the axis explicitly so
  Rust, C, and Nim clients agree without retaining an unnamed wire value.
- This entry closes only the irreversible vocabulary decision. The separate live
  move/resize coalescing and security-cancellation tranche is recorded above.

<!-- END IMPORTED BODY -->
