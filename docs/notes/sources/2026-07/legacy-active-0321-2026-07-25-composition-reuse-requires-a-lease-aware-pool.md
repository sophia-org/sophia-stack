---
id: legacy-active-0321
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-25: Composition Reuse Requires A Lease-Aware Pool

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10038–10065. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The successful keyboard proof still reported 201 composition-target and GL
pipeline creations for 201 mixed exports. CPU and direct DMA-BUF paths already
returned their persistent target to the renderer after exporting a locked
front buffer. The mixed-composition path instead destroyed its target on every
successful export even though the exported buffer retained an `Arc` lease on
the GBM/EGL surface until scanout retirement.

An attempted optimization returned the context, surface, and GL pipeline to
the per-output target slot after a successful export. The exported front
buffer independently retained the originating surface, and the verifier was
temporarily changed to require zero target recreations.

The first physical run of this lifetime change aborted on the third render
after AMDGPU rejected the command stream. Moving startup proof from a
post-swap CPU map to a pre-swap GL readback produced the same third-render
abort, disproving front-buffer mapping as the root cause.

The invalid lifetime is single-surface reuse while a front buffer from that
GBM surface remains leased to KMS. Mixed composition therefore returns to the
previous fail-safe rule: destroy its rendering target after each successful
export while the exported buffer's independent surface lease survives through
scanout retirement. Future reuse must use a bounded lease-aware pool and only
select a target whose surface has no exported buffer owners. Verbose tracing
still captures one representative composition frame instead of synchronously
reading every frame.

<!-- END IMPORTED BODY -->
