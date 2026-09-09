---
id: 6tewvlbh
date: 2026-09-08
kind: plan
tags: [plan, rendering, x11]
---
# Universal device negotiation across Sophia clients

## Scope and exit

Provide a universal device-negotiation solution equivalent in purpose to
Wayland DMA-BUF feedback, adapted to Sophia's native X11 architecture. Device
and buffer compatibility must follow capabilities and resource identity rather
than executable names, browser flags, or user-maintained launch recipes.
There must be no supported-application registry anywhere in the implementation.
Users and developers should receive the same negotiated behavior through normal
startup, a shortcut, a catalog entry or an independent terminal command.

Mason explicitly promoted this work to top priority on 2026-09-08. `t069` owns
its design and implementation; its ordering and status live in
[todo.md](../../../todo.md). The Chromium launch adapter in `t068` remains a
bounded compatibility measure and does not satisfy this task's exit.

## Design and acceptance

- Trace advertisement, allocation, import and presentation across Sophia's
  existing DRI3/EGL/GLX paths. Identify which device and format/modifier facts
  every client can consume, including independent video-allocation paths.
- Review niri/Smithay DMA-BUF feedback and relevant X11 references. Borrow
  capability and lifecycle semantics without adding Wayland to Sophia or
  assuming that a server advertisement controls all client device selection.
- Keep Engine protocol-neutral, frontend translation authoritative, and the WM
  blind. Engine must contain no application-specific adapters, executable-name
  recognition, browser switches or toolkit exceptions. It consumes generic
  device identity and buffer capabilities. Protocol translation stays in the
  frontend; the existing temporary Chromium workaround stays in the CLI.
  Specify device identity, compatible format/modifier sets, topology changes,
  stale evidence and bounded failure before changing interfaces.
- Define a performant cross-device strategy where direct import is unavailable;
  measure any copy or conversion path. Do not count a crash-driven software
  browser fallback as successful negotiation.
- Prove behavior for unclassified executables and multiple client/toolkit
  families, launched both through Sophia and independently from a terminal.
  Cover both same-device and incompatible cross-device buffers on the two-GPU
  host, plus device loss and recovery with bounded resources and no stale reuse.
- Retain exact source, binary, device and buffer evidence, deterministic
  conformance tests, and physical accelerated-video acceptance. State any
  client/protocol limitation explicitly instead of claiming universal coverage
  from a supported-name list.

## Approved implementation contract

The 2026-09-09 implementation keeps the end-to-end exit above. Server-side
conformance, transfer tests and an adapted browser are intermediate evidence;
`t069` stays open until normal unmodified clients work without device overrides.
A client-internal failure before submission remains an explicit boundary, even
if its eventual resolution requires an upstream client change.

Each authenticated connection pins an immutable device bundle: one originating
render device, its measured import capabilities and its allocator. Installing a
successor changes new connections only. Backings keep their originating bundle
through publication, free, cross-connection use and final release. Device loss
changes availability, never an old connection's screen modifiers. The frontend
retains at most sixteen live bundle generations and preserves its GLX capability
catalog across replacement.

DRI3 1.3 device hints remain advisory and confer no device authority. Screen
modifiers stay fixed per connection; window preferences may change with exact
surface and output-topology generations. Present reallocation feedback requires
client opt-in, an actual copied completion and proof that another advertised
layout would permit a flip with every other eligibility condition satisfied.
A format refusal by itself is insufficient. Reallocation advice is suppressed
until the window's preference generation changes.

Render-device membership is compared separately from connector topology. An
identical startup event replay causes no output reconstruction. Keep the chosen
client device while it remains healthy, prepare replacements outside the owner
loop and install only an acknowledged current generation. Pending preparation
and renderer refresh obligations survive a bounded deferral.

Rendering always tries the actual incoming descriptor on the destination first.
Successful source-device hints may order fallback candidates but never bypass
that attempt or establish provenance. Cross-device transfer uses persistent
source contexts and at most three reusable internal linear bridge buffers,
charged to the renderer's existing memory budget. GPU completion and release of
internal exports precede reuse. Destination images exported to other consumers
remain immutable allocations: an external descriptor can outlive the renderer's
local reference count.

Worker shutdown must not block the owner on a full channel or an unfinished
thread. Retired workers remain in a bounded lifecycle registry; replacements on
the same exact device identity wait for their predecessors to finish. No claim
is made that a thread can cancel an unresponsive driver call.

Validation proceeds through protocol and lifetime tests, source-backed
contrary-path review, real same-device and cross-device pixel/performance probes,
and the normal-launch physical gate. Signed implementation checkpoints may be
pushed before physical acceptance; installation and acceptance are separate.

## SuboptimalCopy gate

The 2026-09-09 contrary-path review found no sufficient counterfactual proof
that a different allocation would flip. Keep signaling disabled until that gate
passes. This is conformant: Present reallocation advice is optional. The five
tracked obligations are:

| Condition | Implementation seam | Evidence and exit |
| --- | --- | --- |
| Client opt-in | `dispatch/extensions/present.rs`, `present_standard_pixmap`, transaction record | Bit `0x8` is decoded and validated but discarded. Preserve it per exact transaction only after the proof gate is met; test opted-in and ordinary clients. |
| Actual copied disposition | `live_session/presentation.rs`, `LivePresentBufferDisposition` | Existing ownership is exact. Regression must restrict advice to `Copied`; `Retained` and `Flipped` remain `Flip`, with existing idle ordering preserved. |
| Different layout would permit a flip | Backend allocation and atomic validation | Prerequisite for signaling. A failed TEST_ONLY has no attributed cause. Establish a sufficient proof on the exact candidate and output generation, with an available alternative allocation. |
| Every other eligibility check passes | `DirectScanoutVerdict`, exporter atomic validation | Composition proof exists. KMS eligibility still needs the counterfactual proof above. Unrelated geometry, assignment, bandwidth, synchronization or policy refusal must suppress advice. |
| No repeat per preference generation | Frontend per-window allocation state | Record the last signaled exact surface/preference generation. A later ordinary Copy is allowed; a second SuboptimalCopy in the same generation is not. |

A differential TEST_ONLY with an otherwise equivalent candidate allocation can
supply the missing evidence. IN_FORMATS membership can prove that the original
layout is unsupported, but an alternative's membership alone does not prove
that it would pass atomic validation. Preserve that distinction in any evidence
type; do not name a necessary condition as a sufficient one.

The proof task also resolves three open questions: whether atomic failure errno
is retained anywhere useful; whether the live preferred modifier set is nonempty;
and whether the cached direct-scanout test resets on buffer/layout change.
Treat the review's negative searches as bounded observations until those paths
are traced. Tests must cover stale candidates and topology changes as well as
one successful alternate-layout proof.

## Implementation checkpoints

First replace invented DRI3 modifier responses with measured renderer import
capabilities and refuse legacy exports that cannot represent the backing's
actual offset or stride. Keep import and allocation/export capabilities distinct.
Then add bounded renderer-owned multi-device import and transfer where direct
sampling fails, with exact buffer identity and synchronization. Acceptance needs
real device and pixel evidence; an advertised overlap alone is insufficient.

The initial independent audits establish a boundary: Chromium can fail to import
its own media buffer before submitting any buffer to Sophia. Its preliminary
GPU identity comes from PCI enumeration, while its X11 importer uses DRI3 Open.
Neither standard server feedback nor server-side transfer can intercept that
client-internal failure. This task must not claim to fix that path through server
capabilities; retain it under `t068` with the source evidence and explicit limits.

The [first capability checkpoint](../milestones/f24uuvwu-measured-dri3-import-capabilities-and-exact-legacy-exports.md)
records measured DRI3 negotiation and exact legacy exports, its retained
validation, and the transfer and physical-acceptance limits.

The [cross-device capture checkpoint](../milestones/j11gjkh1-generic-cross-device-image-capture-and-worker-recovery.md)
records submitted-buffer transfer, worker and inventory lifetime repairs,
offscreen pixel evidence, and the remaining frontend migration boundary.

The [first-buffer regression investigation](../investigations/09ywbgwt-measured-modifiers-exposed-an-rgb-stride-check-on-compression-metadata.md)
records a real Mesa client rejected before Present because descriptor validation
applied RGB image geometry to compression metadata. Repair `009498fb` retains
descriptor bounds and passes real GLX/EGL GPU pixel checks. Mason confirmed
Kitty and Quickshell visibility and responsiveness in the installed release;
this accepts that regression repair without closing the broader exits above.

The [implicit-modifier investigation](../investigations/n5i1x7iv-implicit-dma-buf-exports-used-the-wrong-drm-modifier-sentinel.md)
records a second generic defect: legacy DRI3 exports preserved the FD but
substituted a non-ABI modifier, producing black pixels in a real EGL consumer.
The corrected export passes private pixel checks; Brave's logged initialization
failure and physical video acceptance remain separate gates.

The later [default-visual investigation](../investigations/g930kzbe-default-x-visual-excluded-rgba-pixmap-configurations.md)
traces the surviving initialization failure to alpha-zero GL configurations
on the default X visual. The XLibre-compatible correction separates native X
depth from GL color bits and passes default-visual texture and window pixel
tests. Mason confirmed normal video on installed `b43d23d0bb15` on 2026-09-09.
That accepts the white-video repair with the existing CLI device override;
it does not satisfy this plan's requirement for generic device negotiation
without an application adapter.

The [connection and renderer refresh checkpoint](../milestones/szr8j0rg-connection-pinned-device-negotiation-and-bounded-renderer-refresh.md)
records immutable frontend bundles, DRI3 hints and window preferences, bounded
source inventory refresh, pooled internal transfers, and full repository plus
2,400-frame offscreen pixel validation. Its remaining physical and client-internal
limits preserve the end-to-end exit above.

## Connections

The [Brave GPU investigation](../investigations/uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md)
records the cross-device failure and current CLI adapter. The
[GLX pixmap investigation](../investigations/uffn76nu-glx-pixmap-exports-need-coherent-backing-and-reply-ordering.md)
records related export and lifetime repairs. Preserve the boundaries in
[architecture](../../architecture.md) and the current
[client launch contract](../../configuration.md#client-launch-adapters).
