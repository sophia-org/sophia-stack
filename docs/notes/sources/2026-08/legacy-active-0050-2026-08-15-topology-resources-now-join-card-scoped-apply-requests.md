---
id: legacy-active-0050
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-15: topology resources now join card-scoped apply requests

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1576–1595. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`HeadlessOutput` has no origin, so retaining only its extent during candidate
resolution erased the distinction between a real extended layout and several
overlapping outputs. Resolved candidates now retain ordered root-space logical
viewports. A read-only visual-runtime seam captures each viewport from one
committed surface slice, selects per-head content variants, and lowers native-
size frames; a deterministic spanning-surface test proves both halves of an
extended desktop are produced from the same scene generation.

Renderer topology preparation now uses blocking modeset policy and retains the
renderer owner, framebuffer/imports, and mode blob without submitting. Prepared
heads expose copied atomic contributions while keeping their resources affine.
The new card request builder combines independently buffered enabled heads and
explicit disabled heads, rejecting object overlap before adding properties. A
successful combined commit can adopt prepared owners into ordinary scanout
retirement; failed/unsubmitted work remains cancellable. The remaining live
transaction must prepare a rollback pool, submit cards, roll back prior cards on
failure, rebuild runtime targets, and wait for first presentation before publish.

<!-- END IMPORTED BODY -->
