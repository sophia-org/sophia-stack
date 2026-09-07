---
id: legacy-active-0062
date: 2026-08-12
recorded_date: 2026-08-12
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "tooling"]
---
# 2026-08-12: Mirror groups are configurable before they are drivable

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1850–1875. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The configuration half of mirroring landed ahead of the scanout half, which
  raises the question of what a `mirror` directive should do while nothing can
  drive it. It is validated fully and then refused. Accepting and ignoring it would
  leave an operator staring at an unmirrored screen with no error to search for,
  which is the worst of the three options; refusing without validating would hide
  configuration mistakes until the day the feature arrives.
- Validation is split by what each layer can answer, which kept each rule where its
  inputs are. Parsing rejects self-reference, repeats, emptiness, and the bound —
  true whatever hardware is attached. The candidate as a whole rejects one connector
  claimed by two logical outputs, the only rule needing more than one output in
  view, and the only arrangement that would make "one logical output backed by N
  connectors" untrue. The topology rejects unknown, disconnected, or mode-mismatched
  members.
- Same-mode is enforced at reconcile because no plane scaling exists on this path.
  The alternative to refusing is letterboxing a screen the operator asked to match,
  and silently changing what a display shows is not a fallback.
- Ordering the errors mattered more than expected. An impossible request and an
  unimplemented one send an operator to different places, so the topology checks run
  before the unsupported refusal. A single "not supported" for both would have
  buried real configuration errors until the feature shipped.
- The mode is resolved through the primary's own `resolve_mode` rather than a second
  copy. Two answers to "which mode did the operator ask for" is one too many, and
  the copy would have drifted the first time preferred-mode handling changed.

<!-- END IMPORTED BODY -->
