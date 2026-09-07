---
id: legacy-active-0236
date: 2026-07-14
recorded_date: 2026-07-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "rendering"]
---
# 2026-07-14: Probe-Backed GTK Startup And Real SHM Pixels

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7912–7927. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The native frontend now advertises the measured XKB and XI2 startup subset,
including XI version/device discovery, client pointer/focus queries, event
selection, and optional device-property reads. Zenity consequently advances
through normal window creation and software drawing with no X protocol error.

That probe exposed the blank-block cause: `ShmPutImage` validated the segment
but discarded its offset and payload, then materialized a zero-filled damage
buffer. A narrow SysV SHM adapter now validates segment size with `IPC_STAT`,
attaches read-only, copies only a bounded image range, and detaches immediately.
The generic pixel-proof policy now records one 310-by-233 committed surface,
288,920 nonzero bytes, and `first_error=none`. This is software-pixel evidence,
not completion of interactive GTK input, the full XI2/grab contract, or
Milestone 3 hardware promotion.

<!-- END IMPORTED BODY -->
