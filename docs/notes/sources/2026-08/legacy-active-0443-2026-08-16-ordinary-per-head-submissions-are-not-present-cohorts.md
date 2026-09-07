---
id: legacy-active-0443
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-16: ordinary per-head submissions are not Present cohorts

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13286–13304. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first signed unequal-head physical run after semantic per-head composition
  rendered, prepared, submitted, and retired native-size frames on both heads.
  The session then failed after the next ordinary `HeadComposition` submission
  because the native completion path unconditionally advanced the DMA-BUF
  Present scheduler and found no in-flight Present cohort. Atomic scanout and
  renderer ownership were not the failing boundary.
- Native submission now classifies the retained content against the
  output-scoped Present frame and transaction before changing Present state.
  A matching `MixedPresent` advances its cohort, an ordinary compositor frame
  remains independent, and mismatched ownership fails closed. The scheduler
  exposes its reserved output frame for this ownership check without treating
  that reservation as submission or presentation evidence.
- Runtime-fatal sessions now emit the final clean outer-cleanup record after all
  client workers, namespace authority, and Xauthority state have been released,
  before returning the original error. This preserves the error while allowing
  the failed physical run to enter the diagnostic archive.

<!-- END IMPORTED BODY -->
