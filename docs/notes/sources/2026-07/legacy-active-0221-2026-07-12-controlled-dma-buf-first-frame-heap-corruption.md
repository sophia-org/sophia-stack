---
id: legacy-active-0221
date: 2026-07-12
recorded_date: 2026-07-12
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-12: Controlled DMA-BUF First-Frame Heap Corruption

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7384–7435. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first real controlled DMA-BUF run reached Sophia's full-size 1920x1200
client frame and recorded `sophia_wayland_frame` with `buffer=dmabuf`. The
native process then aborted with `corrupted size vs. prev_size`, disconnecting
the producer before presentation retirement or buffer release could be proven.
This is a renderer/resource-ownership safety failure, not DMA-BUF evidence.

The producer itself now follows the compositor's initial xdg configure rather
than assuming 640x480, and uses a driver-supported explicit linear GBM
allocation. Those corrections moved the test past allocation and target-size
rejection; they did not make the native import/presentation path safe. The
300-frame lifecycle and three-Kitty promotion gates remain blocked pending
allocator/lifetime diagnosis.

The next controlled rerun uses a GDB-backed diagnostic mode with explicit
DMA-BUF stages. The importer now detaches the EGLImage from its GL texture and
finishes that detach before destroying the EGLImage and dropping the imported
client FD. This makes input-image teardown independent from the retained GBM
front-buffer owner.

The GDB-backed three-frame rerun passed: each 1920x1200 frame completed EGL
image creation, rendering, texture detach, image destruction, KMS submission,
page-flip observation, scanout retirement, and client buffer release. The
session exited normally with three imports, three retirements, three callbacks,
no cleanup debt, and a 14 ms maximum submit-to-page-flip interval. The
GDB-backed 300-frame lifetime proof then completed with 300 imports,
submissions, page flips, and retirements, no allocator diagnostic or cleanup
debt, and the same 14 ms maximum submit-to-page-flip interval. A subsequent
normal release 300-frame run nevertheless aborted with `corrupted size vs.
prev_size` after frame 8 (an earlier normal run reached frame 13). This makes
the fault timing-sensitive: the GDB result is diagnostic evidence, not a
completed lifecycle gate. A release-timing trace then completed all 300 frames
with ordered ownership stages and an 18 ms maximum submit-to-page-flip interval.
One uninstrumented rerun and then three separately retained uninstrumented
300-frame runs all completed normally: each reported 300 imports, 300 callbacks
and retirements, no cleanup debt, no surviving process, and a 14 ms maximum
submit-to-page-flip interval. The next full promotion preflight nevertheless
aborted on its first uninstrumented DMA-BUF frame with `free(): invalid pointer`,
before Kitty started. A later post-repair normal run also aborted after frame 2.
The persistent CPU-upload texture was therefore isolated from imported images:
each import now gets a transient per-frame texture, which is deleted after
`glFinish` before EGLImage destruction. The repaired three-frame proof passed
with three imports, three retirements, and a 16 ms maximum interval. A normal
core-capture 300-frame run and three separate uninstrumented normal 300-frame
runs then all completed: every run had 300 imports, callbacks, and retirements,
no cleanup debt, no surviving process, and 14–16 ms maximum latency. This meets
the bounded controlled gate, while retaining the normal-stability wrapper as a
regression guard for the earlier intermittent abort. The next required evidence
is three guarded native-SHM Kitty runs; a later GPU-composition milestone must
precede any real-Kitty DMA-BUF runs.

<!-- END IMPORTED BODY -->
