---
id: legacy-milestone-0021
date: 2026-07-11
recorded_date: 2026-07-11
date_basis: first-heading-commit
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
date_commit: 99d764b54e7fff859490b9714e1cc13f050b98bb
committed_at: 2026-07-11T21:39:54-04:00
---
# Completed Visible Interactive X Terminal

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 485–499.
Date from the first addition of this heading in commit `99d764b54e7fff859490b9714e1cc13f050b98bb`
(2026-07-11T21:39:54-04:00); it does not date every event or later edit.

<!-- BEGIN IMPORTED BODY -->

- [x] Backed core X drawing with bounded XRGB8888 CPU buffers, including the
  fixed-font xterm path, and composed those pixels into renderer-owned frames.
- [x] Kept X Authority, backend ticks, native scanout ownership, and xterm under
  one persistent session owner with clean submit, page-flip retirement, and
  cleanup evidence.
- [x] Routed QMP virtio-keyboard input through libinput and Engine focus, then
  repeated the exact 14-event `sophia` plus Return proof with an operator on AMD
  TTY hardware. Both paths changed later xterm pixels without internal X event
  injection.
- [x] Selected xterm's core-key event target from authority-private event masks,
  emitted the required focus transition, and withheld readiness until the
  nonzero prompt checksum was page-flip-confirmed.

<!-- END IMPORTED BODY -->
