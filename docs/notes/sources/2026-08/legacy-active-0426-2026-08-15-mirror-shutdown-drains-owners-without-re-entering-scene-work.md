---
id: legacy-active-0426
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering"]
---
# 2026-08-15: mirror shutdown drains owners without re-entering scene work

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12888–12912. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Diagnostic mirror attempt `0006` on signed source `cabd6b18` rendered and
  retired both physical heads through logical frame 17, then submitted frame 18
  on both. Completion failed because its mirror drain called the full backend
  tick with an empty input while committed xterm surfaces still required layer
  templates. The resulting `InvalidSurface` was a shutdown-only scheduling
  error, not lost KMS ownership.
- Mirror retirement now collects and applies every physical callback, joins the
  logical generation, and retries every head cleanup before reporting an
  aggregated error. The drain path invokes only that ownership work; it cannot
  run Engine scene projection, renderer export, KMS submission, successor
  promotion, or the scheduler watchdog. Normal scheduling also applies accepted
  callbacks before its fallible Engine tick, closing the callback-loss window.
- A drain failure still attempts forced detach. Its typed error preserves the
  original drain error, the detach error or report, and the known abandoned-owner
  count. Completion clears renderer images only after detach is established.
- The same run exposed eighteen DP-2 `OutputMismatch` damage rejections. One
  logical 2560x1440 snapshot had been cloned into a 1920x1080 physical-head
  presentation state even though the pixels were projected. Every mirror queue
  path now prepares a destination-native snapshot before reserving the group
  generation, projecting surface, border, and cursor rectangles with the same
  fit transform as the pixels. Connector-qualified damage evidence must join a
  causal frame on both heads, and any `OutputMismatch` fails physical promotion.

<!-- END IMPORTED BODY -->
