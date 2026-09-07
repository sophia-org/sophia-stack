---
id: legacy-active-0419
date: 2026-08-09
recorded_date: 2026-08-09
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "validation"]
---
# 2026-08-09: Hagia restart proof exposed asynchronous keyboard loss

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12717–12756. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first opt-in Hagia policy-restart run reached the actual hardware path:
  two-output startup completed, Hagia saved a nonempty checkpoint, the
  supervisor replaced it at epoch 2, the replacement loaded and reconciled the
  checkpoint, fullscreen geometry survived, and later native page flips
  retired. The run did not pass. The exact text matcher first observed the
  eighth letter because the preceding seven physical keys arrived during the
  short Engine/frontend focus-acknowledgement gap after policy commits.
- `control_plane_only` had intentionally kept emergency, VT, and WM shortcuts
  responsive while dropping client input. Dropping was safe against
  misdelivery but created visible text loss under ordinary rapid interaction.
  Treating the mismatch as a proof typo would have hidden a production race.
- Engine now resolves the control plane once and holds only the unmatched
  client-bound keyboard sequence for the exact seat and generational focus
  target. Matching frontend acknowledgement releases that sequence in order.
  Target replacement, focus/seat or security change, topology transition,
  timeout, and capacity exhaustion discard it atomically. Original libinput
  timing sidecars survive the hold, so latency evidence starts at physical
  ingress rather than acknowledgement. Focused state-machine and live-router
  tests cover ordered release, retarget rejection, timeout, capacity, exact
  text matching, and no premature client delivery. A new physical run remains
  the acceptance proof.

- The replacement run completed all 34 exact text press/release events, their
  terminal pixel change, libinput-to-page-flip timing, bounded shutdown, and
  clean TTY restoration. It still was not a policy-gate pass: the final phrase
  was entered before the post-restart action sequence, so the evidence had no
  ordered physical action commits. It also revealed that startup itself
  reached checkpoint occurrence 4 and triggered the injected fault before the
  documented `Super+Y`, `Super+Right` boundary.
- The gate now faults on deterministic occurrence 6: four startup checkpoints
  followed by the fullscreen and active-output checkpoints. The final phrase
  is explicitly labeled as an immediate exit signal and must follow every
  post-restart action. Caller-specific verification reports the first missing
  pre- or post-restart phase and separately requires exact text, one restart,
  clean session/topology health, drained native ownership, and clean process
  teardown. Generic CPU-layer proof assumptions no longer mask the more
  specific Hagia result.

<!-- END IMPORTED BODY -->
