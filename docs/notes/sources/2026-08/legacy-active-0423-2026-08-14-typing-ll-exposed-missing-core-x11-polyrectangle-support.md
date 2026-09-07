---
id: legacy-active-0423
date: 2026-08-14
recorded_date: 2026-08-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-14: typing `ll` exposed missing core X11 PolyRectangle support

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12831–12852. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Diagnostic mirror attempt `0003` on signed source `5d40cbae` again bootstrapped
  both heads and joined repeated logical generations with correct `outputs=1/1`
  readiness. Typing `ll` in the proof xterm deterministically made the client
  issue core X11 opcode 67 (`PolyRectangle`). X Authority did not decode that
  request, returned `BadRequest`, and xterm treated the protocol error as fatal
  and exited with status 83. The mirror scheduler had submitted frame 10 on both
  heads; one callback had retired before the client failure.
- X Authority now decodes both byte orders of the 12+8n request, validates the
  drawable, graphics context, namespace confinement, and drawable/GC depth, and
  reports `BadDrawable`, wire `BadGC`, `BadMatch`, or `BadAccess` as appropriate.
  Window rendering uses clipped, non-overlapping outline bands with i32 extents,
  including degenerate rectangles and wide/XOR lines without double-toggled
  corners. Graphics contexts retain their creation depth independently of the
  lifetime of the drawable used to create them.
- A fatal proof client no longer returns directly past native cleanup. Frontend
  intake stops, the owner loop enters bounded completion, partially joined
  mirror callbacks drain, renderer-image and Present ownership shut down, and
  explicit cleanup evidence is emitted before the original client error is
  returned. Cleanup failures are aggregated without masking that original error.

<!-- END IMPORTED BODY -->
