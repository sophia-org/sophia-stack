---
id: legacy-active-0486
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: ece6b4f96f33092645d177a0c5c7b5f408f48f2e
committed_at: 2026-08-21T14:31:03-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# What the gate's sharpness verdict actually proves

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14794–14832. The heading has no date. Its first recorded addition is commit
`ece6b4f96f33092645d177a0c5c7b5f408f48f2e` (2026-08-21T14:31:03-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Reading the composition path for the gamma work turned up something unrelated
and worse. The mixed-output gate reports `extended_text=sharp`, and the evidence
behind that phrase cannot distinguish a sharp frame from a resampled one.

Two derivations, unconnected. The engine's `sampling_class` compares two
`density_millis` scalars -- a property of the output-to-head projection that
knows nothing about any raster's extent. The renderer's
`native_composition_sampling` compares the actual source and target rectangles.
The parameter the renderer names `requested_sampling` is its own value, not the
engine's: the two names are homonyms across a boundary nothing crosses.

They disagree four ways. Content whose `logical_extent` differs from the
geometry it is placed into -- the ordinary state of any surface mid-resize, and
documented as legitimate. Rounding, because the projected density truncates
while the expected pixel size ceilings and each projected edge truncates on its
own, so every scale that is not dyadic-clean lands a pixel off; the engine's own
test happens to use exactly 0.75, where they agree. Mixed-axis projections,
where one scalar is asked to describe an upscale in x and a downscale in y.
And retained renderer images, exempt from the source-size check by design, whose
class is therefore computed from a number that is not the one drawn.

So the string the verifier greps for is satisfiable by a frame the GPU filtered,
and the single renderer-side check in that script rejects `status=fallback` and
`status=unavailable` while passing `requested=sharp_downscale status=active` --
the literal signature of the resample. There is also a quiet consequence beyond
the evidence: the plan's one-pixel repaint dilation for filter footprint is
gated on the plan's class, so it is skipped in exactly the cases where the
renderer is filtering and the footprint does exceed the rect.

It is the same defect as the raster-versus-presentation extent split landed a
few commits earlier, in a different place: one fact derived twice from different
data with nothing forcing agreement. It does not block the gamma work, whose
evidence comes from luminance rather than from this field, so it stays a
separate change rather than being folded in. Recorded here because it was found
by reading, not by failing, and a gate that overstates what it proves is worth
writing down the moment it is noticed.

<!-- END IMPORTED BODY -->
