---
id: legacy-active-0472
date: 2026-08-19
recorded_date: 2026-08-19
date_basis: first-heading-commit
date_commit: 98605bdcbe3463349a208223f571b508c21a29ae
committed_at: 2026-08-19T07:25:49-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy"]
---
# Optimizing a mirror group for its smaller head shrinks the desktop under a policy

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14325–14349. The heading has no date. Its first recorded addition is commit
`98605bdcbe3463349a208223f571b508c21a29ae` (2026-08-19T07:25:49-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The first run with `--optimize-for=DP-3` applied the topology cleanly --
`first_presented transaction=1 outputs=2`, both heads composed at the new sizes
-- and then died on `InvalidSurfaceGeometry` at the next policy projection,
followed by a cleanup failure: "topology rollback runtime rebind failed:
logical viewport replacement omitted an output".

Optimizing for the member is the first change that makes a logical output
smaller. The mirror group goes from 2560x1440 to 1920x1080 and the desktop
union from 4480 to 3840 wide, so a placement that fitted before the commit may
not fit after it. `validate_output_projection` refuses such a placement, which
is right, and the session ends over it, which is the part worth questioning:
a projection written against the previous topology is stale, not invalid.

The rejection said only `InvalidSurfaceGeometry`, which cannot distinguish a
policy proposing nonsense from a policy that has not yet seen an output shrink
beneath it. It now carries the surface, the placement, the work area, the
output bounds, and whether the placement claimed fullscreen -- the same
treatment `SourceSizeMismatch` needed an hour earlier, for the same reason.
The empty-geometry case in the scene validator became its own variant rather
than sharing a name with the fitting case: one is a malformed surface, the
other is a placement that no longer fits, and reading a bare error could not
tell you which.

<!-- END IMPORTED BODY -->
