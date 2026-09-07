---
id: legacy-active-0437
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: live mirror submission now has a real prepare-all barrier

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13172–13193. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Native primary-plane setup is split into two affine operations. Preparation
  exports the renderer result, creates/imports the framebuffer and optional mode
  blob, discovers properties, and builds the complete atomic request without
  calling the commit ioctl. Its owner can only be submitted or explicitly
  cancelled against the same DRM device.
- The production multi-head scheduler records one `HeadFrameCandidate` per
  completed native owner in Engine's `OutputPresentationCohort`. A head may not
  submit until the cohort has every required head, so neither a fast renderer nor
  loop ordering can cross the first-submit barrier. Submission, callback, and
  cleanup transitions are reflected back into that same cohort while the older
  physical lifecycle continues to provide card polling and timing evidence.
- A preparation failure cancels every sibling owner before poisoning the logical
  generation. Normal shutdown cancels prepared-but-unsubmitted owners before its
  callback-only drain. Deterministic fake-DRM coverage proves preparation issues
  zero commits, cancellation frees its framebuffer, and two prepared mirror
  heads remain at zero commits until the complete cohort is ready.
- This does not yet implement live topology replacement. Candidate mode objects,
  replacement target pools, KMS modeset apply/rollback, runtime rebuild, and the
  first-presentation publication barrier remain the next effect transaction.

<!-- END IMPORTED BODY -->
