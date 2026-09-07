---
id: legacy-active-0278
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-18: Backend Owns Present Scheduling State

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8843–8857. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Backend-live now owns `LiveProductionPresentScheduler`: queued and submitted Present state,
first-frame acquire delay, fence polling, bounded timeout rejection, controlled-rejection
proof policy, diagnostic triggering, and acquire/rejection counters. The CLI visual wrapper
asks for a reduced gate decision and supplies native scanout availability; it no longer owns
the scheduling tables or timing state. A backend regression proves delayed acquire admission
and one-shot controlled rejection with a registered DMA-BUF presentation.

The full offline all-feature suite passes. The rebuilt X13 guarded native mixed diagnostic
crossed the new scheduler, exported one CPU plus one DMA-BUF layer, and ended with zero live
sources, fences, or transactions. The remaining central Milestone 6 extraction is the
concrete per-output runtime owner and the legacy committed-snapshot APIs shared with Wayland.


<!-- END IMPORTED BODY -->
