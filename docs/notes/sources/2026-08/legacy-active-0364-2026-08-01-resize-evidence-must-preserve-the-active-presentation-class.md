---
id: legacy-active-0364
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-08-01: resize evidence must preserve the active presentation class

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11177–11200. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next physical run proved the exact 1280-by-1040 Firefox admission frame
  now reaches native retirement and releases its temporary recovery extent.
  Sophia then delivered the standing 1276-by-1422 configure and committed the
  layout without arming a matching visual candidate.
- The matching transaction was a passive CPU backing snapshot. Firefox's
  visible DMA-BUF producer continued submitting 1280-by-1040 frames, which the
  1276-by-1422 layer clipped to 1276-by-1040. The logical layout therefore had
  roughly 382 uncovered rows, matching the physical black lower region and
  clipped browser content.
- Complete visual evidence now establishes a protocol-neutral, monotonic
  requirement for the surface lifetime. Once `PresentedBuffer` is observed,
  `BackingSnapshot` remains available as safe recovery state but cannot stage
  or commit a later resize. An exact presented transaction must follow the
  existing visual-arm and native-retirement path. CPU-only surfaces retain
  synchronous backing resize, and explicit software Present remains valid
  presented evidence so storage-class changes do not deadlock.
- Engine and all-feature live-session regressions reproduce the physical
  1280-by-1040 to 1276-by-1422 sequence, preserve the standing target across
  the rejected backing snapshot, require matching DMA retirement, retain the
  CPU-only path, cover explicit software Present, and clear the requirement on
  surface removal. A fresh physical Firefox run remains the acceptance proof.

<!-- END IMPORTED BODY -->
