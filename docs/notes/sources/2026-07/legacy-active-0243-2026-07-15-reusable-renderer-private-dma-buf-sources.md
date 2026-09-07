---
id: legacy-active-0243
date: 2026-07-15
recorded_date: 2026-07-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "tooling"]
---
# 2026-07-15: Reusable Renderer-Private DMA-BUF Sources

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8120–8140. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The renderer lifetime boundary now distinguishes a persistent DRI3 pixmap
source from one in-flight presentation. Plane FDs remain renderer-private and
reusable across Presents, while every presentation receives duplicated plane
and acquire-fence ownership in the existing bounded registry. Page-flip
retirement removes only the in-flight ownership; explicit source removal or
disconnect releases each persistent source once.

External tests use a real xshmfence to prove that an unsignaled acquire fence
holds submission, a trigger makes the presentation ready, page-flip retirement
allows the same source to be presented again, an in-use source cannot be
removed, and disconnect cleanup is idempotent. The complete offline all-feature
workspace suite passes with this reusable lifetime model. Live-session import,
mixed CPU/GPU composition, and page-flip-driven Present feedback remain open.
The X frontend also exposes a cloneable protocol-only feedback router that can
emit Present Complete and Idle after the broker moves into its service thread.
It is intentionally not attached to the current CPU fallback submission: doing
so would acknowledge a page flip that did not contain the imported Vulkan
pixels.

<!-- END IMPORTED BODY -->
