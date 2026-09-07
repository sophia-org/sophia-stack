---
id: legacy-archive-0008
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-07-10: xsetroot And xlogo Coverage Probes

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 179–193. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`x-authority-xsetroot-name-smoke` now launches `xsetroot -name
"Sophia Root"` and exits successfully with no X protocol error, proving root
property mutation through existing bounded property and GC paths. Its reduced
evidence was `outcome=client_exited_success`, `requests=7`, `opcode_count=6`,
`opcodes=18,20,43,55,60,98`, `transactions=0`, `runtime_committed=0`,
`runtime_surfaces=0`, and `first_error=none`.

`x-authority-xlogo-smoke` now launches `xlogo` and reaches
Engine/Runtime committed authority transactions without adding new X protocol
surface. Its reduced evidence was `outcome=proof_window_killed`, `requests=34`,
`opcode_count=11`, `opcodes=1,2,8,9,16,18,20,55,69,70,98`, `transactions=6`,
`runtime_committed=6`, `runtime_surfaces=6`, and `first_error=none`.

<!-- END IMPORTED BODY -->
