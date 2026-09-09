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

## Connections

The [Brave GPU investigation](../investigations/uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md)
records the cross-device failure and current CLI adapter. The
[GLX pixmap investigation](../investigations/uffn76nu-glx-pixmap-exports-need-coherent-backing-and-reply-ordering.md)
records related export and lifetime repairs. Preserve the boundaries in
[architecture](../../architecture.md) and the current
[client launch contract](../../configuration.md#client-launch-adapters).
