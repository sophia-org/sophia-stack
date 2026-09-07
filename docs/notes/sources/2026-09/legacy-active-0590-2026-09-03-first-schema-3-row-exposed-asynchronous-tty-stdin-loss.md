---
id: legacy-active-0590
date: 2026-09-03
recorded_date: 2026-09-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-09-03: first schema-3 row exposed asynchronous TTY stdin loss

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18663–18681. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical `cp14-schema3` row ended at the `justfile` line-98 recipe
wrapper. It sealed no row and produced no new Sophia session or attempt logs.
Pre-display candidate, checksum, and executable admission passed independently.

The gate had launched `start_sophia_tty3.sh` as an asynchronous subshell. In
non-interactive Bash, an asynchronous command without job control receives
standard input from `/dev/null` unless the script supplies an explicit
redirection. The established launcher could therefore fail its `-t 0` contract
before its logging boundary. That matched the observed absence of new evidence,
but did not uniquely identify the failing pre-launch stage.

The gate now captures the path returned by `tty`, admits only `/dev/tty3`, and
redirects the asynchronous launcher stdin from that same device. This keeps
terminal recovery and input-guard ownership on the already-validated local
TTY. A launcher-safety regression prevents the explicit stdin contract from
being removed.

<!-- END IMPORTED BODY -->
