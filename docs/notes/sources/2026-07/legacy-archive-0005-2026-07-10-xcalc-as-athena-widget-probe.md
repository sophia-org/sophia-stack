---
id: legacy-archive-0005
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-10: xcalc as Athena Widget Probe

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 126–140. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`x-authority-xcalc-smoke` now launches `xcalc` against Sophia X
Authority and reaches Engine/Runtime committed authority transactions with no X
protocol error. The compatibility work stayed probe-driven: xcalc added bounded
`AllocNamedColor`, `UnmapWindow`, and padded one-character `PolyText8` handling.
Socket write-side disconnects from clients that close after proof are treated
as normal teardown rather than authority failures.

The passing reduced evidence was `outcome=proof_window_killed`,
`requests=219`, `opcode_count=23`,
`opcodes=1,2,8,9,10,16,18,20,43,45,47,49,50,53,54,55,60,61,72,74,85,94,98`,
`transactions=37`, `runtime_committed=37`, `runtime_surfaces=37`, and
`first_error=none`.

<!-- END IMPORTED BODY -->
