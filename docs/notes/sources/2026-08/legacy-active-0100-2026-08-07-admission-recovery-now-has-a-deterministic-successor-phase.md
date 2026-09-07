---
id: legacy-active-0100
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-08-07: Admission recovery now has a deterministic successor phase

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3298–3336. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The installed `1a7d67c3` session reproduced the short, black Firefox window on
Super+Space. Its fallback Present retired at `1280x1040`, but admission kept
that recovery extent because the standing `1276x1422` target was still unmet.
The same temporary extent then constrained every ordinary xmonad relayout back
to the fallback. A target Present could not become the bounded successor while
the fallback candidate was still awaiting retirement, so the only previously
working route was an unarmed-retirement timing race.

Recovery is now explicitly two phase. Exact fallback retirement makes the
surface managed, removes the temporary constraint while retaining its pixels,
preserves the standing target, and queues one normal relayout. The visual
tracker permits one fallback and one distinct logical-target successor per
surface, rejects repeated successors, and requires exact native-retirement
identity before the target changes committed layout state. Session completion
also fails closed on a remaining standing target, not only a recovery extent.
The `AdmissionRecovery` TLA+ model explores target observation before and after
fallback retirement and proves one relayout, constraint release before target
commit, exact target retirement, and eventual convergence.

The packaged xmonad order is restored to the user's established
`ThreeColMid`, `Tall`, `Mirror Tall`, `Full`, `Spiral` sequence. The promotion
page no longer assumes the focused Firefox surface must resize on the first
cycle: it accepts an outer position or size change, while the full M8 proof
retains its resize-specific checkpoint. The strict verifier correlates Firefox
from its launch transaction, requires one moved layout, every affected exact
retirement, three visible managed surfaces, a post-action Firefox Present, and
clean recovery. Deterministic Rust, proof-page, canary, verifier, configured
xmonad, and TLC gates cover the recovery change. Installed `7bd3e7db` proves
that recovery phase but exposes the separate move-feedback defect recorded
above; its focused Firefox run is not promotion evidence.

The local source audit also exposed the already-generated `sophia_wm_v1` Rust
wire table above the review threshold. It remains one generator-owned protocol
table, so the exact path now has a temporary cohesion-ledger entry; splitting
the schema generator is separate from this runtime recovery correction. The
new recovery tests were instead split at their actual admission/recovery seam.

<!-- END IMPORTED BODY -->
