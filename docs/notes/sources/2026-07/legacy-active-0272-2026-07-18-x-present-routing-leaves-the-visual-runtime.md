---
id: legacy-active-0272
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-18: X Present Routing Leaves The Visual Runtime

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8760–8777. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The production visual runtime no longer stores an `XServerFrontendProtocolRouter` or any X
Present completion, idle, fence, or disconnect proof counters. It emits reduced
`LivePresentFeedbackOutcome` values through an injected protocol-neutral sink. The separate
`XPresentSessionObserver` translates those records to X wire events and owns all session-proof
accounting; shutdown consumes the renderer disconnect report outside visual control. A direct
regression proves the runtime sink receives the paired reduced outcome unchanged.

The full offline all-feature suite passes. The rebuilt X13 QEMU image passed strict two-xterm
in 6,988 ms with 117 of 117 authority transactions, 8 ms input presentation, 40 submissions,
38 retirements, and zero cleanup debt. Classic GTK accepted exact physical text and pointer
selection, exited normally with `first_error=none`, and retired both outputs cleanly. The
remaining structural extraction is X authority-batch and resource translation plus session
supervision around the protocol-neutral runtime, followed by deletion of legacy committed-
snapshot entry points then shared with the Wayland maintenance path.


<!-- END IMPORTED BODY -->
