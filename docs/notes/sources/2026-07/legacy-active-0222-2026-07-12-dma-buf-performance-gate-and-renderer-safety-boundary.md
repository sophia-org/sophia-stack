---
id: legacy-active-0222
date: 2026-07-12
recorded_date: 2026-07-12
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "validation", "architecture"]
---
# 2026-07-12: DMA-BUF Performance Gate and Renderer Safety Boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7436–7458. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The current native Wayland/Kitty presentation route is SHM-backed and stable
enough to serve as the production fallback, but the latest hardware result was
about 110 ms input-to-presentation and therefore missed the 100 ms budget.
DMA-BUF descriptors are admitted only as a bounded single-plane linear subset;
their native import and presentation path remains explicitly experimental.
There is no passing real-hardware DMA-BUF result at this point.

A controlled external Wayland producer now allocates linear XRGB8888 GBM
buffers, alternates them only after `wl_buffer.release`, and waits for each
frame callback. The first hardware gate uses three frames; the second uses 300
frames to exercise import, presentation, feedback, and retirement lifetime.
Only after both pass may the three independent guarded real-Kitty runs begin.
Those acceptance runs remain on SHM until GPU composition exists. DMA-BUF stays
non-default until a real-Kitty GPU-composition log proves input, recovery,
presentation, and the 100 ms budget.

The current CPU composition copy and 2 ms native idle cadence are a safety
boundary, not merely a tuning choice. Removing the copy or tightening that loop
has reproduced native renderer/exporter heap corruption on hardware. Further
latency work must isolate that ownership fault before changing either setting.

<!-- END IMPORTED BODY -->
