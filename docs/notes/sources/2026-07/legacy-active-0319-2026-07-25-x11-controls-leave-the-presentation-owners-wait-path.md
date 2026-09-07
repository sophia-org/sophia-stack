---
id: legacy-active-0319
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-25: X11 Controls Leave The Presentation Owner's Wait Path

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9968–9995. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The delayed native submission during the four-Kitty workload was an ownership
bug rather than a renderer throughput limit. The live owner sent X11
configure, rollback, focus, clear-focus, and close requests and then waited as
long as 500 ms for each acknowledgement. During that wait it could not poll
DRM retirement or service input.

Those requests now enter a fixed-capacity session-control ledger. Configure
and close controls may be in flight concurrently; focus and clear-focus are
serialized globally. Acknowledgements correlate on client, command kind,
transaction, and surface. Channel pressure leaves work queued for a later
owner tick, while rejection, timeout, disconnect, duplicate identity, and
unexpected acknowledgement fail closed.

Engine focus remains authoritative. Client focus becomes applied only after
the matching X authority acknowledgement. While Engine and frontend focus
differ, physical routing retains cursor motion, emergency exit, VT switching,
and WM shortcuts but suppresses client keyboard, button, and axis delivery.
The owner services controls at both tick boundaries and limits its authority
wait to one millisecond while controls remain pending.

The CLI emits a separate `sophia_live_session_control` record with balanced
lifecycle counts, peak depth, queue dwell, and acknowledgement latency. The
four-Kitty verifier requires a drained failure-free ledger and bounds both
latencies to 100 ms. Synchronous initial-modeset evidence also moved from the
backend library to the CLI evidence boundary, removing direct library output.

<!-- END IMPORTED BODY -->
