---
id: legacy-active-0456
date: 2026-08-17
recorded_date: 2026-08-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-17: the mixed-output gate died asking for a raster of a DMA-BUF

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13700–13749. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- `tools/run_mixed_output_gate_tty4.sh` failed every run on `3a4398a1` at
  `stage=runtime exit=1` with `failed to satisfy surface raster requirements:
  InvalidResource`. The evidence contains no `sophia_x11_raster_requirement`
  line at all, so the X authority server died answering the first requirement
  it ever received, immediately after
  `sophia_live_output_authority status=first_presented` — the moment the mixed
  topology brought a 750-density head into existence.
- Two defects, one on each side of the boundary. Engine's requirement tracker
  had no content-source filter, and `SurfaceContentSet::singleton` labels every
  single-variant set `AuthorityRaster`, renderer buffers included. So a
  non-1000 head made Engine ask X Authority to produce a 750-density variant of
  vkcube's DMA-BUF. X Authority then answered an unanswerable question by
  failing: a surface with no CPU presentation snapshot returned
  `InvalidResource`, the connection loop propagated it with `?`, and one
  surface's demand ended the display server.
- Why it surfaced only now: the crash needs a DMA-BUF client *and* a
  non-1000-density head. The unequal mirror gate had the head and only CPU
  clients; every other session had renderer clients and only 1000-density
  heads. The mixed gate is the first run with both, so it died deterministically
  at the same instant every time.
- The neighbouring `Invalidate raster replay across Present` change is correct
  and was not the cause. It poisons the journal for a surface that had CPU
  content and then presented, which degrades cleanly to sampled fallback. This
  crash needs a surface that never had a CPU drawable at all, which
  invalidation cannot reach.
- Fixes: Engine raises requirements only for CPU-backed canonical content;
  X Authority reports an absent canonical drawable as the new
  `no_canonical_raster` cause and treats the store's refusals as answers rather
  than runtime errors; and the connection loop no longer propagates a
  per-surface error at all — it warns with `status=refused` and keeps serving.
  The last of those is the one that matters longest: a surface can switch CPU
  to renderer content while a requirement is in flight, so the race outlives
  the demand filter, and no single surface should ever be able to take the
  server down.
- Rejected: relabelling `singleton` content as `SampledFallback` for non-CPU
  sources. That label is load-bearing in head-plan selection, where it would
  turn natively correct renderer buffers into `Fallback` bindings. The demand
  filter is the right place.
- Left as-is deliberately: the malformed-input guards at the top of
  `apply_surface_raster_requirements` (`requirements.validate()` and
  `transaction.is_valid()`) still return errors. Those indicate a programming
  fault rather than a legitimate content state, and the connection loop now
  contains them without dying.
- Retained as a regression that reproduces the exact failure offline: a
  requirement against a window that never core-drew must return
  `no_canonical_raster` rather than an error. Reverting the fix makes it fail
  with the gate's own `InvalidResource`.

<!-- END IMPORTED BODY -->
