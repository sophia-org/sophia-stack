---
id: legacy-roadmap-0021
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Status, Launcher, And Shell Integration

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 2564–2592.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0017-post-promotion-capability-roadmap.md).

<!-- BEGIN IMPORTED BODY -->

- [ ] Define a bounded redacted status feed for workspace number, approved
  layout name, focus state, output health, and supervised-component health.
  Workspace number, layout name, and focus state are settled by the indicator
  descriptor and arrive on the layout commit, not through a broker: policy owns
  them and no broker has an upstream source. Output and supervised-component
  health remain session-owned and still need a path. See
  `docs/sophia-indicator-descriptor.md`.
- [x] Render tier-0 indicator chrome in Engine from the committed descriptor,
  reusing the existing `capability "chrome"` path and the renderer-neutral
  display list. The private semantic strip lowers through ordinary CPU layers,
  uses one bundled font, reserves 14 logical pixels before the first public WM
  snapshot, and publishes exact last-presented hit targets. The existing
  `tools/hagia-proof` one-shot now requires two pointer activations and their
  committed policy actions. Signed archive `0005` verifies both activations,
  all fourteen ordered action commits, nonzero presentation on both outputs,
  exact physical text, and clean teardown.
- [x] Emit indicators from Hagia's private tags, keeping tags private and
  crossing only labels, state bits, and action tokens. Hagia's independent Nim
  codec and the cross-repository conformance gate cover the records.
- [ ] Register a new bounded opaque launcher action and decide whether the
  compatibility UI is dmenu or native Engine/shell chrome. Do not reuse the
  established xmonad layout-action IDs.
- [ ] Implement lock, screenshot, wallpaper, and audio actions through their
  owning shell or portal boundaries.
- [ ] Admit tray/XEmbed only from a retained application workflow and keep it
  outside blind WM policy.

<!-- END IMPORTED BODY -->
