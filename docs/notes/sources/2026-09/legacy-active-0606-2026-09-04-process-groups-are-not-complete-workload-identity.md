---
id: legacy-active-0606
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-09-04: process groups are not complete workload identity

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19210–19236. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Fresh owner-only run `cp14-schema4-69520f50`, bound to signed candidate
`69520f50`, proved the qualification correction: all four targets passed with
728 motion events, the instruction draws produced no X refusal, and the final
session protocol-error tally was clean. Its Sophia row again retained an empty
baseline, one focused and visible DP-1 workload for all 60 samples, 60 resource
samples, a complete workload record, and 3,599 contiguous single-delivery
kernel frames over 60.021 seconds. The application surface withdrew after
capture. Process-group cleanup reduced the quiescence survivors from two to
one, but the remaining accepted client kept frontend drain false through the
two-second deadline. The staged row correctly remained partial.

Every resource sample consistently observed three workload processes through
the root PID's descendant tree. The sampler already held each process's PID and
start time long enough to aggregate it, then discarded that identity. A private
process group is necessary but not sufficient when a toolkit helper changes
groups or sessions after launch. The conformance owner now retains the exact
PID/start pairs discovered by the existing one-pass sampler for the duration of
the attempt. Teardown sends TERM and then KILL both to the original groups and
to every retained identity that still has the same `/proc` start time, waits a
bounded two seconds for those identities to disappear, and reports only a
survivor count on failure. Application identity remains transient inside the
trusted conformance owner and is not written to comparison evidence. The
`69520f50` partial remains immutable; one fresh signed physical row must prove
that strict session quiescence now completes.

<!-- END IMPORTED BODY -->
