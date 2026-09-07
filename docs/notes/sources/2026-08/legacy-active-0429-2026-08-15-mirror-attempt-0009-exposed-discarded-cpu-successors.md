---
id: legacy-active-0429
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: mirror attempt 0009 exposed discarded CPU successors

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12970–12996. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Diagnostic attempt `0009` reached both physical heads at their native modes
  and displayed the centered blue focus frame, but the xterm content remained
  black. The X11 path had produced forty CPU updates and a new scene checksum;
  native scanout presented only the older black CPU generation followed by a
  retained-mixed chrome generation. While any mirror generation was active,
  `queue_frame` reported GPU ownership and discarded each newer composed CPU
  frame, and the caller had no later owner from which to recover those pixels.
- A mirror output now owns one immutable active generation and one output-scoped,
  latest-wins successor containing every head's projected frame and damage
  snapshot. Queueing prepares all physical heads before changing state, replaces
  the successor atomically, and promotes it only after the active generation's
  joined retirement. Repeated driving of one mixed Present reuses its reserved
  frame identity. A cleanup-blocked or fast-flipping head therefore cannot drop,
  relabel, or independently advance the successor.
- CPU updates that arrive with a focus/order/chrome change bypass the stale
  retained-CPU shortcut and compose the current pixels. Joined presentation now
  records the logical content source and checksum after confirming every head
  agrees. Physical promotion requires a causally retired CPU generation whose
  checksum equals the final composed CPU checksum and both completion-head
  checksums; a blue retained-only frame can no longer certify terminal pixels.
- Reducer and visual-ordering regressions, the complete backend all-feature
  suite, the CLI binary suite, and the physical-verifier mutation suite pass.
  A clean signed successor must rerun the physical gate; this implementation is
  not itself physical acceptance evidence.

<!-- END IMPORTED BODY -->
