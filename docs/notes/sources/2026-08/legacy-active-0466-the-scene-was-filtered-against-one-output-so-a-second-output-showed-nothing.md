---
id: legacy-active-0466
date: 2026-08-18
recorded_date: 2026-08-18
date_basis: first-heading-commit
date_commit: ddcc5ff3fe809e5c5b1aa1d08f4c192068460e78
committed_at: 2026-08-18T20:19:36-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# The scene was filtered against one output, so a second output showed nothing

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14146–14190. The heading has no date. Its first recorded addition is commit
`ddcc5ff3fe809e5c5b1aa1d08f4c192068460e78` (2026-08-18T20:19:36-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The gate finally reached its telemetry stage: readiness at 256 ms, the whole
topology lifecycle committed and settled, health clean,
`content_ready source=stable_present_scanout` twice, and presents reported
`status=stable` with real pixel counts. What it failed on was the extended
head, which never showed a single client pixel in thirty seconds.

Transaction 597 tells the story on its own. It appears exactly once in the
evidence -- `visual_armed epoch=14 surface=4194318 width=1920 height=1080` --
and then never again. Not staged, not composed, not retired, not rejected. Its
sibling on the mirror output, armed in the same instant, committed normally.

The presentation layout is built by filtering every layer through
`surface_visible_on_output(layer, output.id)`, where `output` is the primary
output alone. While the desktop had one logical output that question was
harmless. A mixed topology makes it wrong: the policy placed one surface on
the mirror group and one on the extended output, and the extended one was
filtered out of the scene entirely. From there the deadlock is short. A staged
Present is released only when its surface is in the projection; the surface is
in the projection only if it is composed; and the resize that would place it
on the extended output commits only when that Present retires. The head stayed
blank, and the client -- still waiting on a Present that could never complete
-- stopped drawing, which is what made the terminal look dead to the keyboard.
The keys were observed, routed, and flushed; the window they went to could not
repaint.

One scene spans every output, so the question is whether any live output shows
the surface. `policy_projections_place_surface` now answers it over a set of
outputs, and per-head plans continue to decide by geometry which head shows
what -- that part was always right.

The gate itself needed one correction, in the opposite direction from the
usual. It required the extended head to prepare an unsampled exact frame
during the topology apply and looked for a `head_composition_plan` line to
prove it. No such line can exist there: the first topology frame is composed
from the last committed scene rather than from a fresh engine plan, exactly as
the check own comment says, and plans are emitted where the engine binds scene
layers to a head. The evidence that does exist is the queued candidate, which
names the head mode and its exact mapping. The check now reads that, and
separately requires every plan after the commit -- where plans do exist -- to
be unsampled and free of fallback. Inventing a plan-shaped line for the
install path would have meant deriving the same fact twice, which is the
defect class this log keeps recording.

<!-- END IMPORTED BODY -->
