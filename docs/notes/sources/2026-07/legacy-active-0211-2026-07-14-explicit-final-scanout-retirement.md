---
id: legacy-active-0211
date: 2026-07-14
recorded_date: 2026-07-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-14: Explicit Final Scanout Retirement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7161–7178. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The post-completion X11 allocator failure exposed a teardown ownership gap.
Persistent presentation deliberately retained the last displayed submission,
but the bounded session drained only the in-flight submission. Returning from
the loop therefore dropped the last GBM owner implicitly without first
retiring its framebuffer, mode blob, and imported GEM handles through the live
DRM device.

Persistent runtime shutdown now explicitly retires that displayed submission,
retries any reduced cleanup while the DRM device and renderer context are
still alive, and refuses clean completion if either in-flight or cleanup state
remains. Lifecycle diagnostics bracket the terminal retirement without logging
native handles. On X13, the focused backend regression and native-feature CLI
build pass, followed by ten of ten uninstrumented exact-text native stability
runs with clean evidence and no allocator diagnostic. The three operator-typed
runs remain the physical acceptance gate.

<!-- END IMPORTED BODY -->
