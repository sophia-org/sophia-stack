---
id: legacy-active-0068
date: 2026-08-11
recorded_date: 2026-08-11
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "tooling"]
---
# 2026-08-11: The configured two-output desktop is valid to the kernel

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2054–2081. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- First hardware answer about a Sophia-composed topology rather than a hand-built
  one. `tools/native_topology_validate.sh` runs startup's own chain — capabilities,
  topology projection, candidate reconciliation, plan preparation, head resolution,
  activation phase machine — and submits the result as one `TEST_ONLY` request:
  `validation=accepted settlement=not_applied outputs=2 heads=2`.
- What that does and does not establish. It establishes that the configured
  candidate on this host resolves into KMS objects the kernel accepts as one atomic
  desktop, which is the question head resolution existed to raise. It does not
  establish that applying it works: no framebuffer is named, nothing was mutated,
  and apply remains absent rather than gated.
- The proof runs read-only by construction, not by care. `NativeOutputTopologyValidationExecutor`
  has no apply path at all, and bringing the live scanout up performs no modeset. A
  validation that could accidentally mutate would need a reviewer to check; one that
  cannot express mutation needs only its type read.
- An accepted validation still settles as `Rejected`, and that is not a defect. The
  reducer proceeds from a passing test into apply, apply refuses, and the settlement
  records the refusal. The trap is that apply refuses with `WouldBlock`, which is
  also what a busy device reports, so the settlement cannot distinguish "the kernel
  accepted your desktop" from "the card was busy". The executor therefore retains
  the kernel's own answer and it is logged separately. A reduced log line that
  cannot separate the interesting outcome from the boring one is not evidence.
- Multi-device topologies are declined rather than validated. One atomic request
  reaches one DRM device, so a desktop spanning two cards cannot be judged as a
  unit; validating one card's fragment and reporting it as the answer would be worse
  than reporting nothing.

<!-- END IMPORTED BODY -->
