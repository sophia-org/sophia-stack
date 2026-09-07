---
id: legacy-active-0444
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-16: mirror attempt 0019 promotes PutImage replay to the critical path

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13305–13332. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Signed source `a5d916279c9fb8cd03415945d0dfeb11515c1a32` completed the
  unequal-mode mirror runtime without a renderer, KMS, protocol, or ownership
  crash. Both heads produced native plans, submissions, callbacks, and
  retirements; native suspend drained with zero abandoned scanouts, renderer
  worker failures were zero, and final session health was clean. The attempt
  failed at visual confirmation, so it remains diagnostic rather than promotion
  evidence.
- The density contract did not pass. The log contains 76
  `sophia_x11_raster_requirement ... status=sampled_fallback` observations. The
  750-density head repeatedly selected the canonical 1000-density handle and
  produced no exact 750-density selection. This explains the visibly softer
  smaller-head text without contradicting native-size final composition: the
  head target was native, but its client content variant was sampled.
- Code inspection found that an accepted X11 `PutImage` marks the surface's
  semantic journal unsupported. A bounded local real-xterm trace confirmed
  opcode 72 during startup, before later ImageText8 and line drawing. Once that
  operation is accepted, late density demand can only publish the canonical
  raster, so the current gate cannot produce an exact derived 750-density
  store.
- Decision: implement bounded `PutImage` ownership and replay inside X
  Authority, treating a complete opaque replacement as a possible journal
  baseline only when canonical semantics are preserved. Cross-drawable
  `CopyArea` follows with explicit source-generation dependencies. Do not make
  Engine interpret X drawing, weaken the exact-density verifier, or label a
  resampled canonical raster as independently rendered content.

<!-- END IMPORTED BODY -->
