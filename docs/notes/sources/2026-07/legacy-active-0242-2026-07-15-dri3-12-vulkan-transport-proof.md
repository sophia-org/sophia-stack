---
id: legacy-active-0242
date: 2026-07-15
recorded_date: 2026-07-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-07-15: DRI3 1.2 Vulkan Transport Proof

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8092–8119. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The X11 socket output boundary now sends a bounded byte record plus up to four
SCM_RIGHTS descriptors. Standard DRI3 `Open` obtains a duplicated render-device
FD only from the live backend provider and returns it in a one-FD reply; neither
the authority runtime nor Engine stores a device path or native handle.

Mesa's DRI3 1.2 startup required `GetSupportedModifiers`, modifier-bearing
`PixmapFromBuffers`, and the small XFIXES region lifecycle used by Present. The
portable modifier reply advertises linear plus the implicit-modifier sentinel,
and the multi-buffer decoder retains bounded plane strides, offsets, and the
wire modifier in the reduced DMA-BUF descriptor.

The first Vulkan failures were caused by Unix-stream FD association rather than
an AMD modifier. A single `sendmsg` can attach descriptors to bytes preceding
the X11 request that consumes them. The server now queues ancillary FDs in
stream order, leaves them pending across no-FD requests, and drains exactly the
declared arity for each later FD-bearing request. A deterministic regression
sends two descriptors alongside an earlier no-FD XFIXES request and proves that
the following DRI3 pixmap and fence requests consume one each.

On the Void Linux X13 with Mesa RADV, the bounded DRI3 1.2 `vkcube` run remained
healthy for its eight-second proof window: 68 requests, three imported pixmaps
and fences, one accepted standard Present transaction, one committed runtime
surface, and `first_error=none`. This proves Vulkan transport into the Engine
transaction seam; it does not yet claim native KMS presentation of the Vulkan
pixels.

<!-- END IMPORTED BODY -->
