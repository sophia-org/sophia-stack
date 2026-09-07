---
id: legacy-milestone-0025
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: first-heading-commit
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
date_commit: 2d7fa1b57a8741816ace149d6e69671222ef45f5
committed_at: 2026-07-10T18:12:53-04:00
---
# Completed Atomic Scanout Smoke Milestones

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 562–599.
Date from the first addition of this heading in commit `2d7fa1b57a8741816ace149d6e69671222ef45f5`
(2026-07-10T18:12:53-04:00); it does not date every event or later edit.

<!-- BEGIN IMPORTED BODY -->

- [x] Added `tools/atomic_scanout_preflight.sh`,
  `tools/atomic_scanout_smoke.sh`, and the strict reduced evidence verifiers for
  preflight, destructive atomic scanout, and runtime rendered-scanout evidence.
- [x] Added `tools/atomic_scanout_hardware_proof.sh` as the combined operator
  proof for preflight, two-phase atomic scanout, and runtime submit-to-retire
  evidence.
- [x] Advanced reduced atomic scanout evidence to schema 10 and runtime
  rendered-scanout submit evidence to schema 6 with reduced scanout-buffer
  format, modifier, plane-count, format-table, framebuffer-creation, submit,
  retire, and cleanup-debt fields.
- [x] Added backend-private primary-plane `IN_FORMATS` discovery and bounded
  modifier parsing so rendered GBM/EGL scanout export can choose usable scanout
  candidates without exposing DRM property blobs or native modifier values.
- [x] Allowed explicit non-linear multi-plane XRGB8888/ARGB8888 buffers to
  reach modifier-aware AddFB2 while keeping unsupported implicit/linear
  multi-plane buffers rejected before native resource creation.
- [x] Made rendered GBM/EGL scanout skip rejected multi-plane export candidates
  and continue searching for a single-plane buffer when the driver rejects the
  modifier-aware framebuffer path.
- [x] Added backend-private PRIME import for renderer-exported DMA-BUF planes:
  backend-live imports them into the KMS submit device, builds AddFB2/AddFB from
  KMS-local handles, and closes imported GEM handles through the existing
  cleanup path.
- [x] Fixed the destructive smoke lifetime rule so the initial rendered GBM/KMS
  owner remains active until the steady page flip presents, then both resource
  bundles retire after accepted page-flip callbacks.
- [x] Captured TTY3 reduced smoke evidence where both `InitialModeset` and
  `SteadyPageFlip` pass with `framebuffer=CreatedWithAddFb2`,
  `page_flip=Presented`, `retire=RetiredAfterPageFlip`, and
  `retire_cleanup_pending=false`.
- [x] Closed the combined TTY3 hardware proof with verifier-accepted preflight,
  destructive two-phase atomic scanout evidence, and runtime rendered-scanout
  submit-to-retire evidence.

---

<!-- END IMPORTED BODY -->
