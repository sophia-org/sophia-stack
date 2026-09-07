---
id: queue-20
date: 2026-09-06
kind: plan
tags: [plan, milestone]
---
# CP-14.2 — Same-hardware comparison

This plan retains the scope, constraints, and task details from the roadmap
cutover. Task status and order live only in [todo.md](../../../todo.md)
and the [monthly completion history](../../../done.md). Follow the
[work-tracking contract](../../work-tracking.md).
Historical candidate identities in the details require revalidation before use.

[Parent scope](queue-19-deferred.md).



## t053

Complete and verify the 36-row Kitty/Firefox/resize/launch-burst matrix
against Sophia, XLibre+xmonad, and niri only when the user selects a stable
candidate or a named performance investigation. No automatic restart follows
a development-session fix. The separate two-hour soak remains optional.


Run `cp14-schema4-251d9acd` retains nine sealed Kitty rows and a failed Firefox
row-10 partial. These remain exact-candidate historical evidence, not a complete
or passed comparison. A changed comparison candidate requires a fresh pinned
run when this work is resumed; old rows cannot be relabelled for newer binaries.

The [comparison contract](../../validation.md#deferred-same-hardware-comparison)
and all existing tooling remain intact: clean signed preparation, executable
and configuration hashes, raw visibility/resources/kernel-frame evidence,
post-teardown sealing, exact complete matrix verification, and diagnostic
`verdict=none`. The partial still blocks only its own run. Deferral does not
waive validation or promote the failed attempt, and does not block CP-14.3.
