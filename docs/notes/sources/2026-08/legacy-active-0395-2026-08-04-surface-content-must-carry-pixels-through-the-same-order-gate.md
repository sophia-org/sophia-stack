---
id: legacy-active-0395
date: 2026-08-04
recorded_date: 2026-08-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-04: surface content must carry pixels through the same order gate

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11990–12021. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The post-vkcube audit found that the existing surface fence delayed later
  `SurfaceTransaction` values but not their CPU patch payloads. Renderer
  updates remained envelope-scoped and were applied before fence admission, so
  a stable CPU handle could expose future pixels during an unrelated repaint.
  Software Present did not arm the fence, and a released batch rebased every
  group against the same committed generation.
- Engine now owns a bounded, protocol-neutral `SurfaceContentStream`. It tracks
  exact `SurfaceTransactionKey` owners per surface and retains opaque payloads;
  multi-surface work waits for every owner, later work cannot pass an earlier
  overlapping group, removals remain nonblocking, and shutdown discards bounded
  debt explicitly. The reducer contains no X11 or renderer types.
- A live authority group now carries its CPU buffer mutations beside its
  transactions. Production admission happens before those mutations reach the
  renderer. DMA and software Presents acquire exact content ownership, and
  retirement or rejection releases the FIFO backlog into the next ordinary
  production cycle. Released groups run before new authority work and rebase
  sequentially; their buffer handles remain residency roots while deferred.
- Admission quarantine retains the same grouped pixel payload and removes it
  from the projected batch until release. X authority remains the sole owner of
  Present, SHM, clear, core-drawing, copy, and clipping semantics; Engine sees
  only the reduced surface order. The authority regression verifies one stable
  handle and generations 1 through 4 across Present, SHM, clear, and core draw.
  The Rust stream regressions cover exact settlement, multi-owner FIFO,
  independent progress, removal, capacity, and shutdown.
- `SurfaceContentStream.tla` models the Present owner, three representative
  deferred operations, independent progress, retirement, visible generations,
  and fair drain. The pinned TLA+ Tools 1.7.4 gate explored all 28 distinct
  states and found no safety or liveness error; the three established models
  passed in the same reproducible run.

<!-- END IMPORTED BODY -->
