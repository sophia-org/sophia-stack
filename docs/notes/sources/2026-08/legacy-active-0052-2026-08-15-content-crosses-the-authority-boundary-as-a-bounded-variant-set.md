---
id: legacy-active-0052
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security", "architecture"]
---
# 2026-08-15: content crosses the authority boundary as a bounded variant set

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1615–1642. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- `SurfaceTransaction` no longer carries a bare `target_buffer` and
  `target_content_size`: it carries a `SurfaceContentSet` — a bounded,
  density-deduplicated list of `SurfaceContentVariant` records with its own
  named capacity (`MAX_SURFACE_CONTENT_VARIANTS`). Every current producer
  normalizes into a one-variant set at identity density; the committed state
  clones the whole set so no later stage can silently squash
  authority-asserted variants to the canonical raster.
- Set invariants are enforced at construction with private fields, so an
  empty, over-capacity, or duplicate-identity set is unrepresentable rather
  than validated downstream. Envelope identity is structural too: the set
  lives inside its transaction or committed state, so a variant cannot name a
  different surface or generation than its envelope. This replaces the
  reviewed plan's duplicated identity fields plus intake validation.
- Deliberately deferred until their consumers exist, per the
  production-reachability rule: per-variant damage and readiness (their
  consumer is the per-head damage ledger), a source-transform class (no
  producer can emit one), and a wire encoding for multi-variant sets — the
  authority socket still encodes the canonical extent and source, so the wire
  format is byte-identical and the frontend needs no version bump.
- Content generation is not a second clock: a new set is a new authority
  transaction, so `CommittedSurfaceState.committed_generation` is the set's
  generation identity.
- DMA-BUF Present pairing now ranges over every variant of a set: a DMA-BUF
  variant anywhere requires its exact Present pair even when the canonical
  variant is a CPU raster.

<!-- END IMPORTED BODY -->
