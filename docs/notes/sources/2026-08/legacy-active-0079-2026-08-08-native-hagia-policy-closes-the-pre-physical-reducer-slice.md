---
id: legacy-active-0079
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-08: Native Hagia policy closes the pre-physical reducer slice

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2686–2723. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The critical-path profile is now one fixed nine-view scroller rather than an
attempt to reproduce Triad's full runtime before the public boundary settles.
Hagia retains stable logical window/output identities, bounded focus and
minimize histories, output focus, consume/expel and size actions, floating and
fullscreen/maximize/minimize state, and completed Engine-reduced pointer
geometry. General tag mutation, scratchpads, continuous pointer phases, and
additional layouts remain outside this promotion slice.

Revision-1 now carries the missing lifecycle facts directly. Snapshots and
proposals name one explicit active output; requests carry the strictly admitted
private policy generation; and committed bindings explicitly reference an
optional advertised session-operation slot. Numeric action ranges no longer
confer session authority. Canonical validation requires an active-output switch
to replace both old and new outputs, fullscreen geometry to equal complete
output bounds, nonfullscreen geometry to remain in the work area, and a
minimized surface not to hold focus. Minimized placements remain semantic but
do not enter the render-layer candidate.

Idle `PolicyDirty` admission is generation-fenced and output-bounded. Pending
scopes coalesce, while a newer generation arriving during an in-flight refresh
remains pending for a later complete cycle. Reducer and layout successors still
promote only after frontend settlement. Hagia's independent Nim codec and
reducer implement the same contract, and its private checkpoint is bounded,
validated, atomically replaced, and reconciled against the next complete
snapshot. Sophia supplies that checkpoint inside the owner-only endpoint
directory so it survives supervised child replacement but not session teardown.

`PolicyRefreshLifecycle.tla` passed independently and in the pinned complete
TLA+ gate. Its temporary non-atomic active-output control produced the expected
counterexample. `PolicyOperationBinding.als` and
`PolicyPresentationGeometry.smt2` passed the official Alloy 6.2.0 and pinned Z3
4.16.0 gate, including satisfiable weakened attacks; the local Z3 5.x
differential matched. The remaining promotion evidence is an opt-in installed
physical run proving checkpoint restore, presentation transitions, active
output, and refresh behavior without losing the standing application scene.

<!-- END IMPORTED BODY -->
