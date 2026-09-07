---
id: legacy-archive-0009
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-10: xprop as Root Property Probe

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 194–210. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`x-authority-xprop-root-smoke` now launches `xprop -root` against
Sophia X Authority and exits successfully with no X protocol error. The
compatibility work stayed property-introspection only: xprop added bounded
`ListProperties` decoding and replies for namespace-local window property atom
sets, without broad root metadata synthesis or extra drawing behavior.

The external-probe CLI path is now table-driven for real clients whose only
differences are binary, arguments, display range, namespace, and transaction
requirement. Bespoke synthetic/protocol smokes remain explicit because their
report shapes and exercised authority paths differ.

The passing reduced evidence was `outcome=client_exited_success`, `requests=10`,
`opcode_count=5`, `opcodes=16,20,21,55,98`, `transactions=0`,
`runtime_committed=0`, `runtime_surfaces=0`, and `first_error=none`.

<!-- END IMPORTED BODY -->
