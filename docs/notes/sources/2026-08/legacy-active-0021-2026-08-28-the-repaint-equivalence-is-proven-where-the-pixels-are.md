---
id: legacy-active-0021
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-28: the repaint equivalence is proven where the pixels are

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 712–745. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Damage-limited repaint is implemented end to end and proven pixel-identical
  on the promoted host's own GPU. `tools/check_buffer_age_equivalence.sh`
  drives an identical twelve-frame mixed sequence through the real render
  path twice -- damage-limited and forced-full -- through a render node only:
  no DRM master, no display takeover, safe inside a live desktop session.
  Captured checksums match frame by frame, and the run asserts that partial
  repaints actually occurred, because identical checksums prove nothing if the
  feature never fired. A negative control renders against a lying damage table
  that claims an aged buffer owes nothing while a tile changed; the checksums
  diverge, so the comparison is load-bearing rather than decorative.
- Two findings from making the smoke run. Pixel capture is budgeted to three
  startup frames because per-frame `glReadPixels` is hot-path poison, so the
  context gained a smoke-only override past the budget. And a render node must
  be opened read-write: the GPU maps buffers through the descriptor, and a
  read-only open fails inside Mesa with EACCES and a segfault rather than at
  open. The probe helper's openability check uses a read-only open, which is
  fine for its purpose and a trap for anyone copying it.
- The feature shipped enabled and that had the risk backwards, so it is now
  opt-in via `SOPHIA_ENABLE_BUFFER_AGE_DAMAGE=1`. Its failure mode is a frame
  that is presentable, self-consistent, and stale in one region, which no
  health check would catch and an operator would read as a rendering glitch.
  Off by default costs a repaint; wrong by default costs trust in the
  evidence. The native gate exports the switch as the promotion step, and
  schema-8 `sophia_live_native_resources` evidence must show at least one
  partial repaint -- a promotion run in which the boundary never fired is not
  the run being promoted. Archive `0001` predates the feature and stays
  verifiable through its schema-7 shape.
- The gated equivalence tests skip silently without a device, and a skipped
  proof looks exactly like a passing proof from the outside, so the check
  wrapper refuses when no render node is writable instead of reporting
  success it did not earn.

<!-- END IMPORTED BODY -->
