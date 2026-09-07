---
id: legacy-active-0317
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-07-25: Pending Layout Commit Merges Concurrent Surface Admission

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9922–9940. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first run after per-output hardening reached the four-window transition,
then exited with `new WM surface is missing from live layout`. Rapid
Super-Enter actions admitted another Kitty while an older resize proposal was
waiting for matching pixels. Authority intake inserted the new surface into
the live layout and unmanaged set, but committing the older proposal replaced
the layout with its pre-admission snapshot. The unmanaged ID survived without
its layer, so the next manage request correctly rejected the inconsistent
state.

Pending layout snapshots now merge every authority observation not owned by
that proposal's requested-size set. A concurrently admitted surface is
preserved for the next blind-WM manage request, and ordinary pixel updates for
unrequested surfaces advance with the pending snapshot. Resize-owned surfaces
remain quarantined until their matching pixels arrive. The merge is a pure
data reducer with integration coverage for insert, replace, and resize-owned
outcomes.

<!-- END IMPORTED BODY -->
