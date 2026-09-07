---
id: legacy-active-0253
date: 2026-07-17
recorded_date: 2026-07-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-17: GTK Input Stall Split From Scanout Throughput

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8344–8367. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The latest guarded X13 classic run presented the Zenity entry dialog but
accepted only five physical key presses before input stopped. The retained
15-second interval contained 984 X requests, including 252 outputless requests,
62 MIT-SHM PutImage requests, 31 CPU compositions, and 30 native submissions.
That showed both avoidable redraw work and socket-output lock contention, but
not a KMS deadlock: presentation continued while keyboard progress stopped.

Physical libinput collection now runs on a bounded worker instead of the
authority loop. Outputless X requests skip the shared output-stream lock,
software-only authority batches may coalesce their CPU composition while every
Engine transaction is still applied in order, and cursor-only movement produces
a composed native frame. During the pointer acceptance phase, physical Return
press and release are suppressed and reported instead of aborting the session.
Raw X request tracing and native lifecycle tracing are no longer enabled by the
normal GTK hardware runner; `SOPHIA_M5_GTK_DIAGNOSTIC=1` opts into both.

A bounded local Zenity entry proof then routed and flushed all fourteen
synthetic press/release events for `sophia` plus Return. GTK continued issuing
geometry, property, and SHM redraw requests but never exited or produced the
expected stdout before the semantic timeout. The throughput and lock fixes are
therefore retained, while GTK entry submission remains an explicit Milestone 5 compatibility gap.

<!-- END IMPORTED BODY -->
