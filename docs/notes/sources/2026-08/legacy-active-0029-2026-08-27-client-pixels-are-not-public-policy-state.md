---
id: legacy-active-0029
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy"]
---
# 2026-08-27: client pixels are not public-policy state

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 992–1017. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first four-Kitty attempt after in-place xmonad recovery did not crash.
  All three `Super+Enter` activations and both observed `Super+J` activations
  reached committed policy actions, native shutdown drained without an
  abandoned scanout, and emergency recovery restored the TTY. Interactivity
  was nevertheless unusable: the 56-second run rejected 19 stale responses,
  rebuilt the private xmonad adapter 17 times, and restarted the outer bridge
  twice. Raw input queue dwell remained at 2 ms, so input acquisition was not
  the latency source.
- The public snapshot populated `SnapshotSurface.state_generation` from
  `LayerSnapshot::generation`. That layer field is committed raster identity
  and advances as Kitty draws; it is not the X window-lifecycle generation
  described by `sophia_wm_v1`. A client repaint therefore advanced the whole
  scene while a policy response was in flight, rejected the response, and
  discarded xmonad's speculative focus/layout state.
- The snapshot now takes state generation from the retained X-authority
  surface facts and uses raster generation only as a fallback for authorities
  that provide no separate lifecycle facts. A regression retains authority
  generation 7 while an admitted layer advances to raster generation 91. Real
  surface withdrawal and authority lifecycle changes still advance the public
  state and preserve fail-closed stale-response rejection. Signed replacement
  source `c681f762` reduced the stale count to the three real surface-arrival
  races, confirming this repair. Further xmonad validation is compatibility
  work; the Sophia WM and shell protocol gate above owns the product path.

<!-- END IMPORTED BODY -->
