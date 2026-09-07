---
id: legacy-archive-0022
date: 2026-07-07
recorded_date: 2026-07-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-07-07: Engine Backend Boundary

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 382–391. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Use niri and Smithay as references for compositor backend boundaries, not as a
base or dependency.

Sophia Engine now treats the headless compositor as the first backend behind a
small `EngineBackend` trait. This keeps backend mechanics separate from passive
protocol packets and leaves room for real output, XComposite import, and test
backends without changing the WM or X Bridge packet shapes.

<!-- END IMPORTED BODY -->
