---
id: legacy-active-0288
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-07-18: GTK Client Exit Hang And Post-Proof Completion Watchdog

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9006–9032. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first X13 run with routed pointer buttons accepted exact physical `sophia` text, routed
the OK click, and presented the surface-removal frame, then held a blank screen until the
emergency chord restored the TTY. Reduction found a completion-phase deadline vacuum: once
the text proof completes and a button routes, the keyboard-sequence and pointer-selection
deadlines are disarmed and the global runtime budget intentionally stays out of input proofs,
so any stall after the proof loops without a bound. The loop exit requires the primary
client's reaped exit status; a toolkit that destroys its window but never exits leaves that
term false forever. On a bare text TTY without a session bus address, GTK finalization is the
prime suspect for the missing exit.

The session now bounds the post-removal wait: when the application proof surface is gone and
the client has not exited within five seconds, the loop fails closed with the exact exit-term
states instead of presenting blank frames. Application-proof clients launch under
`dbus-run-session --` when no bus address exists and the runner resolves on `PATH`, giving
the toolkit a bounded per-client bus that exits with the client. The first watchdog draft
armed on proof-complete-plus-button and falsely expired inside the QEMU click-then-submit
sequence; the retained trigger is surface removal, which is the actual abnormal state.

The full offline all-feature suite passes. The rebuilt X13 QEMU image passed strict two-xterm,
and resize-enabled classic and confined GTK passed exact text, routed button selection,
committed 640x360 redraw, normal exit, `first_error=none`, native presentation, and clean
two-output retirement. Fresh paired physical X13 evidence remains the acceptance gate; if the
watchdog fires there, its record names the missing exit term.


<!-- END IMPORTED BODY -->
