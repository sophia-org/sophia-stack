---
id: legacy-active-0442
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: live output IPC crosses the physical publication barrier

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13266–13285. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The session now retains candidate and rollback affine owners through an
  ordered per-card modeset transaction. Accepted candidate owners are installed
  directly into rebuilt output runtimes, avoiding a second modeset, and one
  native-size cohort is queued for every replacement logical output.
- Engine, X topology, WM work areas, pointer bounds, and input authority remain
  coordinated around an all-output first-presentation barrier. Only then does
  `sophia_output_v1` publish the new topology and release the rollback pool.
- A renderer/native service failure before that barrier is no longer a session
  escape: physical rollback is requested first, the Engine transaction enters
  `RollingBack`, and completion retains the same recovery record. Supervised WM
  restart or output-peer loss uses the same handshake and defers connection-epoch
  replacement until reverse apply settles.
- Hardware hotplug projects fresh capabilities and an authority snapshot from
  the replacement native owner, but publishes them to the optional output
  service only after replacement scanout presents. Remaining critical work is
  deterministic effect-path failure coverage, per-head DMA-BUF/retained
  lowering, and the signed physical multi-monitor gate.

<!-- END IMPORTED BODY -->
