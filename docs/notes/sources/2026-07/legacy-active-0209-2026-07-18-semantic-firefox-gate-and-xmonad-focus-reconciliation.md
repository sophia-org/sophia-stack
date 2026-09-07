---
id: legacy-active-0209
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy", "validation"]
---
# 2026-07-18: Semantic Firefox Gate And Xmonad Focus Reconciliation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7102–7128. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The M8 page proof no longer treats changing pixels or fixed host sleeps as
evidence for browser behavior. The X frontend reduces title changes to property
name and byte length only; XIDs, namespaces, and title bytes remain inside the
authority. Six monotonically sized witnesses prove load, keyboard, clipboard,
primary selection, resize, and dialog stages. Separate reduced counters require
at least two selection-owner changes and two conversions before the session can
publish a complete Firefox record.

The first evidence-driven close attempt exposed a bridge synchronization bug.
Xmonad could focus a newly managed synthetic window after producing its layout,
while the bridge returned without reconciling the private X server's current
focus. Engine therefore retained the previous opaque surface and a subsequent
close-focused action targeted the wrong client. The bridge now queries its own
synthetic focus after each quiet layout cycle and translates that opaque window
back to `FocusSurface`; no XID or client metadata crosses the WM boundary.

Application launch waits for both process start and a committed layout. Close
waits for the WM action acknowledgement and a zero-status managed-process exit.
The soak repeats those evidence-driven terminal, Firefox, and launcher restarts
instead of fixed-delay chords, and its verifier requires twenty restarts of
each application, sixty committed closes, the semantic Firefox proof, zero
unexpected protocol errors, and clean final ownership. Offline verifier
regressions and the complete all-features test suite pass. Real mix and soak
promotion remain pending their unattended QEMU runs.

<!-- END IMPORTED BODY -->
