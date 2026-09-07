---
id: legacy-active-0533
date: 2026-08-24
recorded_date: 2026-08-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "tooling"]
---
# 2026-08-24: a queued Present must source the scene it plans

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16380–16417. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next signed run repeated `MissingCpuSource(4)` a third time, and this time the
handle was resident throughout. Committed content roots every CPU variant, and the
browser's raster was its canonical and only one, so no eviction could reach it. Two
rounds of residency work had been necessary and were aimed at the wrong mechanism.

The defect is skew, not lifetime. `drive_gpu_presentation` prepares its transaction
against the live committed scene and builds every display list from that rebased
candidate, while its sources came from CPU layers frozen when the Present was
enqueued. Releasing a layout-deferred Present rebased only the previous committed
generation, never the layer set. The guide's Present was staged behind layout epoch
20 before Super+B existed; the browser was admitted while it waited; the focus change
released it and page-flip retirement submitted it, so the plan named a surface its
own sources predated. A silent match arm for an unsourced surface turned that into a
lowering failure one layer down, where the error names a handle rather than the skew.

Present submission now derives its sources from the same candidate slice it plans,
the way retained composition always has, and the enqueue-time snapshot is deleted
rather than refreshed: a snapshot that must be kept current is the same defect with a
shorter window. A candidate surface with no source is now refused where it is found,
leaving one deliberate skip for a surface policy ordered before its pixels committed,
which the planner drops on its own. The presenting surface also contributes any CPU
variants the candidate still carries, because a head may select a retained raster
over the buffer being presented. A differential regression drives a parked Present
past a late admission and requires the live source set to resolve the browser while
the enqueue-time set still fails with exactly `MissingCpuSource(4)`.

Four adjacent plan/source pairings were audited and left alone as latent rather than
live: the CPU-layer path reads committed state before its batch commits, retained
lowering deliberately precedes the cycle's own scene update, the retained source set
builds from the primary output's display list because Surface commands are
viewport-independent, and the GPU-cycle guard checks the pre-commit set. The scene's
`presentation_layers` and `presentation_variant_layers` still drop unresolvable
handles silently; every failure of this class has surfaced downstream of one of those
drops, and making them total is a wider change than this defect justifies. The signed
installed rerun remains the promotion gate.

<!-- END IMPORTED BODY -->
