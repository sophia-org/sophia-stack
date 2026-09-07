---
id: legacy-active-0611
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "tooling"]
---
# 2026-09-04: a retained CPU snapshot must not replace a new Present source

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19358–19411. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The `f92acb2f` canary still showed a black Firefox content area. Final-region
readback measured zero nonzero RGB pixels in its 1266x1408 content at
`1285,23`, despite page readiness and software retirement records. The earlier
description of full frames rendered was geometry/retirement evidence, not a
browser visual pass. The blue square was unrelated: it was the indicator
strip's intentional output-focus marker.

A reproducing authority test found the source-selection defect. Firefox first
paints a CPU startup background, then imports a DRI3 pixmap and Presents it on
a child. `finish_drawing_update` replaced the selected DMA-BUF with the retained
CPU snapshot whenever that snapshot existed. Socket dispatch consequently
classified the GPU Present as software and bypassed its GPU offset/fence
route. Raster-variant expansion now requires exact CPU source-handle identity.
A second regression exposed software Present publishing the child rather than
the presentation root. CPU, SHM, and GPU now resolve that root before branching
by storage; CPU raster extents describe the materialized parent backing.

The image boundary now validates core packed/planar formats, depth, padding,
byte order, and complete payload length before mutation. GC raster functions,
plane masks, and clipping are applied to canonical pixels. The common
little-endian 32-bit GXcopy path still borrows input and copies clipped rows;
copy-on-write remains once per operation. Tests cover both byte orders,
advertised depths, planar/bitmap colors, negative destinations, masks, invalid
payload atomicity, and Firefox's 1266x1408 image split into 27 51-row uploads and
one 31-row tail. An old SHM test used a fictitious OS segment and accidentally
relied on fabricated pixels; it now asserts rejection with no publication.

The Firefox rendering verifier no longer hard-codes the old left-column
geometry or treats metadata as visibility. Engine emits opt-in surface/scene
geometry, and the renderer associates final-region readback with that opaque
head scene. The verifier joins these through the queued frame to native
retirement and requires two changing nonblack regions. Black, static,
wrong-surface, wrong-head, stale-generation, clipped, and unretired evidence
fail. The offline page alternates its background and counter every 500 ms.
Existing region schema readers remain compatible; the causal identity record
is additive. Verbose tracing preserves an explicitly selected readback mode.

Logout now closes client streams while preserving accepted ordered egress,
then waits for resource removal and the existing bounded native drain.
`DrainAndDisconnect` is distinct from emergency `StopAndDisconnect`, which
cancels blocked producers only after drain failure. A rendezvous-channel test
proves graceful teardown drains accepted work while the client socket remains
open. Focus uses the existing active palette on the layout label instead of
a detached square; no styling policy or X identity enters the blind WM.

Verification: 305 focused authority/wire regressions pass, including both
source-selection tests that failed before their fixes; the full offline
`cargo xtask check` passed, including the host buffer-age pixel-equivalence
check and archived verifier corpus. This is implementation evidence only.
One fresh signed physical Firefox rendering canary remains required before
restarting the 36-row comparison; the optional overnight soak is unchanged.

<!-- END IMPORTED BODY -->
