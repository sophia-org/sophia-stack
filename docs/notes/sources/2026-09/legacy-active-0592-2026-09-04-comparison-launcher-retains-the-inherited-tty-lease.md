---
id: legacy-active-0592
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "shell", "validation"]
---
# 2026-09-04: comparison launcher retains the inherited TTY lease

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18713–18739. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next signed candidate `7fea8dbb` failed at the newly attributable
`tty-admission` stage with `could not open the validated operator terminal`.
The invoking shell had already proved that standard input was tty3 and that tty3
was the active VT. After the attempt, `/dev/tty3` was owned by `root:tty` with
mode `0620`, and the invoking user was not a member of `tty`. The login shell
could continue using the descriptor inherited during login, but a new pathname
open was correctly denied.

Both earlier fixes reopened `/dev/tty3`: first inside the asynchronous child,
where the failure was invisible before the launcher log, and then in the
foreground parent, where the stage journal finally exposed it. Bash's
asynchronous `/dev/null` substitution requires preserving an existing
descriptor, not reacquiring the device.

The gate now duplicates its already-validated standard input with
`exec {operator_tty_fd}<&0`, checks that the duplicate is still a terminal,
and resolves `tty` through that descriptor to prove its identity did not
change. The asynchronous child receives the duplicate as standard input and
performs its existing identity check before launch. The gate neither changes
device ownership or permissions nor broadens user group or sudo policy.

Launcher-safety coverage requires inherited-descriptor duplication and rejects
the pathname-open spelling that caused the failure. The null-standard-input
regression and structured gate journal remain intact.

<!-- END IMPORTED BODY -->
