---
id: legacy-active-0260
date: 2026-07-17
recorded_date: 2026-07-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "security"]
---
# 2026-07-17: Authority Commits Once Before Per-Output Projection

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8502–8526. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The persistent live backend no longer creates one authority inbox per output or
replays the same X transaction through every output Engine. The production
coordinator exposes a bounded commit phase; the primary runtime commits each
batch once, then every output consumes the same immutable committed snapshot and
the same precomputed commit observations. The late-client generation bridge now
runs once at this boundary, before the single commit, rather than priming every
output ahead of replay. A two-output regression proves both output assemblies
end on generation 6 while the runtime records one committed transaction.

The full offline all-feature suite passes. The rebuilt X13-hosted QEMU image
passed the strict two-xterm profile in 7,104 ms with 114 of 114 authority
transactions applied, two CPU layers, 7 ms input presentation, 4 ms maximum
composition, 40 submissions, 38 retirements, and zero cleanup debt. An initial
run without the centralized late-discovery bridge correctly rejected the second
xterm generations as stale; retaining that fail-closed evidence drove the fix.
Confined GTK then passed 54 SHM transactions, exact text and pointer evidence,
normal exit, `first_error=none`, 108 submissions, 106 retirements, and clean
resource shutdown. Classic GTK passed the same application contract, and the
emergency profile flushed all five routed chord deliveries before clean shutdown
in 178 ms. Composition still uses the CLI scene table and runs before
this commit phase; the next slice must compose from the coordinator snapshot.


<!-- END IMPORTED BODY -->
