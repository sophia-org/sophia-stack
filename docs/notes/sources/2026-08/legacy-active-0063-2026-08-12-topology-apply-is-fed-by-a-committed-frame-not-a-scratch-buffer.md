---
id: legacy-active-0063
date: 2026-08-12
recorded_date: 2026-08-12
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "tooling"]
---
# 2026-08-12: Topology apply is fed by a committed frame, not a scratch buffer

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1876–1907. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The apply path has been blocked on "output-scoped framebuffer allocation" since
  the reference host refused it. Reading the architecture before building the
  unblock showed the obvious version of it is forbidden.
  `docs/renderer-import-boundary.md` states that native KMS initialization waits for
  the first committed-state frame rather than requiring a speculative or blank
  visual bootstrap. Allocating a scratch buffer at the new mode's size so apply can
  run on demand is exactly that bootstrap, and it would put a frame on screen that
  no committed state produced.
- The correct sequence is renderer-led: resize the frame target to the new mode,
  compose one frame at that size from committed state, then apply the topology
  naming that frame. `LiveGbmEglFrameTargetRecord` and its
  created/retained/resized/invalidated/retired lifecycle already model the resize;
  nothing tied it to activation.
- `native_output_apply_admission` is that tie. It is a pure reducer over the plan
  and a slice of reduced frame targets, returning ready or one precise cause with
  the output and both sizes. Apply consults it before composing heads, so a mode
  change now refuses with "output 2 has a 2560x1440 frame but the requested mode is
  1920x1080" instead of discovering a missing framebuffer mid-resolution and
  reporting it as a property of the hardware.
- One fidelity mistake caught on the way. The first wiring derived the frame size
  from the output's configured size rather than from what the CRTC is actually
  scanning out. Those are equal except during a mode change, which is the only time
  the question is asked, so the check would have passed exactly when it mattered
  least. Both the precondition and head composition now read the live framebuffer
  through one function, `read_native_current_framebuffer`, because two readers of
  "currently displayed" is one more than the number of answers that can be right.
- The roadmap's three-slot recycling pool is not a prerequisite. It is gated on
  measured software-fallback parity in Milestone 14, and treating it as the unblock
  would have built a pool to answer a question nobody had measured.

<!-- END IMPORTED BODY -->
