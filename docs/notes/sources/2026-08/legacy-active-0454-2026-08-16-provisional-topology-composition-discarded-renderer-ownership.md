---
id: legacy-active-0454
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "tooling", "architecture"]
---
# 2026-08-16: provisional topology composition discarded renderer ownership

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13654–13674. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first mixed mirror-plus-extended rerun after moving preparation into the
  session owner crossed the new quiescence barrier correctly. Its last ordinary
  frame retired, no KMS submit occurred, and candidate preparation then failed
  with `MissingSource(DmaBuf { handle: 8 })`. The supervised reference peer
  restarted and reproduced the same deterministic rejection until its restart
  budget was exhausted.
- Cause: ordinary retained composition resolves Engine's committed scene into a
  mixed set of CPU buffers and authority-owned renderer images. Provisional
  topology composition instead rebuilt CPU layers and called the CPU-only head
  lowerer. A committed DMA-BUF handle records logical source identity; it does
  not recreate the renderer-private image that already owns those pixels.
- Decision: candidate and rollback head plans use the same retained mixed source
  set as ordinary frames. The provisional viewport still produces its own
  display list, snapshot, native geometry, target generation, and damage, but
  lowering consumes the already-owned CPU or renderer-image realization. This
  remains read-only until the complete topology resource cohort is prepared and
  preserves the Engine/backend boundary: Engine names committed pixels, while
  the backend retains their renderer realization.

<!-- END IMPORTED BODY -->
