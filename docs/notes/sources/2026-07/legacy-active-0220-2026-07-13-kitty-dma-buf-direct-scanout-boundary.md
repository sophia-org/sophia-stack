---
id: legacy-active-0220
date: 2026-07-13
recorded_date: 2026-07-13
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "architecture"]
---
# 2026-07-13: Kitty DMA-BUF Direct-Scanout Boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7364–7383. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Enabling the experimental DMA-BUF global for guarded Kitty failed before a
usable native presentation and surfaced the misleading scheduler invariant
`native frame was neither submitted nor retained for a later submit`,
disconnecting Kitty. Sophia's current DMA-BUF route is direct KMS scanout,
whose exporter requires the client buffer to match the physical output exactly.
An arbitrary Kitty toplevel therefore cannot be a valid client for that route,
regardless of the exact buffer that reached the failed run.

This is an architecture boundary, not evidence that Kitty can use the current
direct path. The controlled full-output XRGB producer remains the direct
DMA-BUF lifetime proof. The interactive Kitty harness now deliberately does
not advertise DMA-BUF and continues to prove native SHM composition, input,
recovery, and latency. The next DMA-BUF milestone is GPU composition: import
an arbitrary window-sized client DMA-BUF, scale/blend it into a Sophia-owned
output-sized render target, retain it through the target page-flip retirement,
and only then release the client buffer. Only that route can support Kitty
without requiring fullscreen, output-sized buffers.

<!-- END IMPORTED BODY -->
