---
id: legacy-active-0073
date: 2026-08-10
recorded_date: 2026-08-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-10: Desktop output activation has an immutable pre-I/O plan

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2375–2402. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Native startup already projected the atomic owner's capabilities and
  reconciled the unified output candidate before launching clients, but the
  reconciled connector states had no explicit handoff shape for a future
  backend executor or rollback path.
- `sophia-config` now centrally validates a complete output reconciliation
  against its admitted topology, including exact connector coverage, modes,
  scale, transform, VRR, focus, overlap, and at least one enabled output. The
  reconciler calls this same validator, so downstream boundaries do not copy
  those invariants.
- The CLI coordinator now prepares one immutable plan in stable Engine output
  order. Each target carries its `OutputId`, exact requested state, and exact
  rollback state plus the shared generation/digest; no connector, CRTC,
  property, framebuffer, or file-descriptor handle crosses the backend
  boundary. A second capability snapshot with changed mode or VRR facts is
  rejected as drift. Startup logs `prepared_not_applied` and performs no KMS
  test or mutation. Atomic request construction and effect execution remain
  the next tranche.
- A pure coordinator now supplies that typed completion contract without
  connecting hardware I/O. Test, apply, and rollback effects carry the same
  immutable plan and exact generation/digest key. Only a completion for the
  expected phase may advance the attempt; late, duplicate, and out-of-order
  completions are inert. Test failure discards the candidate immediately,
  apply failure requires rollback, and rollback failure is a terminal outcome
  retaining both failure causes. The executor and native request construction
  remain deliberately deferred.

<!-- END IMPORTED BODY -->
