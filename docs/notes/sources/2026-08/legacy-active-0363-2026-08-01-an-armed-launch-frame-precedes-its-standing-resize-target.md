---
id: legacy-active-0363
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering"]
---
# 2026-08-01: an armed launch frame precedes its standing resize target

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11157–11176. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next physical run proved that `Super+F` and application launch were not
  the failing boundaries. Firefox started, published surface 8388611, and
  supplied exact 1280-by-1040 Present transaction 1559. Recovery selected that
  frame and emitted `visual_armed`, but no matching native submission or
  `visual_committed` followed; application admission eventually timed out.
- The launch-timeout recovery retained the blind-WM 1276-by-1422 tile as a
  standing target. Present disposition incorrectly preferred that future
  obligation over the already armed 1280-by-1040 recovery candidate and
  classified the candidate as a layout mismatch. This formed a cycle: native
  retirement was required to clear the temporary extent, while the uncleared
  standing target prevented the exact frame from reaching native retirement.
- Present disposition now bypasses a different standing target only for the
  exact armed transaction, surface, and buffer size. A later transaction with
  the same launch-sized buffer remains rejected or fenced, and the standing
  target is still discharged only by its own exact visual retirement. Focused
  tracker and live-admission regressions retain both identities; a fresh
  physical Firefox launch remains the acceptance proof.

<!-- END IMPORTED BODY -->
