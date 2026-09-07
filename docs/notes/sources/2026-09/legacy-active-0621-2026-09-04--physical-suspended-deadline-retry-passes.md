---
id: legacy-active-0621
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-09-04 — Physical suspended-deadline retry passes

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19975–19995. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The operator reports "seems to be working" on the fixed candidate. The run in
`.artifacts/diagnostics/cp14-3-mixed-source-20260905T005021Z/resume/` exited 0
after 90,542 ms without the earlier `MissingSource` failure. Native epoch 1
closed on seat release, settled with 3,427 submissions, 3,425 retirements,
zero submit/retire/settlement failures, and no remaining native work. The seat
then suspended. Runtime-deadline quiescence completed in 539 ms with zero
authority, coordinator, CPU, or native work pending and no new owner opening.
The final record retained native presentation and retirement totals; application
and frontend cleanup was clean. TTY handoff restored the manager's safe baseline
and reported both origin and manager input usable, with the manager ready.

Credit this as the suspended-deadline canary based on the recorded behavior,
despite its `resume` directory name and 90-second rather than 60-second budget.
The VT-return check remains open: there is no epoch 2 or post-resume retirement.
The operator's general observation does not establish an exact replay of the
previous Firefox tab gesture or separate visual acceptance of both outputs.
Preserve this successful evidence and run only the remaining VT-return check;
no comparison rows or unrelated workflow evidence are reset.

<!-- END IMPORTED BODY -->
