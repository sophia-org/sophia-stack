---
id: legacy-active-0262
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-18: Production Feedback Waits For Asynchronous Retirement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8552–8570. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The initial production adapter incorrectly modeled KMS submission and retirement
as one synchronous callback. That shape could not own the real live path without
either blocking a cycle or treating submission as page-flip completion. The
contract now has separate `submit_frame` and `poll_retirements` phases. A
`ProductionRetirement` carries its originating cycle, and protocol feedback is
routed only for records returned by the retirement poll. Submission evidence and
zero or more feedback records remain distinct in each cycle report.

Engine regressions cover ordered immediate retirement, retirement-poll failure
with no feedback, feedback failure after retirement, and a frame held across one
cycle then retired on a later poll. The live closure adapter exposes the same four
callbacks. The full offline all-feature suite passes. This establishes the
correct state-machine seam for moving `PersistentNativeScanout` and Present
Complete/Idle timing out of the CLI; the live path is not yet wired through it,
so the Milestone 6 coordinator and sequencing items remain open.


<!-- END IMPORTED BODY -->
