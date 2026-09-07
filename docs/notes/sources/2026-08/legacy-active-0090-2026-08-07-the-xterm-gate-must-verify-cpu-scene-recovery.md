---
id: legacy-active-0090
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "rendering", "validation"]
---
# 2026-08-07: The xterm gate must verify CPU-scene recovery

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3011–3030. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed xterm attempt `xterm-runs/0002` launched correctly, committed a
2556-by-1422 backing snapshot inside the primary work area, switched VTs with a
drained native owner, reacquired both outputs, retired new primary frames, and
logged out cleanly. The automatic result nevertheless failed because the new
fixture had modeled xterm as a Present client with one imported renderer image.
The live contract is deliberately different: xterm's ordinary drawing commits
an X Authority CPU backing snapshot, while the Engine scene retains those
pixels across renderer replacement. No imported image exists to capture.

The successor gate now records the atomic source and target geometry when a CPU
backing snapshot is admitted. It requires that commit to reach native retirement
before startup readiness, requires an exact zero-image renderer handoff, and
records the nonzero Engine scene rehydrated on both outputs before post-resume
retirement. This proves recovery without depending on a static client to issue
a new Present or repaint after VT reacquisition. The imported-image handoff
contract remains covered by Firefox and Vulkan rather than being imposed on
the CPU-only terminal path.

<!-- END IMPORTED BODY -->
