---
id: legacy-active-0056
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: multi-monitor composition fans out before rasterization

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1710–1741. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The mismatched-mode mirror proof established native scanout ownership and
  joined retirement but also made the remaining visual shortcut obvious. The
  production CPU path composes a 2560x1440 logical frame first and reduces that
  flat image for the 1920x1080 head. A sharper filter can preserve more contrast,
  but it cannot turn an output-level raster into native per-head composition.
- The ratified target is one immutable logical Engine scene with a distinct
  native composition for every physical head. Mirrored heads retain the same
  scene generation, surface set, geometry, stacking, focus, and logical
  retirement identity; their native targets, damage ledgers, selected client
  variants, framebuffers, KMS owners, and pixel results are intentionally
  distinct. Extended outputs use the same path and no longer inherit visual
  work from a distinguished primary output.
- The client-content limit is explicit. Engine-owned primitives can be
  rasterized at each head's density, but an arbitrary client buffer cannot be
  semantically recreated at another resolution. Protocol authorities may emit
  a bounded immutable set of raster variants for one surface generation.
  Engine selects the best ready variant per head; a singleton client buffer
  remains admissible through an observable resampling fallback. Engine never
  acquires permission to replay X11 drawing requests.
- Ownership and retirement do not change. The WM still sees one `OutputId` for
  a mirror group and no connector identity. Rendering prepares every required
  mirror head before the first per-head KMS submit, callbacks join on the
  logical scene generation, and head loss fails the candidate closed. The
  existing `VisualRetirement.tla` head layer was deliberately written over
  heads rather than shared buffers; it remains the base and must gain the
  prepare-all invariant before scheduler implementation.
- [Multi-Monitor Per-Head Composition](../../../multi-monitor-composition.md) is the
  normative target. The existing flat-frame projection is retained as current
  implementation evidence, not described as compliant per-head rasterization.

<!-- END IMPORTED BODY -->
