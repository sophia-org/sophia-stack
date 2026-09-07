---
id: legacy-active-0614
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-09-04: fresh comparison candidate prepared after the Firefox canary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19504–19529. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Run `cp14-schema4-251d9acd` is prepared under `.artifacts/desktop-comparison/`
with 36 interactive rows and no optional soak. Its detached source checkout is
`.artifacts/desktop-comparison-checkouts/251d9acd`, pinned to signed, pushed
`251d9acdc631565a72513be9a73d17cef3ac30c5`. It has its own build outputs, so
ongoing work in the main checkout cannot change its candidate identity or
executables. Prior runs are untouched.

Hagia and Narthex come from the immutable `2823807e2ecd` canary artifact;
their binary hashes match that successful run. Preparation records all six
stack/policy/shell binary hashes, configuration/input hashes, host identity,
and the X11-core cursor digest. Kitty 0.48.2, Firefox 155, and niri 26.04 match
the expected versions. The detached Hagia profile initially inherited mode
0664 from the host umask; removing group-write permission made both Hagia and
Sophia config checks pass without changing any profile bytes. Niri validation
and the native profile-family check also pass.

`/tmp/c` now invokes this checkout's xtask and run with explicit pinned binary
paths; `/tmp/s` is an alias to the same helper. The existing post-capture niri
IPC exit watcher is retained. The next row is order 1, Sophia `kitty-60s`, with
the excluded physical cursor qualification. No graphical session or workload
has been started. Live attestation and tracefs preflight remain gate-owned:
the read-only sudo probe could not run without a password in this terminal,
so TTY3 will request it normally. Status verifies `completed=0 total=36`.

<!-- END IMPORTED BODY -->
