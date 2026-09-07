---
id: legacy-active-0591
date: 2026-09-03
recorded_date: 2026-09-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-09-03: comparison pre-launch failure becomes attributable

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18682–18712. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The replacement run prepared from signed candidate `5cfbb314` repeated the
same status-1 wrapper failure before sealing row 1. It again created no attempt
and did not cross the launcher's old logging boundary. That falsifies the claim
that absence of the launcher log alone located the failure at asynchronous
standard input: candidate admission, TTY admission, tracefs admission, and
launcher admission all previously shared the same evidence-free interval.

The next adapter version tried to open the validated operator terminal in its
foreground parent, checked that descriptor with `-t`, and gave the asynchronous
Sophia launcher a duplicate. The child verified that it still resolved to the
admitted terminal before it invoked the established launcher. This made a
failure to acquire the terminal synchronous and explicit, but it still reopened
the device path.

The adapter also writes an owner-only
`~/.local/state/sophia/desktop-comparison/gate-last.log`. Structured stage
records cover TTY, candidate, tracefs, stack, and Sophia-launch admission; an
`ERR` trap retains the exact unexpected command, line, and status. Expected
failures retain their stage and detail, the tracefs probe reports its captured
status and output, and an early launcher logging boundary retains a rejected
launcher invocation. The journal contains controller state only, not
application identity or measured evidence, and remains outside immutable run
contents.

An integration regression invokes the adapter with null standard input and
requires both a direct terminal error and the structured TTY-admission record.
Focused launcher tests and shell parsing pass. No physical row has yet exercised
this hardened boundary, so the comparison remains at zero sealed rows.

<!-- END IMPORTED BODY -->
