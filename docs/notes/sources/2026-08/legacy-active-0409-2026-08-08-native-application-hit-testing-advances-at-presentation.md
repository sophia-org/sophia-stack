---
id: legacy-active-0409
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-08-08: native application hit-testing advances at presentation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12476–12498. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Production pointer selection previously rebuilt its `LayerSnapshot` list as
  soon as Engine transactions committed, even when the corresponding native
  frame was pending or submitted. A newly exposed or moved surface could
  therefore become a route candidate before those pixels reached scanout.
- Native scanout already carries an immutable `OutputFrameDamageSnapshot`
  through pending, rendering, submitted, and presented states. The visual
  runtime now publishes application input layers from that snapshot only after
  an accepted page flip moves it to presented state. Initial modesets publish
  synchronously; suspend or revocation clears the visible input projection.
  Non-native/headless output ticks retain their immediate commit-and-present
  behavior.
- The owner loop services page-flip retirement before draining physical input,
  so the new snapshot becomes visible to routing at the same owner boundary.
  Focused regressions retain the retired geometry/generation/order and exclude
  a metadata-known but unpresented surface.
- This closes the committed-versus-presented selection hole only for Sophia's
  current primary-output pointer coordinate domain. Per-output pointer domains,
  Engine-visible application grab leases, client queue failure isolation, XID
  generation advancement, and stale focus-handoff revalidation remain release
  blockers before native shell coexistence.

<!-- END IMPORTED BODY -->
