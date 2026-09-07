---
id: legacy-active-0265
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-18: GPU Present Uses One Engine Snapshot Across Outputs

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8618–8636. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The live Present path no longer prepares and applies the same GPU transaction once per
output assembly. It prepares against the primary production coordinator snapshot, applies
that prepared commit exactly once after the matching page flip, and projects the resulting
immutable committed snapshot to the remaining outputs. This removes a multi-output visual
authority fork while preserving per-output scanout state.

A focused coordinator regression proves that applying a prepared Present mutates the
coordinator-owned snapshot, and the full offline all-feature suite passes. The rebuilt X13
QEMU image passed strict two-xterm in 6,907 ms with 120 of 120 authority transactions,
5 ms input presentation, 38 submissions, 36 retirements, and zero phase or cleanup debt.
Classic and confined GTK passed exact physical text and pointer selection, normal Zenity
exit, `first_error=none`, and clean two-output retirement. Emergency recovery flushed all
five routed chord deliveries and shut down cleanly in 187 ms. Runtime/scanout
invocation and the retirement-to-commit trigger still reside in `live_session.rs`; moving
that sequencing behind the production adapter remains the next Milestone 6 boundary.


<!-- END IMPORTED BODY -->
