---
id: j11gjkh1
date: 2026-09-08
kind: milestone
status: recorded
tags: [milestone, rendering, x11]
---
# Generic cross-device image capture and worker recovery

## Result

This implementation checkpoint extends [t069](../plans/6tewvlbh-universal-device-negotiation-across-sophia-clients.md)
with renderer-owned transfer of submitted XR24/AR24 images. It follows the
[measured DRI3 checkpoint](f24uuvwu-measured-dri3-import-capabilities-and-exact-legacy-exports.md).
It uses no application registry, browser switch or Engine adapter.

Backend inventory admits at most sixteen initialized devices belonging to the
actual seat, including headless GPUs. Open descriptors are checked against node
and physical-device identity. Native workers receive owned FDs, initialize
auxiliary contexts lazily, and discard them with the worker incarnation.
An actual direct-import refusal can cause another admitted GPU to sample the
image into an explicitly linear bridge. The destination captures that bridge
into its own immutable retained image. No source-owned BO enters the retained
store. Both copies import actual descriptors and use DMA-BUF implicit fencing;
no CPU readback or glFinish is inserted between the two GPU accesses.

A 64-entry set of previously successful transfer layouts changes attempt order,
not authorization. Every new image imports its actual FDs, while an unchanged
immutable image ID is reused without copying. Existing staged, promoted,
rollback, budget and retirement rules remain authoritative. Byte accounting
includes the temporary bridge during capture; it measures pitch times height,
not every internal driver allocation.

Two prerequisite lifecycle repairs are included: retained image buffers are
dropped before their EGL display, and shutdown delivery cannot be lost when a
renderer worker's command queue is full. Kernel revocation and processed udev
admission events feed the existing full native-owner reconstruction path.
Monitoring starts before initial inventory discovery, closing the subscription
gap. Uninitialized udev records never acquire the default seat implicitly.

## Evidence

The implementation is based on `e3c39890`, with the first capability checkpoint
staged separately. Signed publication is waiting for the user's signing key:
GPG returned `Operation cancelled`; the staged checkpoint was preserved and
gpg-agent was not changed.

The final candidate's source, binary, exact device identities and logs are retained in
`~/.local/state/sophia/development-evidence/t069-image-transfer-20260908/attempt-001/`.
Its manifest identifies the exact source rather than treating these uncommitted
changes as HEAD. There is no signed code commit receipt yet. The archive also
preserves the index separately from the later working-tree changes; all 26
staged files match the first checkpoint's archived source hashes.

The final `cargo xtask check` passed with 2,783 tests and zero warnings, all
twenty retained archive proofs, and buffer-age pixel equivalence on this host.
It includes the six new provider identity tests. The private GLX socket test
also passed on `renderD128`, proving exact initial, dirty and retained pixmap
pixels through the real provider after its identity checks were added.

Private offscreen tests use AMD Navi 31 (`renderD128`, PCI `03:00.0`) and Raphael
(`renderD129`, PCI `16:00.0`). They first find and verify a source layout that
actually fails direct import on the destination, then prove exact transfer
pixels for both directions and both AR24/XR24. Eighty changing full-HD images
exercise warm transfer, eviction and synchronization. Separate assertions prove
immutable-ID reuse, staged export refusal, rollback, promotion and exact pixels
after exporting a retained snapshot and destroying its original context.

The initial implementation passed these checks but paid for repeated failed
direct attempts. The bounded transfer-order hint reduced warm capture-call
medians from roughly 8–12 ms to roughly 6 ms on this host. Raw samples and
per-case p50/p95 are retained. Capture-call time is CPU submission latency;
capture-through-readback additionally includes output composition, consumer
initialization and GPU readback. Neither metric is display latency, and the
small sample does not establish sustained desktop frame pacing.

The public Smithay review used commit
`13738f8f2cc18224c229e7e8309ccdaa34e92e2a`. Its client-import shadow path relies
on DMA-BUF implicit synchronization; no admitted source returns a refusal,
not a memory rescue for an unreadable descriptor. Sophia chooses a verified
linear bridge on this path rather than assuming equal tiled modifiers work
across devices. See the [kernel DMA-BUF synchronization contract](https://docs.kernel.org/driver-api/dma-buf.html).

## Limits and remaining gates

This is not closure of t069. No build was installed, no session restarted and
no browser or live input exercised. Physical multi-client, device-loss and
recovery acceptance remain separate from offscreen pixel and state tests.

The frontend provider and allocator still belong to their original GPU. Native
worker recovery does not replace those resources; loss of the original device
can refuse new DRI3 opens or exports. Transparent primary-GPU replacement would
need coordinated provider/capability/allocation generations or a new frontend.
The current identity checks must refuse a replaced node, not claim migration.

Bounded inventory and queues do not cancel a blocked kernel driver call. If no
admitted GPU can sample an image or create the verified bridge, import remains
a named refusal. This path does not add external-only or arbitrary YUV sampling.
The [client-internal Brave failure](../investigations/uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md)
still occurs before Sophia receives a buffer and is not repaired by this work.
