---
id: legacy-archive-0006
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-10: xrandr as Output Introspection Probe

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 141–158. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`x-authority-xrandr-query-smoke` now launches `xrandr --query`
against Sophia X Authority and exits successfully with no X protocol error.
The compatibility work stayed output-introspection only: xrandr added a minimal
`RANDR` extension advertisement, bounded `RRGetScreenSizeRange`, and empty
`RRGetScreenResources` replies. It does not synthesize connectors, CRTCs,
modes, providers, leases, or monitor objects yet.

The external-probe trace now carries bounded parse-error detail with the first
request bytes. That exposed the real RandR minor opcodes (`6` and `8`) after
the client-facing error string incorrectly labelled the failed request as
`RRQueryVersion`.

The passing reduced evidence was `outcome=client_exited_success`, `requests=10`,
`opcode_count=4`, `opcodes=20,55,98,132`, `transactions=0`,
`runtime_committed=0`, `runtime_surfaces=0`, and `first_error=none`.

<!-- END IMPORTED BODY -->
