---
id: legacy-active-0055
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "architecture"]
---
# 2026-08-15: per-head composition review fixes the prerequisite contracts

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1685–1709. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Review of the multi-monitor architecture found two load-bearing divergences
  in `VisualRetirement.tla`: one global committed generation accidentally made
  unrelated outputs supersede each other, and no guard prevented one physical
  head from being in flight for two generations. The model now commits per
  output, prepares and submits per head, reserves overlapping output cohorts,
  and joins transaction feedback only after every required output retires.
- A deliberate premature-completion control initially passed: it settled the
  generation as superseded instead of emitting false feedback, exposing an
  invariant gap rather than a production defect. The final model separately
  requires every terminal candidate to settle every required output, so both
  false success and premature failure are rejected.
- Mirror completion evidence now distinguishes shared logical identity from
  optional physical pixels. A head reports scene generation and logical-content
  checksum; a native head-pixel checksum, when available, may differ. Unrelated
  outputs may legitimately show identical content, so checksum inequality is no
  longer treated as output identity.
- The reviewed architecture now states the current-to-target migrations rather
  than assuming them complete: raw connector/CRTC identities leave Engine,
  configured mirror fit is normalized into an Engine transform, startup obeys
  prepare-all, and final target/lease ownership remains permanently head-local.
  The appended review was folded into the normative sections instead of retained
  as a competing non-normative appendix.

<!-- END IMPORTED BODY -->
