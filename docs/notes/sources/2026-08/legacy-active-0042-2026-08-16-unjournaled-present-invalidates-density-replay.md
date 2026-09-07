---
id: legacy-active-0042
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-16: unjournaled Present invalidates density replay

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1353–1373. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first run after head-affine image realization completed candidate and
  rollback preparation, applied all three heads atomically, presented both
  logical outputs, and committed topology epoch 2. It then terminated X
  Authority while reconciling the new physical-density demand with `failed to
  satisfy surface raster requirements: InvalidResource`.
- Standard pixmap and DRI3 Present replace surface pixels without an X drawing
  command that the semantic raster journal can replay. The old path left any
  earlier journal live. After a presentation resize, density satisfaction saw
  the old journal extent and elevated that private mismatch to a process-fatal
  resource error; at an unchanged extent it could instead replay stale commands
  and falsely label the result as an authority raster.
- Every unjournaled Present now invalidates the surface's replay state and
  derived variants at the actual presentation extent. Later density demand
  returns the existing bounded `unsupported_command` sampled fallback, while a
  stale journal extent returns `logical_extent_mismatch`; neither kills the X
  server. A later full opaque semantic baseline may recover replay. This does
  not weaken the extended-output proof: a client-rendered DMA-BUF at that
  head's exact native extent remains exact without derived raster replay.

<!-- END IMPORTED BODY -->
