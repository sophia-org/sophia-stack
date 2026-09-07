---
id: legacy-active-0434
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: CPU composition crosses the per-head lowering seam

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13101–13125. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The production CPU adapter now carries every resident CPU content variant
  from its one committed scene read into submission. Engine selects a variant
  separately for each `HeadRenderTarget`; `sophia-renderer-live` lowers that
  exact `HeadCompositionPlan` into native placements, clips, opacity, bars,
  borders, cursor state, and a head-local damage snapshot.
- `LiveProductionNativeScanout::queue_head_composition_frames` admits the set
  transactionally by opaque head. It rejects partial or duplicate coverage,
  logical checksum disagreement, missing damage, and native target mismatch,
  then queues one `HeadComposition` frame per physical owner without applying
  the old whole-output projection. The flat CPU result is retained only for the
  synchronous first modeset; the immediately following cohort uses the new
  path.
- Variant selection and transformation now fail closed: a plan does not fall
  back from an unavailable compatible raster to an arbitrary transformed
  canonical buffer, and non-normal output transforms remain rejected until
  their geometry/raster lowering is implemented. Off-viewport surfaces are
  filtered from extended-output snapshots rather than invalidating the frame.
- This is not the final prepare-all cutover. Renderer export and KMS commit are
  still one operation in the live mirror scheduler, so the Engine cohort
  reducer is not yet its production owner. DMA-BUF and retained renderer-image
  sources also remain on their existing projected mixed path until a per-head
  affine lease resolver can prepare every required target before submission.

<!-- END IMPORTED BODY -->
