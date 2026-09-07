---
id: legacy-active-0057
date: 2026-08-14
recorded_date: 2026-08-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-14: failed mirror proofs are diagnostics, not promotion evidence

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1742–1757. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The physical mirror runner now preserves a strict evidence boundary. Only a
  successful session plus explicit pixel confirmation reaches the promotion
  archive; runtime and visual-confirmation failures go to a separate diagnostic
  archive and retain their original exit semantics.
- A failed record pins the same source, binary, and profile identity as the
  promotion proof, then adds the failing stage, exit, derived signal, and a
  bounded kernel-log delta. Kernel access is opportunistic and non-interactive:
  inability to read it is an explicit fact rather than a reason to prompt or to
  discard the rest of the failure evidence.
- Kernel deltas retain the newest 256 lines by default and record whether the
  snapshots were continuous, reset, or truncated. The archive verifier binds
  that metadata to the retained delta and rejects promotion markers, so a useful
  crash artifact cannot accidentally become proof that mirroring worked.

<!-- END IMPORTED BODY -->
