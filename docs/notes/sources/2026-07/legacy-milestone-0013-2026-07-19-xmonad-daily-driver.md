---
id: legacy-milestone-0013
date: 2026-07-19
recorded_date: 2026-07-19
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-07-19 xmonad Daily Driver

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 310–328.

<!-- BEGIN IMPORTED BODY -->

- [x] Added the normal supervised xmonad session with approved startup/action
  applications, logout, bridge recovery, and generic process-group ownership.
- [x] Closed Firefox compatibility gaps with focused wire regressions and an
  offline six-stage proof covering keyboard, pointer, resize, dialog,
  `CLIPBOARD`, `PRIMARY`, normal exit, and cleanup.
- [x] Passed three consecutive two-output mixed-application QEMU gates with
  xterm, GTK, Vulkan, Firefox, workspace movement, and bridge restart.
- [x] Passed the unattended 30-minute QEMU soak with 22 terminal, Firefox, and
  launcher cycles, 66 closes, zero unexpected protocol errors, no pending WM,
  action, or input work, and clean native/frontend/application teardown.

The retained `xmonad-m8-mix` and `xmonad-m8-soak` scenarios are the Milestone
8 regression contract. The accepted soak ran for 1,891,936 ms and completed
with 9,551 authority transactions and no native cleanup debt.

---

<!-- END IMPORTED BODY -->
