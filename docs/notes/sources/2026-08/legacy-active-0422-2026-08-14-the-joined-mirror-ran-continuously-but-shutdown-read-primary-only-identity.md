---
id: legacy-active-0422
date: 2026-08-14
recorded_date: 2026-08-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering"]
---
# 2026-08-14: the joined mirror ran continuously but shutdown read primary-only identity

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12806–12830. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Diagnostic attempt `0002` on signed source `e3435eb8` completed joined mirror
  generations 2 through 11 on both DP-1 and DP-2. Direct bootstrap, persistent
  workers, asymmetric callback order, repeated CPU updates, and logical damage
  publication all ran without duplicate transitions, watchdog stalls, AMDGPU
  rejection, or process abort. The run failed during bounded shutdown rather
  than visual service.
- On frame 12 the primary head submitted and flipped before the sibling submitted.
  Its physical callback correctly moved the primary's content from submitted to
  presented ownership. The sibling submission then completed the logical submit
  join, but the visual runtime asked the now-empty primary slot for the logical
  frame identity and rejected the drain with `native submit did not retain its
  frame identity`. No frame identity or buffer owner was actually lost.
- The mirror lifecycle now exposes the logically submitted generation while all
  heads have submitted and at least one callback remains. Runtime submission and
  drain observation use that group identity; per-head content remains available
  only as physical ownership evidence. A regression covers primary-flips-before-
  sibling-submit at shutdown.
- The same run revealed that startup readiness printed two physical heads as
  `outputs=2/2`, contradicting the one logical output required by the verifier.
  Startup now AND-joins readiness by `OutputId`, reports `1/1`, and emits exactly
  one logical synchronous-modeset record only after every mirror head has proof.
  Per-head direct-bootstrap and worker-readiness evidence remains unchanged.

<!-- END IMPORTED BODY -->
