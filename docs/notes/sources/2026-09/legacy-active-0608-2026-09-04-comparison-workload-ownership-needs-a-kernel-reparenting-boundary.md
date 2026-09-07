---
id: legacy-active-0608
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation", "architecture"]
---
# 2026-09-04: comparison workload ownership needs a kernel reparenting boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19257–19289. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Fresh run `cp14-schema4-2b50438c` proved the location-independent launcher and
reached the complete physical row. Cursor qualification passed all four targets
with 955 motion events. The row retained an empty baseline, 60/60 focused
visibility and resource samples, a stable three-process Kitty population, and
3,599 contiguous single-delivery kernel frames over 60.021 seconds. Its X11
protocol-error tally was clean. The staged row again remained partial because
strict session quiescence timed out with only `frontend_drained=false`.

This run falsified exact periodic PID retention as a complete ownership model.
`workload.log` exists only after process-group and every retained PID/start
identity have disappeared, and the Kitty surface withdrew immediately after
that successful termination. X client 7 nevertheless retained a descriptor
until forced frontend cancellation 31 seconds later. The remaining holder was
therefore outside both the original group and the ancestry visible in the
periodic samples, whether it detached before observation or appeared during
toolkit shutdown. Extending the PID list or the session timeout would preserve
the same race rather than own it.

The capture process now arms Linux child-subreaper ownership before it launches
the workload. A descendant that reparents out of the toolkit tree is adopted by
the capture owner instead of init. Adopted roots and their descendants join the
workload and aggregate resource populations using the sampler's existing one
`/proc` pass; no second sampling traversal or persistent process identity is
added. Teardown still gives the original private groups and retained identities
their bounded path, then repeatedly kills and reaps every newly adopted child
until the same two-second bound is empty. Pre-existing controller children,
including the trace owner, are excluded by exact PID/start identity. An isolated
regression makes a shell orphan a 30-second child, proves adoption, and requires
bounded termination and reaping. The `2b50438c` partial remains diagnostic; a
fresh signed physical row must prove frontend drain and clean finalization.

<!-- END IMPORTED BODY -->
