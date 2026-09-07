---
id: legacy-active-0609
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-09-04: admission recovery must follow retained visual evidence

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19290–19324. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Run `cp14-schema4-124ad6c1`, pinned to signed candidate `124ad6c1`, sealed all
nine Kitty rows before Sophia Firefox row 10 failed its 30-second visibility
bound. The partial retained an empty visibility baseline and no owned top-level.
Firefox had reached the loopback readiness endpoint and produced its disposable
profile, so this was not a launcher or page-readiness failure.

The Engine first primed surface `10485776` at 200x210 and repeatedly reconciled
that exact recovery constraint while the surface remained unassigned. Firefox
then supplied complete 1290x1050 PresentedBuffer candidates, including
transactions 472 and 478, but no matching layout committed. The evidence reducer
correctly selected the stronger Presents; the independently stored recovery
extent was first-write-wins, however, and remained 200x210. Exact candidate
selection consequently rejected 1290x1050 while policy continued requesting
200x210. The two individually safe checks formed a no-progress loop.

Admission recovery now reduces four passive facts together: presentation phase,
selected safe observation, exact quarantine retention, and current recovery
extent. Candidate-less or unavailable startup geometry cannot prime admission.
During `PolicyPending`, `ControlPending`, and `AwaitingPixels`, a stronger or
newer retained candidate replaces the temporary extent; `AwaitingRetirement`
freezes it under the already-armed exact frame. An unusable stale extent is
cleared once instead of driving repeated policy cycles. Timeout recovery applies
the same exact-retention rule. The path adds no wire state, application identity,
or hot-path allocation, and emits a single `admission_extent_rebased` record only
when the extent changes.

Deterministic native-session coverage reproduces a 200x210 backing candidate
followed by a 1290x1050 Present and requires the recovery extent and relayout to
advance. External reducer tests cover missing quarantine identity, stale clearing,
idempotence, and the retirement/managed freeze. The existing run and row-10
partial remain immutable diagnostic evidence; a new signed candidate must pass a
short physical Firefox visibility canary before a fresh 36-row run begins.

<!-- END IMPORTED BODY -->
