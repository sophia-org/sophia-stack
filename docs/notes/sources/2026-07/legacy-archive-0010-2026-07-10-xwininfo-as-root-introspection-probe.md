---
id: legacy-archive-0010
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-10: xwininfo as Root Introspection Probe

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 211–222. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`x-authority-xwininfo-root-smoke` now launches `xwininfo -root`
against Sophia X Authority and exits successfully with no X protocol error. The
compatibility work stayed introspection-only: xwininfo added bounded
`GetWindowAttributes`, `GetGeometry`, `QueryTree`, and `TranslateCoordinates`
replies for root/window facts without creating Engine transactions.

The passing reduced evidence was `outcome=client_exited_success`, `requests=9`,
`opcode_count=6`, `opcodes=3,14,15,16,20,40`, `transactions=0`,
`runtime_committed=0`, `runtime_surfaces=0`, and `first_error=none`.

<!-- END IMPORTED BODY -->
