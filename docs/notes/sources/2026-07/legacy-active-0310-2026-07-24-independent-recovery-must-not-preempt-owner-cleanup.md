---
id: legacy-active-0310
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-07-24: Independent Recovery Must Not Preempt Owner Cleanup

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9649–9665. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The independent input guard previously exited 250 milliseconds after detecting
the emergency chord. The TTY wrapper's `wait -n` then returned and cleanup
immediately sent `TERM` to the graphical process group. That could preempt the
live owner loop even when it had independently observed the same chord and was
draining routed input, native scanout, and Present state.

After a guard trigger, the wrapper now gives the live session a bounded
five-second window to finish its in-process emergency path. A schema-3 recovery
record distinguishes `graceful` completion from `fallback_term` and retains the
session exit status alongside KD and termios restoration. The physical
emergency verifier requires guard and owner observations, a status-zero
graceful exit, fully drained input, no native or Present debt, and exact TTY
restoration. Fixture mutations ensure that a TERM fallback cannot be promoted
as a successful emergency capture.

<!-- END IMPORTED BODY -->
