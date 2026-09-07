---
id: queue-08
date: 2026-09-06
kind: plan
tags: [plan, milestone]
---
# CP-15.1 — Native protocol-family lifecycle audit

This plan retains the scope, constraints, and task details from the roadmap
cutover. Task status and order live only in [todo.md](../../../todo.md)
and the [monthly completion history](../../../done.md). Follow the
[work-tracking contract](../../work-tracking.md).
Historical candidate identities in the details require revalidation before use.

[Parent scope](queue-01-critical-path.md).



## t022

Audit `sophia_wm_v1`, `sophia_shell_v1`, and `sophia_output_v1` against
`docs/sophia-policy-ipc.md`.


Required exit:

- align hello/welcome negotiation, effective bounds, capabilities, epochs,
  transaction identity, complete transfers, outcomes, recovery, and extension
  handling;
- document every intentional role-specific difference in its role contract;
  and
- remove or explicitly version accidental transport forks without weakening
  the frozen WM revision.
