---
id: legacy-active-0167
date: 2026-07-28
recorded_date: 2026-07-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-07-28: Xserver Parity Uses Present Evidence, Not Demo Wall Time

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5673–5716. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The software-Present optimization reached an observed 59.947 FPS with a
17.564 ms p95 interval on a 459-sample physical run, but the tree retained no
same-machine Xorg or XLibre baseline. Treating an ideal 60 Hz interval as that
baseline would hide provider, output, mode, and server-side presentation
differences.

The paired reference runner now launches the same fixed 500-by-500,
900-frame, FIFO `vkcube` workload under an Xserver. A bounded XCB probe attaches
only to newly created matching top-levels and records actual Present Complete
UST/MSC values. The shared cadence reducer computes FPS and p95 for both
systems. The comparison fails closed when workload, requested frames, Vulkan
provider, Vulkan/X Present mode, or output pixel count differ, then applies the
existing 90% rate and inverse-p95 gate.

`glxgears` is retained only as an optional Xserver GLX/OpenGL reference and
gross cadence probe. Its workload and driver behavior are not representative
enough to become Sophia's rendering acceptance metric, so its record is kept
separate and cannot affect the Vulkan parity decision. A bounded Sophia-side
direct-GLX/DRI3/Present pair is tracked explicitly rather than treating the
Xserver-only number as Sophia evidence.

The first retained Xserver capture used XFCE's composited Xorg path. It
completed 898 of 898 observed frames with monotonically advancing UST and MSC,
but every completion mode was `Copy`; Sophia's corresponding completion is
post-KMS `Flip`. Rejecting all non-Flip samples incorrectly classified healthy
FIFO client cadence as missing evidence. The reference reporter now admits
advancing Pixmap `Flip`, `Copy`, and `SuboptimalCopy` completions while
recording the path. A mismatch is explicitly `cadence_only`: it can gate
throughput and cadence, but cannot establish final scanout latency. An
unredirected Xserver Flip capture remains the stronger follow-up if that claim
is required.

The paired physical Vulkan gate passes. Both fixed workloads produced 898
observed completions with the same llvmpipe provider, 500-by-500 surface,
FIFO Vulkan mode, and 2560-by-1440 target output. Sophia measured 59.953 FPS
and 17.155 ms p95; composited Xorg measured 59.950 FPS and 16.686 ms p95. The
rate ratio is 1.0001 and the inverse-p95 ratio is 0.9727, above the required
0.90 in both dimensions. Sophia's maximum CPU composition was 6 ms, maximum
native upload was 3 ms, and native submission failures remained zero. This
closes the software-Present cadence gate while preserving the separate
unredirected-Flip latency follow-up.

<!-- END IMPORTED BODY -->
