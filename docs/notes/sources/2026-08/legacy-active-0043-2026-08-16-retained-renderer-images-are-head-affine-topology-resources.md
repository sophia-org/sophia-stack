---
id: legacy-active-0043
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "tooling"]
---
# 2026-08-16: retained renderer images are head-affine topology resources

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1374–1398. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next three-head run crossed policy admission, output-owner
  authorization, quiescence, and candidate-frame lowering, then failed before
  KMS with head 3's worker reporting `InvalidTarget`. The candidate frame had a
  valid 1920x1080 native target; its Kitty layers named renderer-image IDs that
  existed only in head 1's persistent renderer store. A retained image ID is a
  semantic identity, not proof that every physical renderer already owns its
  pixels. Head 3 had previously displayed an empty output and therefore could
  not resolve those IDs when it joined head 1's mirror group.
- Candidate and rollback frame admission now derives a unique retained-image
  requirement set per opaque physical head. While frame ownership is quiescent
  and before any candidate worker submission, native topology preparation
  accepts an already-promoted local copy, promotes a staged local copy, or
  exports a compositor-owned DMA-BUF snapshot from another live head and
  restores it into the target head's renderer store. Missing donors and failed
  restoration reject preparation with zero KMS submits. The scene remains an
  Engine plan; this is renderer-resource realization, not policy geometry or a
  cross-head scanout-buffer lease.
- The failed worker took several owner turns to drain after abort and the owner
  logged the unchanged `Aborting` report on every turn, producing a 1.2 MiB
  incident log. Preparation progress telemetry is now emitted only when phase
  or prepared counts change. Worker polling and affine cleanup remain
  nonblocking and unchanged.

<!-- END IMPORTED BODY -->
