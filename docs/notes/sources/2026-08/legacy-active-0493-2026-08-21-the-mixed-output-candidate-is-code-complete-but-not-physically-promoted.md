---
id: legacy-active-0493
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-21: the mixed-output candidate is code-complete but not physically promoted

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15100–15131. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Sampling evidence now describes the pixels actually drawn. Engine classifies
  source and native-target extents independently on both axes, including a
  mixed-axis result. Lowering repeats that calculation only after retained-image
  realization, then the native renderer consumes the resulting class rather than
  deriving a competing one. The realized source extent also enters the damage
  snapshot, so filtering footprint and evidence agree with the draw.
- The proof chain is head-keyed. Output, opaque head, and scene generation travel
  through live lowering into schema-3 native sampling records. The mixed verifier
  now requires the exact renderer draw to occur between the same head's queue and
  submit records, rejects legacy/fallback evidence, and requires the schema-2 plan
  to report no mixed-axis draw. Mutation fixtures cover stale and unkeyed claims.
- Public chrome is one transaction with two projections. Blind policy and the
  reducer retain outer allocation; clearance is applied before layout
  reconciliation to derive content geometry and any client configure. Only the
  content projection is materialized, while the outer projection commits after
  acknowledgement and settlement. Schema-2 records expose those stages, including
  the case where policy omitted a configure but chrome generated one.
- The static post-commit topology audit closed four faults before spending another
  physical run: commitment forces a full repaint after its retirement baseline;
  stale parked hardware publications are dropped by authority epoch; overlapping
  policy candidates wait for `Stable`; and startup submission barriers are keyed
  and rebuilt by `RenderHeadId` after scanout replacement. Regressions cover stale
  publication, back-to-back effects, and same-count head reordering.
- `cargo fmt --all --check`, `git diff --check`, offline metadata, the sampling
  verifier fixtures, and `cargo test --offline -q --all-features` pass. This is not
  physical acceptance evidence. The next promotion action still requires a clean,
  signed candidate from the dedicated target TTY, visual confirmation of the
  mixed mirror-plus-extended run, and manual head loss and return. Per-head pacing
  remains ordered after that gate.

<!-- END IMPORTED BODY -->
