---
id: legacy-active-0246
date: 2026-07-15
recorded_date: 2026-07-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "tooling"]
---
# 2026-07-15: Milestone 4 Hardware Checkpoint And AMDGPU Mixed-Draw Blocker

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8204–8243. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The paired X13 gate now proves the software half after a real renderer defect
was isolated. Reusing mixed-composition GL state for the legacy full-screen CPU
upload eventually lost the AMD context. The persistent upload path now restores
its fixed full-screen quad and completes independently; the retained software
run committed the 800x600 configure-plus-pixels resize, flushed all 14 semantic
input events, reported exact text and changed pixels, submitted 36 native
frames, retired 35, and drained with no failure or cleanup debt.

The Vulkan attempt also exposed a transaction-domain mismatch. Present request
generations continue across controlled Skip, while Engine visual generations
advance only on accepted commits. Full-state Present snapshots are now rebased
to the current Engine committed generation immediately before preparation;
external regressions cover the empty baseline and a later post-Skip baseline.
This removed the stale-candidate flood and allowed the real imported image to
reach the renderer.

The remaining failure is specifically the required two-layer native EGL draw.
With the CPU background removed only for diagnosis, the same real `vkcube`
session completed 86 mixed exports and Flip completions, one controlled Skip,
87 matching Idle events and idle-fence triggers, 121 native submissions, 120
retirements, and zero live sources, fences, transactions, or KMS cleanup debt.
Restoring the CPU layer aborts Sophia inside Radeon `glFinish` with
`amdgpu: The CS has been rejected, see dmesg for more information (-2)` before
the first mixed KMS submission.

The failure survived `RADV_DEBUG=nodcc`, explicit CPU/import completion
boundaries, frame-local CPU textures, frame-local vertex buffers, and a
diagnostic layer-order reversal. Those experiments were removed; the retained
implementation keeps only the proven full-screen upload isolation, Present
generation rebase, and EGLImage sampling lifetime order. The next session
should capture the privileged kernel validator message immediately after one
failure, then reduce CPU-texture-plus-imported-image composition to a focused
native-EGL hardware regression before changing more session code. Retained
ignored evidence is under
`.evidence/remote-target/tmp/sophia-milestone4/` and
`.evidence/remote-target/tmp/sophia-milestone4-dmabuf-only/`; neither GPU log
is promotion evidence until the normal paired verifier passes.

<!-- END IMPORTED BODY -->
