---
id: legacy-active-0370
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-08-02: an unarmed standing-target retirement ends launch recovery

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11334–11355. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The physical run confirmed the corrected reseed ordering: xmonad replayed
  Firefox's uncommitted manage request and Sophia admitted its exact
  1280-by-1040 fallback frame. The retained recovery constraint then delivered
  the 1276-by-1422 standing configure, and Firefox retired an exact
  1276-by-1422 PresentedBuffer. Rendering nevertheless stayed clipped to
  1276-by-1040, leaving the observed black lower region and dropping pointer
  focus handoff outside that clip.
- The exact target arrived after the constrained fallback layout had already
  committed, so it intentionally had no armed resize epoch. Native retirement
  accepted only armed candidates and discarded this otherwise conclusive
  frame. The prior regression manually called `record_committed(target)` and
  therefore never exercised the production retirement boundary.
- Native retirement now accepts an unarmed frame only when it exactly matches
  the Engine's outstanding target and the same surface still owns a temporary
  recovery extent. It records that target, clears the extent, and queues one
  constraint relayout. Old-sized frames and unarmed targets without active
  recovery remain unable to bypass a layout epoch. The updated regression
  reproduces the fallback retirement followed by the exact standing-target
  retirement; a fresh physical run remains the acceptance proof.

<!-- END IMPORTED BODY -->
