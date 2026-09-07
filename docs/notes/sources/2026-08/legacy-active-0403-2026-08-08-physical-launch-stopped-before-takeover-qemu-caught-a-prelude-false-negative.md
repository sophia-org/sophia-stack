---
id: legacy-active-0403
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-08-08: Physical launch stopped before takeover; QEMU caught a prelude false negative

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12262–12285. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first installed practical-profile launch ended in a host reboot while
  the independent input guard was still starting. Its immutable run recorded
  `preflight` and `input_guard=ready`, but never recorded `input_guard=armed`,
  graphics takeover, a session loop, or recovery. This proves Sophia had not
  reached the compositor when the host disappeared; the kernel reboot cause
  remains unclassified because this user session cannot read the previous
  boot's root-only kernel log.
- Reproducing the same startup and practical profile in the isolated QEMU
  harness reached focus, layout, workspace, pointer, launch, restart/reseed,
  and clean logout. The run ended with zero protocol or renderer health debt,
  38 accepted page-flip callbacks, and `sophia_qemu_guest status=complete`.
- That run exposed a harness-only false negative: the generic xmonad prelude
  waited for one visible surface even though the M7 scenario intentionally
  admitted two. The prelude now derives the expected projection cardinality
  from the scenario (two for M7, one for M8) and reuses the same predicate for
  focus and layout. The retained M7 acceptance and verifier regression both
  pass after the correction.
- QEMU remains evidence for protocol and policy semantics only. It does not
  close the physical installed short gate, which still requires a successful
  input-guard arm, DRM/VT takeover, visible pixels, and normal teardown on the
  real host.

<!-- END IMPORTED BODY -->
