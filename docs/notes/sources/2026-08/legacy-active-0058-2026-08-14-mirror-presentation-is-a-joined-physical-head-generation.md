---
id: legacy-active-0058
date: 2026-08-14
recorded_date: 2026-08-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-14: mirror presentation is a joined physical-head generation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1758–1777. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The failed physical run disproved the primary-head shortcut. A mirror output now
  owns one scanout lane per connector: exporter, displayed and submitted buffers,
  cleanup, callback serial, and timing. The Engine advances once, physical heads
  submit independently at their native modes, and logical presentation becomes
  visible only after the connector set joins on one frame identity.
- The group is bounded to one active generation plus the exporters' newest pending
  frame. A partially submitted generation is poisoned: accepted KMS owners drain,
  no replacement generation is admitted, and no logical presentation is emitted.
  Sequential per-head commits remain deliberate; they follow the X-style decision
  recorded in `todo.md` and avoid unproven multi-CRTC event semantics.
- CPU mirror layers now share immutable source bytes. Retained CPU, DMA-BUF,
  renderer-image, solid, and cursor geometry is projected per head. Renderer-image
  suspend handoff also covers each physical exporter instead of only the primary.
- Mirror bootstrap uses head-sized CPU buffers before renderer workers start. This
  removes the mirror-specific inline-EGL-to-worker transition from the failed run.
  It is a code-side crash control, not proof that the AMDGPU rejection is fixed;
  that conclusion still requires a diagnostic-capable physical rerun.

<!-- END IMPORTED BODY -->
