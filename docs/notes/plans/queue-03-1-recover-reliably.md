---
id: queue-03
date: 2026-09-06
kind: plan
tags: [plan, milestone]
---
# 1. Recover reliably

This plan retains the scope, constraints, and task details from the roadmap
cutover. Task status and order live only in [todo.md](../../../todo.md)
and the [monthly completion history](../../../done.md). Follow the
[work-tracking contract](../../work-tracking.md).
Historical candidate identities in the details require revalidation before use.

[Parent scope](queue-02-cp-14-3-development-session-readiness-and-milestone-14-c.md).



Previously completed evidence: [Preserve session-wide native evidence across scanout replacement, retaining earlier failures and each owner's drain obligations.](../sources/2026-09/todo-cutover-completed.md#legacy-done-004).


Previously completed evidence: [Honor runtime deadlines while the seat is suspended, without requiring reacquisition to begin shutdown.](../sources/2026-09/todo-cutover-completed.md#legacy-done-005).


Previously completed evidence: [Add regressions for retirement before replacement, repeated resume, immediate shutdown after resume, rejected VT switches, topology replacement, suspension across the deadline, and failure retention.](../sources/2026-09/todo-cutover-completed.md#legacy-done-006).


Previously completed evidence: [Complete the physical suspended-deadline canary.](../sources/2026-09/todo-cutover-completed.md#legacy-done-007).


Previously completed evidence: [Complete the physical VT-return canary.](../sources/2026-09/todo-cutover-completed.md#legacy-done-008).

Stage 1 is complete. Next: establish the normal live session in stage 2.

Exit: recovery and bounded shutdown pass without contradictory counters,
abandoned accepted work, or loss of the fallback desktop. The
[row-10 diagnosis](../sources/2026-09/legacy-active-0616-2026-09-04-cp14-row-10-exposes-scanout-evidence-lifetime-and-suspended-deadline-gaps.md)
is the starting evidence; no new comparison run is required.
