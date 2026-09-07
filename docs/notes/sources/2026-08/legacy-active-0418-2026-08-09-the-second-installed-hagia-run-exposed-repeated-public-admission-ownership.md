---
id: legacy-active-0418
date: 2026-08-09
recorded_date: 2026-08-09
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "tooling", "architecture"]
---
# 2026-08-09: the second installed Hagia run exposed repeated public admission ownership

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12694–12716. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Installed attempt `0002` proved that the constraint correction itself could
  commit, but startup still failed closed at `not_committed`. In eight seconds
  the archive recorded 3,121 constraint reconciliations, 3,124 layout commits,
  and 3,123 checkpoint saves for the same surface. Super+Enter's ordered action
  committed inside that loop, but the newly queued terminal could not be
  admitted before the startup deadline. Exact TTY recovery again succeeded.
- The owner intentionally keeps a policy-managed surface in visual quarantine
  until its first presented buffer retires. API v7 consumed WM planning
  ownership through the committed candidate workspace assignment, while a
  public projection has no Engine-owned workspace effect. A committed public
  `Manage` therefore left the surface eligible for another `Manage`; those
  immediate no-op commits starved native retirement and repeated indefinitely.
- The exact committed public `Manage` source now consumes planning ownership.
  Its pixels remain independently fenced in `AwaitingRetirement`, so no
  presentation safety is weakened and relayouts cannot consume unrelated
  pending admissions. A focused regression proves the surface is no longer a
  Manage candidate while its visual admission is still pending. The live
  Hagia smoke additionally caps reconciliation, layout-commit, and checkpoint
  counts so a high-speed feedback loop cannot pass merely because the process
  exits cleanly. A replacement physical run remains the acceptance proof.

<!-- END IMPORTED BODY -->
