---
id: legacy-active-0203
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy"]
---
# 2026-07-25: Retained Frames Follow One Atomic Layout Snapshot

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6947–6968. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

A physical three-Kitty xmonad run kept submitting and retiring KMS frames but
showed a blinking third tile, then only two visible tiles after a TTY
round-trip. The trace proved that client DMA-BUF sources remained valid while
individual Present records crossed a two-to-three-window layout transition with
different geometry. Per-surface page-flip health was therefore not sufficient
full-state evidence.

The production runtime keeps retained frame ownership separate from
placement. Every cycle projects queued and displayed Presents through one
stack-ordered WM layout snapshot before composition. DMA-BUF pixels remain
unscaled; clipping is not accepted as proof that a client completed a resize.
Present source offsets remain frontend facts carried to the generic backend. The first displayed
buffer stays busy until replacement instead of receiving an immediate Idle.
Native resume also queues a complete retained mixed frame, so quiet clients do
not have to repaint merely to recover visible contents after a VT switch.

This is protocol-neutral visual state. The Engine and renderer contain no
Kitty or xmonad policy. Physical promotion remains open until three windows
stay visible before and after a TTY2/TTY3 round-trip.

<!-- END IMPORTED BODY -->
