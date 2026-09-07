---
id: legacy-archive-0013
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "security"]
---
# 2026-07-10: xclock as External X Authority Probe

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 252–264. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`x-authority-xclock-smoke` now launches `xclock` against Sophia X
Authority and reaches mapped exposure plus Engine/Runtime committed authority
transactions. The compatibility work stayed probe-driven: xclock added bounded
font replies, pixmap/copy handling, subwindow mapping exposure, and core draw
transactions for the line, segment, and polygon opcodes it actually used.

The passing reduced evidence was `outcome=proof_window_killed`,
`requests=95`, `opcode_count=21`, `transactions=7`, `runtime_committed=7`,
`runtime_surfaces=7`, and `first_error=none`; the harness kills xclock after
the proof window because the client is intentionally long-running.

<!-- END IMPORTED BODY -->
