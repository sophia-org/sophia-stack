---
id: legacy-active-0289
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "policy"]
---
# 2026-07-18: X13 GTK Blank Session Reduced To Tap Policy And Pointer Deadline

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9033–9056. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

A fresh classic hardware run accepted exact physical `sophia` input, committed the 640x360
GTK resize, presented the software cursor, and routed sustained touchpad motion. It emitted no
pointer-button record, no application-session record, and no bounded-completion record before
the independent emergency chord restored the TTY cleanly. X13's libinput report confirmed that
the ELAN touchpad supports tapping but defaults tap-to-click to disabled.

The native path-based libinput owner now enables tap-to-click for every tap-capable admitted
device, verifies the applied state, and exports only reduced device/tap counts. The proof loop
now distinguishes motion observed/routed from button observed/routed. Its selection deadline
remains armed after cursor pixels change and ends only after both a routed button and pointer
pixel evidence; this closes the prior unbounded state where motion canceled the only pointer
deadline. Cursor repaint also fails closed if an application proof produces no composed layer
or only the bounded software-cursor footprint.

The full offline all-feature suite passes. The rebuilt X13 QEMU image passed strict two-xterm
in 6,880 ms with 19 of 19 input deliveries, 40 submissions, 38 retirements, and zero native
debt. Resize-enabled classic and confined GTK passed exact text, routed button selection,
normal exit, `first_error=none`, and clean two-output retirement. A bounded non-KMS smoke
against the real ELAN path reported `devices=2 tap_capable=1 tap_enabled=1` and completed its
xterm pixel proof. Paired physical X13 evidence remains the acceptance gate before GTK3
software promotion.

<!-- END IMPORTED BODY -->
