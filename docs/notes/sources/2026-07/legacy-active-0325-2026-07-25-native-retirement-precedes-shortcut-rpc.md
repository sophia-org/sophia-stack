---
id: legacy-active-0325
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "shell"]
---
# 2026-07-25: Native Retirement Precedes Shortcut RPC

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10129–10144. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The physical owner previously routed input before servicing native retirement.
A global shortcut could therefore enter the external WM transport's synchronous
request path, whose configured response timeout is 500 ms, while an accepted
KMS callback waited in the native event queue. The same wait could accumulate
physical input in the acquisition queue.

Native service now precedes shortcut routing. Completion evidence separately
records maximum child-reap, input-routing, and WM-request durations, and the
four-Kitty verifier caps each at 100 ms. This is an ordering correction, not a
claim that synchronous WM transport is suitable long term. If the next
physical evidence attributes the remaining input dwell to WM request time, WM
actions move to a bounded typed worker with explicit response correlation and
stale-response rejection.

<!-- END IMPORTED BODY -->
