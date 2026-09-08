---
id: uffn76nu
date: 2026-09-07
kind: investigation
status: awaiting-physical-acceptance
tags: [investigation, x11, rendering]
---
# GLX pixmap exports need coherent backing and reply ordering

## Trigger and scope

After the DRI3 zero-size repair, accelerated Brave video still failed during
EGL pixmap binding. The [buffer-import investigation](uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md)
contains the browser captures and device evidence. This note owns the subsequent
GLX configuration, CPU export and lifetime work under the same t068 acceptance.
The candidate is based on `559b3907e5e2`; no candidate installation is implied.

## Renderer and client checks

The candidate adds a bounded renderer worker for persistent pixmap exports.
Its private GL consumer reimports neither storage nor image between writes.
The first test left the texture bound while changing its pixmap and observed a
stale second read. That test asked for undefined behavior:
[GLX_EXT_texture_from_pixmap](https://registry.khronos.org/OpenGL/extensions/EXT/GLX_EXT_texture_from_pixmap.txt)
requires synchronization and rebinding after producer writes. With the required
rebind, the test passes for XRGB8888 and ARGB8888, partial writes, duplicate
revision suppression and reads after provider release. The startup capability
probe now includes a partial write as well as a full write. These tests do not
establish synchronization merely from DMA-BUF CPU-access ioctls.

The allocation worker also has deterministic cancellation tests. Sending a
result to a buffered channel does not prove that its caller adopted it: the
caller can time out before dropping the receiver. Atomic adoption/cancellation
settles that race; unclaimed allocations are released. Six private-handshake
tests pass, including mutations of adoption, cancellation and deadline handling.

A separate test starts a private X frontend with the actual live provider and
runs `tools/probes/glx_pixmap.c`. It caught two gaps before reaching pixel reads.
First, the proposed GLX catalog failed Mesa screen matching. The inspected Mesa
revision `fd616bab71a7b24b9b71588125fabea739f511cb` compares texture-binding
attributes exactly. Its driver reports RGB and RGBA binding, Y-inversion and
target mask 7. The proposed mask 2 and Y-inversion 0 excluded every driver
configuration. The available upstream revision was inspected; fetching the
installed-version tag failed, so it is not described as matching source.
Second, the catalog's new configurations were still excluded by hardcoded
configuration-ID ranges in context and window dispatch. Catalog-based lookup
allows the test to initialize and create a direct context. The integrated gate
now passes exact initial, partial-update and retained pixels for all four
depth/target combinations.

The GLX probe uses a non-power-of-two pixmap width. Modern direct GL supports
that ordinary 2D texture shape; rejecting it would also reject common video
sizes such as 640 by 360. The renderer and frontend ownership contract is
[documented separately](../../pixmap-texture-exports.md).

## Integration findings

Socket tests use a persistent memfd provider and inspect the descriptors returned
through SCM_RIGHTS. They prove initial pixels, later partial writes before a
synchronization reply without another export, final-reference release and
retry of refused cleanup after disconnect. A blocked provider allows a second
connection in an independent namespace to receive replies, testing the lock
boundary rather than inferring it from source.

The pixel tests also exposed a producer defect: core rectangle fills on pixmaps
returned accepted without drawing. Existing software fill, outline and line
operations now run for pixmaps. Depth-32 operations also need all 32 planes;
the old raster helper masked every operation to 24 bits. GC depth now constrains
the effective plane mask, preserving opaque-drawable behavior while allowing
alpha writes. These changes precede upload; publication cannot repair absent
source pixels.

A separate diagnostic requesting a 1D texture makes the local Mesa 26.1.8 client
report target zero. The inspected upstream direct-GLX target decoder recognizes
2D and rectangle names only, although the driver's configuration reports mask 7.
The normal private pixel gate covers 2D and rectangle targets for both depths;
`tools/probes/glx_pixmap.c --texture-1d` retains the separate reproducer. This is
not evidence of successful 1D sampling and is not used to qualify the candidate.

SHM-backed pixmaps are refused before export reservation, including retained
and GLX aliases. Their external writes cannot be synchronized by the current
damage tracker. Publication state tests also bound allocation ownership while
cleanup is queued or in flight; dequeueing a failed release does not free capacity.

## Acceptance

The [export contract](../../pixmap-texture-exports.md) specifies ownership,
finite publication obligations and cleanup. `cargo xtask check` passes on the
final candidate: 2,723 tests, workspace clippy, the unchanged layout-debt ledger,
promoted archive verification and the buffer-age hardware proof. The five
socket tests pass. Bypassing the publication call makes the initial-pixel socket
regression fail; restoring it returns the test to green.

The final private GLX run on `/dev/dri/renderD128` passes initial, partial-update
and retained pixels for depths 24 and 32 with both 2D and rectangle targets.
Diagnostic logs and a manifest of changed source hashes against `559b3907e5e2`
are retained under
`~/.local/state/sophia/development-evidence/t068-gbm-import-20260907/attempt-004/`.
The mutation log records an expected failure, not a failed final candidate.

Installed acceptance requires a normal Brave launch without `--use-angle` or
`--render-node-override`, visible video, and evidence that the GPU process
remained accelerated. No installation or restart was performed. No source test
closes that physical gate or t068 in [todo.md](../../../todo.md).
