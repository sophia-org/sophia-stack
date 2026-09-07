---
id: legacy-archive-0004
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-10: xterm as Terminal Setup Probe

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 104–125. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`x-authority-xterm-smoke` now launches `xterm` against Sophia X
Authority and runs through terminal setup/lifecycle requests with no X protocol
error. The compatibility work stayed probe-driven: xterm added bounded
`ConfigureWindow` decoding and namespace-checked dispatch, then accumulated the
same setup-only query/resource coverage used by the stricter render proof. This
smoke remains a no-transaction lifecycle regression; rendered terminal contents
are covered by `x-authority-xterm-render-smoke`.

The external-probe harness now passes `-display` before client-specific
arguments so X Toolkit clients that use `-e` still receive the intended test
display, and no-transaction failure messages include request/opcode counters.

The current reduced evidence is `outcome=client_exited_success`, `status=0`,
`requests=228`, `opcode_count=26`,
`opcodes=1,2,3,12,14,16,18,20,43,45,46,47,53,54,55,60,72,84,91,94,96,98,101,119,133,134`,
`transactions=0`, `runtime_committed=0`, `runtime_surfaces=0`, and
`first_error=none`. The zero-transaction result is accepted here because this
smoke's invariant is setup/lifecycle compatibility without a client-visible X
protocol error, not rendered terminal output.

<!-- END IMPORTED BODY -->
