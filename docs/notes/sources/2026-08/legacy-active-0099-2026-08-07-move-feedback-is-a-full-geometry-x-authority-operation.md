---
id: legacy-active-0099
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "security"]
---
# 2026-08-07: Move feedback is a full-geometry X Authority operation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3261–3297. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed release `0.1.0-7bd3e7db0a90` proved the two-phase admission
successor, then exposed the next independent boundary defect. Super+Space
committed xmonad transaction 9 with three moved surfaces, but the session
reported only two configure deliveries and two matched resize candidates.
Those two Kitty surfaces changed size. Firefox retained its `1266x1412`
content while moving from the right column to the left, so the old size-only
control path sent it nothing. Engine rendered Firefox at `1266x1412_7_21`
while X Authority retained the old root-relative position. The proof page's
`screenX` therefore stayed stale and the layout stage did not advance. Later
pointer-focus transactions moved no surfaces; they merely made the split
Engine/X geometry visible as apparent Firefox jumping.

This is not xmonad policy or an Engine rendering defect. XLibre's
`ConfigureWindow` emits `ConfigureNotify` for a real pure move and invokes the
Present configure hook before core delivery. Yserver retains the same
position-and-size operation and has a pure-move ordering regression. Sophia's
X Authority now follows that ownership: `ConfigureSurface` carries the whole
logical rectangle, updates X position and size together, emits Present
ConfigureNotify before core ConfigureNotify for any real change, and remains
silent for an identical rectangle.

The session coordinator separately derives geometry controls and pixel
obligations. Every changed surface receives one full-rectangle control, while
only resized surfaces await new pixels; a move-only surface keeps its committed
pixels. Timeout recovery queues the complete last-committed rectangle, so a
late target control cannot leave X Authority at a stale position. Focus-only
proposals emit no geometry control. Deterministic Rust tests cover pure move,
no-op silence, move-only layout, focus-only layout, and full-rectangle
rollback. The `GeometryFeedback` TLA+ model explores delivery on either side
of logical commit plus late-target/FIFO rollback and requires terminal
Engine/X convergence. The physical Firefox verifier now requires the
correlated geometry acknowledgement and stable Present target through the
focus cycle. Installed `a50dfb67` proves this boundary reaches the live
session, but exposes the recovery-reseed omission recorded above.

<!-- END IMPORTED BODY -->
