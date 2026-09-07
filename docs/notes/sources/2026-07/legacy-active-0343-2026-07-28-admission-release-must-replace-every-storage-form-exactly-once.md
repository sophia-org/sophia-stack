---
id: legacy-active-0343
date: 2026-07-28
recorded_date: 2026-07-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-28: admission release must replace every storage form exactly once

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10734–10755. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first physical run with software feedback failed immediately after
  committing the initial CPU snapshot with `DuplicatePresentation`. The same
  transaction reached production twice: admission projection removed the
  quarantined surface transaction and DMA-BUF Present from the original
  observation, but omitted the equivalent software-Present record. The
  same-iteration admission release then appended its retained copy.
- Projection now applies one quarantine predicate to CPU transactions,
  DMA-BUF Presents, and software Presents. The released admission group is the
  sole owner of the reprojected transaction and presentation lifetime.
- Admission and production validation also reject duplicate software Presents
  for one transaction/surface before renderer resource registration. Focused
  regressions reproduce same-iteration replacement and the defensive failure
  boundary.
- The corrected physical standalone run passed its exact verifier. The
  500-by-500 llvmpipe cube animated through 487 software-Present transactions
  in 17,755 ms; all 487 produced native retirements, Complete, Idle, and
  idle-fence triggers. Native submission and retirement failures remained
  zero, all presentation resources drained, X protocol errors remained zero,
  and the session completed normal cleanup.

<!-- END IMPORTED BODY -->
