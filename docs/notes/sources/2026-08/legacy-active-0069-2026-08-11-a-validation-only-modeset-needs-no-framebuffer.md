---
id: legacy-active-0069
date: 2026-08-11
recorded_date: 2026-08-11
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-11: A validation-only modeset needs no framebuffer

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2082–2119. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The open question was whether a `TEST_ONLY` modeset requires plane state and a
  valid `FB_ID`. It decided the shape of head resolution: if validation needs a
  framebuffer, then checking a new mode means first allocating one at that mode's
  size, and allocation stops being a consequence of a validated topology and
  becomes a precondition for validating one. It is driver-dependent and cannot be
  settled offline, which is why `native-topology-probe` exists.
- **Answer: no.** On the AMD two-output reference, from a bare TTY holding DRM
  master, a modeset naming only the connector's `CRTC_ID` and the CRTC's `MODE_ID`
  and `ACTIVE` validates. Adding primary-plane state and reusing the framebuffer
  already being scanned out also validates. Validation precedes allocation, so head
  resolution may size and allocate framebuffers after the kernel has agreed to the
  topology.
- The first run said the opposite, and the difference was ours. Both probes came
  back `EINVAL` with mode and framebuffer sizes in agreement, which ruled out the
  plane-sizing explanation and pointed at the request. The kernel rejects
  `TEST_ONLY` together with `PAGE_FLIP_EVENT` outright, before inspecting any
  property, and `LibdrmNativeAtomicCommitRequest` defaulted `page_flip_event` true
  while `test_only()` left it alone. Every validation-only commit in the tree was
  returning `EINVAL` regardless of content.
- The blast radius was larger than the probe. `submit_native_multi_head_topology`'s
  `Validate` intent carried the same flags, so `NativeOutputCommitExecutor::validating`
  would have declined every topology it was ever handed, and the declined-test path
  at startup would have read as hardware refusing a valid desktop. The bug would
  have been discovered as a mysterious hardware incompatibility rather than as a
  flag error.
- The fix derives the flag instead of storing it: a request that is `test_only`
  reports and sends no page-flip event, so the illegal combination cannot be
  constructed in any call order. A validation-only commit never reaches scanout and
  so has no completion to signal; the kernel's rule and the semantics agree.
- Two general lessons, both cheap to state and expensive to relearn. A probe that
  records only `Accepted`/`Rejected` cannot distinguish a hardware answer from a
  malformed question, so a refusal must carry its errno and the operands that
  explain it. And a negative hardware result should be suspected of being a defect
  in the asker until its errno says otherwise — the first run's conclusion was
  written into `todo.md` as two plausible hardware hypotheses, and neither was true.

<!-- END IMPORTED BODY -->
