---
id: n5i1x7iv
date: 2026-09-08
kind: investigation
status: confirmed
tags: [investigation, rendering, x11]
---
# Implicit DMA-BUF exports used the wrong DRM modifier sentinel

## Incident and limits

On installed `009498fb0aae`, Kitty and Quickshell were physically accepted after
the first-buffer repair. Brave launched through Super+B still showed white
video. Its aligned GPU process stayed alive on renderD128; the diagnostic log
reported repeated `NativePixmapEGLX11Binding::Initialize` failures at Chromium
152.0.7977.83's `native_pixmap_egl_x11_binding.cc:205`, with no DRI3 import error
at line 98, no GBM import failure and no GPU restart.

An earlier diagnostic command accidentally omitted Super+B's existing CLI
device adapter. That different launch had three GBM failures and GPU restarts,
then played video with `--use-gl=disabled`. It does not establish recovery of
the aligned launch. The original unlogged launch discarded stderr.

A read-only live GLX query confirmed texture-from-pixmap in both the server
advertisement and the client-visible extension set. All six configurations had
drawable mask 7 and RGBA binding; rows 5 and 6 also had alpha 8 and stencil 8.
Missing capability advertisement was therefore ruled out for this session.

## Reproduced defect

The protocol crate defined `DRM_FORMAT_MOD_INVALID` as `u64::MAX`. The DRM ABI
uses `0x00ffffffffffffff`: vendor NONE occupies the high byte, and only the low
56 bits are reserved. Compilation against the installed `drm_fourcc.h` and
the GBM crate's `Modifier::Invalid` independently establish that value.

Legacy DRI3 `PixmapFromBuffer` stores an implicit descriptor using this
constant. `BuffersFromPixmap` exports that modifier unchanged. A real ARGB GPU
buffer therefore returned with the same file-descriptor identity, extent,
stride and offset, but the wrong modifier.

The private EGL test painted and read back the source buffer before export.
The legacy import returned `0xffffffffffffffff` and sampled black. A paired
import carrying `0x00ffffffffffffff` returned the original FD and sampled exact
initial and updated pixels. Both one-connection and separate-connection legacy
cases failed, while CPU pixmaps passed. After correcting the constant, both
legacy cases pass unchanged pixel expectations.

This is a confirmed protocol/export defect. It does **not** establish the cause
of Brave's earlier initialization failure: system EGL initialized before its
pixel failure, and both direct-GLX initializer controls also succeeded. ANGLE's
configuration selection and surface creation still need an exact failing-call
trace or acceptance of the installed correction.

## Repair and validation

The shared descriptor now uses the DRM ABI value. Native EGL uses GBM's same
value for implicit imports and absent modifiers. Capability and preferred-layout
filters continue excluding malformed all-ones values. Explicit tiled modifiers,
descriptor bounds, namespace rules and original FD ownership remain enforced.
No copy path, application recognition or browser-specific Engine rule is added.

Tests pin the numeric ABI, packed-row bounds for canonical implicit descriptors,
little- and big-endian legacy DRI3 replies, exact CPU/imported EGL pixels across
connections, and native implicit-modifier import/readback. The hardware gate
also retains the GLX/EGL compressed first-frame checks that caught the prior
blank-window regression. It rejects empty or partial test selections.

`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` passed: 2,805 tests across
249 successful test summaries, no compiler or Clippy warnings, and the hardware
pixel gates passed on renderD128. This includes all seven pixmap cases and the
native implicit-import readback case.

Evidence is retained under `.artifacts/t068-implicit-modifier-20260908/`, including
the failing legacy import, passing canonical control, original FD identity,
post-fix pixel checks, the complete check log, and redacted browser-stage counts.
Browser profile data, URLs and raw user log contents are not copied there. The
evidence manifest records the candidate identity and file hashes.

Physical accelerated-video acceptance remains open under `t068`; this repair
does not close `t069`'s broader generic-device requirement.

## Connections

The [Brave investigation](uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md)
owns the client-internal GBM failure and the later white-video observations.
The [pixmap export contract](../../pixmap-texture-exports.md) specifies modifier
and storage preservation. The [universal-device plan](../plans/6tewvlbh-universal-device-negotiation-across-sophia-clients.md)
owns the generic boundaries; the [first-buffer investigation](09ywbgwt-measured-modifiers-exposed-an-rgb-stride-check-on-compression-metadata.md)
remains physically accepted independently of this incident.
