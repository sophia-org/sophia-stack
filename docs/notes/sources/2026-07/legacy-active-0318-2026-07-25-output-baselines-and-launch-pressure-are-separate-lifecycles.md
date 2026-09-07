---
id: legacy-active-0318
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering"]
---
# 2026-07-25: Output Baselines And Launch Pressure Are Separate Lifecycles

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9941–9967. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next rapid Super-Enter run did not fail in Engine, KMS submission, or X11
protocol handling. Initial modeset had already displayed the synthetic marker
on the secondary output, but modeset itself produced no page-flip callback.
The new unchanged-CPU-frame reduction saw the same displayed checksum and
suppressed the first event-bearing flip. Output 2 therefore remained at zero
callbacks, startup never reached its all-output proof, and the eight-second
deadline ended the session while several action-launched Kitty clients were
entering resize transactions.

An unchanged displayed frame is now suppressible only after that output has
observed a callback. Before then, the reducer emits a baseline-required outcome
and queues exactly one nonblocking flip; matching pending and submitted frames
remain deduplicated. Startup readiness is a monotonic passive record pinned to
the startup surface rather than whichever later surface owns focus.

Application launch pressure is also bounded independently from visual
authority. The CLI session supervisor retains a sixteen-entry FIFO across
active and queued action applications, waits for one opaque surface admission,
matching pixel retirement, and a settled layout pipeline before spawning the
next, and treats capacity, spawn, exit, or admission timeout as an application
outcome rather than a fatal session error. Global scanout quiescence is not an
admission condition: continuously presenting clients may supersede frames
without invalidating the fact that the new surface was displayed. Logout
cancels pending work. Engine and the blind WM remain application-agnostic.

<!-- END IMPORTED BODY -->
