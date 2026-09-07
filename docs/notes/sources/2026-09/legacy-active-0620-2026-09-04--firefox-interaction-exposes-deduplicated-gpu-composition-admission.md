---
id: legacy-active-0620
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-09-04 — Firefox interaction exposes deduplicated GPU composition admission

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19927–19974. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first CP-14.3 physical attempt did not start Sophia: the prepared core profile
retained group-write permission (`UnsafeMode`). Offline checking then caught
Hagia's newer `scratchpad-size` default, unsupported by this Sophia profile parser.
The bundle now uses mode 0600 profiles and an explicit, compatible recovery
profile. Both complete launcher argument vectors pass `--validate-session-args`
before takeover. The failed setup attempt is retained separately.

The retry reached native rendering and Firefox, then failed during a tab
interaction before any VT transition. The operator reports being in Firefox and
possibly clicking a new or existing tab. The runtime error was
`production CPU cycle failed in phase KmsSubmit: MissingSource(DmaBuf { handle: 7 })`.
Only startup native epoch 1 opened; the suspended-deadline canary never started.
Native cleanup drained both pending heads with zero abandoned scanouts and zero
cleanup errors. Frontend/application cleanup was clean. The TTY handoff restored
the manager using its safe baseline and reported usable manager input; this is
separate from successful canary acceptance.

Evidence is preserved under
`.artifacts/diagnostics/cp14-3-recovery-20260905T002819Z/attempts/02-mixed-composition-crash/`,
including logs, binary/profile manifest, source identity, checksums and the
operator's observation. The frozen failed executable remains unchanged.

Source inspection and a failing deterministic regression expose a concrete
routing defect consistent with the failure: a changed presentation order used
whether retained frame queueing produced a *new* frame to decide whether GPU
content should be preserved. Retained queueing legitimately produces none when
an identical newest frame is already pending or displayed. GPU visibility has
already been evaluated against the new presentation order, so the extra
order-change condition incorrectly sends those DMA-BUF sources to CPU-only
head lowering. The fix preserves a visible GPU projection regardless of whether
queueing was deduplicated. Removing the last GPU surface still admits CPU
composition. The exact physical tab gesture has not been replayed deterministically;
a physical retry remains necessary to establish that this fixes the observed run.

`RetainedCompositionAdmission` passes 22 states; its old-rule negative control
fails `CpuHasOnlyCpuSources` on an already-owned GPU frame after an order change.
The regression failed before the fix, and all 24 focused visual-runtime tests
pass afterward. The initial full gate exposed ambient configuration in session
unit tests: the operator's Hagia profile now contains unsupported `view-name`
settings. Verification is rerun with an isolated `XDG_CONFIG_HOME`; the personal
profile is left unchanged. The isolated full gate passed 2,350 test executions,
archive/reader checks, and hardware buffer-age equivalence; the release build
and both offline canary argument checks passed. The new frozen bundle is
`.artifacts/diagnostics/cp14-3-mixed-source-20260905T005021Z/`; `/tmp/c` points to it.
No physical acceptance checkbox is closed by these deterministic results.

<!-- END IMPORTED BODY -->
