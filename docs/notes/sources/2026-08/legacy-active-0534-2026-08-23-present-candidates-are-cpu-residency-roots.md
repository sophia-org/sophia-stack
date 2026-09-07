---
id: legacy-active-0534
date: 2026-08-23
recorded_date: 2026-08-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-23: Present candidates are CPU-residency roots

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16418–16440. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next signed run repeated `MissingCpuSource(4)`, which proved that joining
the two source kinds was necessary but insufficient. The corrected source
builder could carry every CPU variant it received; at the final repaint,
`presentation_variant_layers` no longer received handle 4 from the scene.

CPU residency covered Engine-committed content, the current authority batch,
deferred groups, staged handles, and a bounded recent-update cache. It omitted
the Present scheduler. A queued or in-flight candidate can outlive both its
authority batch and the recent cache while still being the exact immutable
scene used for retained composition. Its content identity survived in the
scheduler, but the registry holding its CPU pixels was free to evict it.

The scheduler now exposes every CPU handle named by queued transactions and by
the complete in-flight candidate. Both CPU and GPU production cycles add those
handles to residency before reconciliation. The root ends only when the queued
candidate is rejected or the full output cohort retires and commits, at which
point ordinary committed residency takes over. One regression pins handle 4 in
an in-flight DMA-BUF candidate's alternate CPU variant; the earlier regression
still pins the renderer-image and CPU-source join. A new signed installed run
remains the promotion gate.

<!-- END IMPORTED BODY -->
