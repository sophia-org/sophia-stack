---
id: legacy-active-0173
date: 2026-07-27
recorded_date: 2026-07-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-07-27: Admission Release Preserves Atomic Transaction Groups

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5941–5969. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The latest physical vkcube trace exposed a second identity defect after
per-Present scene ownership was corrected. The admission quarantine retained
vkcube transaction 858, then appended it to the next ordinary frontend batch
whose envelope transaction was 367. Engine correctly rejected the manufactured
two-surface batch with `expected_transaction=367 actual_transaction=858`.
Transport, Kitty, and KMS remained alive, but the admitted vkcube surface never
crossed into committed visual state.

The envelope and the atomic unit are now explicit separate data shapes.
`LiveProductionAuthorityBatch` owns envelope-scoped DMA-BUF and fence lifetime
facts plus ordered `LiveProductionAuthorityGroup` records. Each group owns one
transaction ID and validates every surface transaction and Present submission
against it before scheduler or Engine intake. The pre-admission path retains
the same complete groups in a fixed 256-entry FIFO, reprojects and rebases them
only at accepted admission, and releases them beside—never inside—the current
frontend group. Mixed identity and capacity exhaustion fail the session closed.

Offline regressions reproduce transactions 367 and 858, validate both groups,
and commit them independently through the production coordinator. A routed
deferred-map vkcube smoke additionally requires real map intent, generic
`AdmitSurface` delivery, DRI3 import, and two exact Present Complete/Idle
round trips. The restricted sandbox has no `/dev/dri`; an unrestricted local
attempt reached intent and admission, but this environment's vkcube selected
llvmpipe and emitted no DRI3/Present handoff. The command therefore remains a
hardware-Vulkan preflight rather than retained proof. Visible vkcube pixels and
native KMS retirement remain the short physical roadmap gate.

<!-- END IMPORTED BODY -->
