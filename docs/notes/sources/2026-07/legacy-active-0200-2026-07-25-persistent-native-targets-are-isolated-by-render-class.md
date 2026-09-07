---
id: legacy-active-0200
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-25: Persistent Native Targets Are Isolated By Render Class

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6839–6887. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The latest physical four-Kitty evidence showed that a stable output epoch
created a GBM/EGL target and GL pipeline for every mixed frame. That avoided an
older AMDGPU command-stream rejection by destroying mixed DMA-BUF state before
the next CPU upload, but replaced the hazard with frame-rate resource churn,
185 ms page-flip latency, and 291 ms input queue dwell.

Native scanout now keeps separate persistent CPU, DMA-BUF, and composition
targets for each output context. Render classes cannot leak EGL/GL state into
one another, while each class reuses its target across a stable
size/format/modifier epoch. An exported scanout buffer retains a reference to
the persistent native surface, so buffer release still precedes surface
destruction without forcing the context or pipeline to be rebuilt per frame.
An epoch change retires the affected target; a bounded retry may replace a
target after an explicitly classified EGL, GL, upload, or composition failure.

Reduced completion evidence reports total and per-class target creation plus
epoch- and recovery-driven replacement counts. The four-Kitty verifier now
requires sustained mixed composition, class-consistent creation counts, zero
replacement in the stable workload, zero launch-admission timeout, and bounded
input, upload, and page-flip latency. Local compilation and verifier mutation
coverage establish the ownership model; the physical workload and recovery
retirement proof remain open.

The first physical run of that implementation crashed on its third mixed frame
with an AMDGPU command-stream rejection. A bounded two-target experiment then
crashed at the same point: target A rendered frame one, target B rendered frame
two, and the driver rejected target A's first reuse after its exported KMS
lease had retired. This disproves both cross-class contamination and an
in-flight front-buffer lease as the complete cause. The retained Mesa/AMDGPU
path cannot safely reuse a composition EGL context after the current DMA-BUF
import sequence.

Mixed composition has therefore returned to the previously proven fail-safe:
destroy its context, pipeline, and target after every exported frame. CPU and
direct DMA-BUF targets remain class-isolated and persistent. Reduced evidence
requires composition creation and retirement counts to match mixed exports,
while epoch and recovery replacement of the other classes remains zero. This
restores startup correctness without falsely closing the lifetime milestone.
The next optimization must change the import/synchronization architecture, not
extend an unsafe context pool.

The same milestone now names click-to-focus separately from pointer motion and
client click-drag delivery. A primary click on an unfocused visible surface
must first select that surface through the blind WM interface, then deliver
ordered client input to the newly focused target. This is pending work; no
xmonad- or application-specific policy belongs in Engine.

<!-- END IMPORTED BODY -->
