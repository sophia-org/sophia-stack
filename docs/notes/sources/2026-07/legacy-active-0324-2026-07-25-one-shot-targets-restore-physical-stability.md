---
id: legacy-active-0324
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-25: One-Shot Targets Restore Physical Stability

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10112–10128. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical cycle after removing every retained GPU target completed
normally with 253 mixed exports. Target, pipeline, frame-surface, and successful
target-retirement counts were all 253. Generation and recovery replacement
were zero, no AMDGPU command stream was rejected, native submissions and
callbacks drained, and the lifecycle returned exit status zero.

The strict gate exposed a separate owner-scheduling defect. Of 254 callback
observations, 248 completed within 20 ms and three exceeded 100 ms. Two
213--214 ms stalls coincided with managed terminal exit and xmonad resize
epochs; a 171 ms stall covered the final blank frame after the startup terminal
exited. Target creation peaked at 4 ms and rendering at 47 ms, while input
queue dwell reached 191 ms. The next fix must prioritize native callback and
input draining across child-exit and layout-transition work. The latency budget
must not be relaxed to hide these isolated starvation events.

<!-- END IMPORTED BODY -->
