---
id: legacy-active-0473
date: 2026-08-19
recorded_date: 2026-08-19
date_basis: first-heading-commit
date_commit: 259671b8a781aa59880b72a5fe6d64c7dd5e23e5
committed_at: 2026-08-19T08:08:45-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# A scene reports where surfaces are; only a proposal asserts where they belong

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14350–14377. The heading has no date. Its first recorded addition is commit
`259671b8a781aa59880b72a5fe6d64c7dd5e23e5` (2026-08-19T08:08:45-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The enriched rejection named it in one run: surface 2097166 at 1280x1440,
work area 1920x1080, not fullscreen. That 1280x1440 is the old tile -- half of
the 2560x1440 mirror group, from the two-surface layout that was correct until
the group was re-optimized onto its 1080p member. The width still fits. The
height cannot.

The rejection did not come from a policy proposal. `validate_scene` was
deriving a projection from the scene's own surfaces and running it through
`validate_output_projection`, which requires every placement to sit inside its
output. That is a true statement about any layout a policy authors, and a
false one for the instant after an output shrinks beneath surfaces that have
not been laid out again yet. The compositor was describing exactly what was on
the screen and being told the description was invalid.

So the fit rule stays where a placement is asserted -- `validated_candidate`,
which judges what a policy proposes -- and leaves the path that reports what
already exists. A scene keeps its shape checks: valid surfaces, non-empty
geometry, unique identity, declared constraints, transient owners. Nothing was
lost by dropping the projection pass over it; every other check it applied is
satisfied by construction when the projection is built from the scene itself.

This is the same shape as the run before it. A retained copy was being held to
a measurement of the buffer it was copied from; a description was being held to
the standard of a proposal. Both were one rule applied to two different kinds
of claim.

<!-- END IMPORTED BODY -->
