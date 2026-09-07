---
id: legacy-active-0166
date: 2026-07-28
recorded_date: 2026-07-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-28: GLX Compatibility Uses the Generic Standalone Workload Slot

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5621–5672. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Sophia's bounded GLX proof now launches `glxgears` through the same standalone
application lifecycle used by the fixed Vulkan proof. Workload selection and
bounded-process policy remain operator tooling concerns: Engine receives an
ordinary X11 client and retains no `glxgears`, Kitty, xmonad, or benchmark
identity. This preserves the frontend-neutral scene and presentation model.

The probe runs at a fixed 500-by-500 geometry with swap interval one and exits
after 20 seconds. Its report fails closed unless the log identifies the OpenGL
renderer, shows client animation samples, advancing routed post-KMS Flip
timestamps, positive native and mixed exports, Present idle-fence progress,
and a clean resource drain. Client-reported FPS and Sophia's actual
presentation cadence are separate fields because they describe different
boundaries.

This proof diagnoses direct GLX bootstrap and the DRI3/Present compatibility
path. It cannot replace the deterministic Vulkan parity workload, and no
GLX-specific fast path has been added to Engine or the renderer. Physical
metrics remain pending until the dedicated-TTY run is retained.

The first physical attempt failed before creating a window:
`glxgears` could not select an RGB double-buffered visual. Kitty had exercised
the modern FBConfig/context path, while the classic catalog advertised zero
depth bits and the authority did not decode legacy visual-based context
creation. The repair makes both GLX entry paths explicit data variants,
advertises a 24-bit depth buffer in matching visuals and FBConfigs, normalizes
legacy visuals to the bounded FBConfig runtime identity, and validates direct
MakeCurrent context/drawable pairs. A bounded external-client preflight must
now reach visual discovery, direct context creation, DRI3 import, and Present
before the script takes over the TTY. The real-Mesa preflight passes those
stages without an X protocol error. Mesa keeps MakeCurrent local for this
direct-rendering workload, so that supported request is not a required wire
observation.

The first run that reached native output exposed two coupled generic
presentation-lifetime faults. The initial gears frame was imported, composed,
and retired successfully, but an unrelated software cycle had queued a stale
CPU frame while that mixed Present was in flight. It replaced the visible
gears with a blank output. Sophia then retained the completed client DMA-BUF
until the successor's KMS retirement; Mesa needed that idle fence while the
successor was being imported, and radeonsi eventually reported a guilty
context hard recovery.

Production frame reduction now preserves an already-submitted GPU Present
instead of queuing CPU fallback behind it. When an acquired mixed Present
replaces the same surface, the runtime retires the prior composited source and
signals its idle fence before importing the successor. Present Complete remains
tied to the prior KMS page flip, so protocol timing and client-buffer reuse are
separate facts. Focused regressions require both the no-superseding-frame
policy and actual xshmfence progress while the successor remains ready.

<!-- END IMPORTED BODY -->
