---
id: legacy-active-0231
date: 2026-07-11
recorded_date: 2026-07-11
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "tooling"]
---
# 2026-07-11: Bounded Per-Output Timelines And Two-Connector QEMU Topology

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7726–7743. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Engine output discovery is now bounded to 16 descriptors. Backend assembly no
longer advances one global deterministic clock: it seeds an independent clock
for each discovered output using that output's fixed refresh rate. A separate
presentation registry tracks pending damage, one in-flight frame, exact
retirement, and the last retired serial per output. Two-output regressions prove
that 60 Hz and 120 Hz timelines advance independently, one output cannot submit
over an unretired frame, and a mismatched retirement cannot clear ownership.
These are scheduling invariants; the clock is not yet driven by DRM vblank.

A single virtio-gpu device configured with two scanouts exposed two connector
objects but only one connected connector, so it was rejected as multi-monitor
evidence. The accepted harness uses two isolated virtio GPU devices with one
scanout each. The guest reports two connectors and both connected; Engine
discovers two and creates two presentation timelines. That topology was the
prerequisite for native multi-output ownership, which is recorded below.

<!-- END IMPORTED BODY -->
