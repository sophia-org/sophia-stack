---
id: queue-05
date: 2026-09-06
kind: plan
tags: [plan, milestone]
---
# 3. Make failures diagnosable

This plan retains the scope, constraints, and task details from the roadmap
cutover. Task status and order live only in [todo.md](../../../todo.md)
and the [monthly completion history](../../../done.md). Follow the
[work-tracking contract](../../work-tracking.md).
Historical candidate identities in the details require revalidation before use.

[Parent scope](queue-02-cp-14-3-development-session-readiness-and-milestone-14-c.md).



## t015

Reuse existing telemetry for identifiable per-session logs and bounded
resource observations; preserve diagnostics after abnormal exit.


## t016

Provide a simple incident-time marker and document how to find the
matching build, configuration, session, and surrounding events. Keep expensive
tracing and pixel inspection opt-in; retain metadata-disclosure boundaries.


Exit: a reported problem can be investigated without reproducing it merely to
recover an overwritten log. Extend existing session/tooling owners rather than
building a separate monitoring platform.

## Implementation and acceptance

The implementation and deterministic evidence are recorded in the
[daily diagnostics investigation](../investigations/e84g9ivq-durable-daily-session-diagnostics-and-incident-markers.md).
The [operator contract](../../operations.md#mark-and-investigate-a-problem)
owns command syntax, limits, privacy, and the distinction from proof archives.

Both tasks retain one physical exit: in a replacement installed session, mark
an event from an independent TTY, log out, log in again, and inspect/preserve
the earlier session by ID. The current session must remain usable while marking.
This is a normal-use canary, not a resumption of the comparison matrix. A passed
deterministic test does not close this physical exit or earlier milestone gates.
