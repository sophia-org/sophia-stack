---
id: legacy-active-0126
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-06: Producer overload retains one newest Present

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4092–4134. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The production scheduler could already retain one queued Present, but a
same-surface owner in `SurfaceContentStream` admitted the first generation and
deferred every successor before the scheduler could see overload. Retirement
released the entire FIFO, then the first released generation became active and
hid the rest again. The queue therefore preserved stale work instead of the
newest drawable frame.

Engine admission now gives a pure, immediate, one-surface DMA-BUF Present one
replaceable deferred slot. Replacement never crosses a layout fence,
multi-surface group, CPU update, removal, software Present, or later work for
the same surface. Backend policy still owns payload classification and routes
every superseded transaction through the ordinary rejected-Present lifecycle.
XLibre's Present completion path confirms `Skip` for work discarded before
display; yserver confirms independent, exact Complete/Idle ownership; niri's
output loop confirms that a second frame does not overtake one awaiting vblank.

The diskless two-output virgl gate drives a three-buffer DRI3 client at 5 ms
intervals beside a static CPU Xterm. Its client selects and drains Complete and
Idle events. Two consecutive runs sustained two five-second overload phases,
kept both the replaceable Engine slot and scheduler queue at depth one, allowed
one KMS submission in flight, and retained at most three sources and two
Present records. The latest run completed 357 displayed frames, skipped 925,
and routed all 1,282 Complete and 1,282 Idle events exactly once. It recorded
906 supersessions, 361 balanced renderer requests/completions, a 40 ms maximum
worker request, no worker stall, no route failure, and clean resource teardown.

The subscribed client exposed two transport bugs during promotion. Protocol
events and WM controls used separate writers contending on one unfair socket
mutex, so feedback could starve focus acknowledgement. A control-output
priority barrier now prevents ordinary replies, input, or protocol events from
overtaking a pending control write. The live broker also inherited its
64-record protocol queue from the unrelated key-input bound. Route capacities
are now independent; the 512-record protocol bound derives from the 256-work
authority queue and Present's two feedback phases. Deterministic regressions
cover both the socket-lock race and lifecycle capacity. The production verifier
also mutation-tests queue depth, KMS overlap, supersession accounting,
client-visible feedback, worker debt and latency, resource high-water marks,
output confinement, and cleanup. Cumulative 250 ms progress samples replace
per-event serial diagnostics so evidence collection does not create a renderer
stall.

<!-- END IMPORTED BODY -->
