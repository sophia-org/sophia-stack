---
id: uqnx2t2b
date: 2026-09-07
kind: investigation
status: investigating
tags: [investigation, rendering, x11]
---
# Brave GPU restarts after VA buffers fail GBM import

## Question

Why does Brave lose its GPU process during GBM buffer import after installing
the AMD VA-API backend? Identify the failing descriptor and device before
changing Sophia's renderer or selecting a browser workaround.

## Observed on 2026-09-07

Installed Sophia is `a44d1e163ceb7188edfebefc9871e1517576f07b`. Brave Origin
1.94.121 uses Chromium 152.0.7977.83. System Mesa, libgbm and the newly installed
mesa-vaapi package are version 26.1.8_1.

The user's `~/brave-vaapi.log` records successful loading of
`radeonsi_drv_video.so`, VA-API 1.24.0 and `va_openDriver() returns 0`.
Installing the driver resolved the earlier initialization failure. This does
not establish working hardware video playback.

Four GPU processes then fail at `gbm_wrapper.cc:465`: 21733 at 20:17:10,
24349 at 20:22:17, 5853 at 20:22:21 and 6666 at 20:22:22. Each failure is
followed by shared-image backing creation failure, lost context and GPU-process
exit 8704. Replacement GPU process 6919 runs with `--use-gl=disabled` and holds
no DRM descriptors. Its later failure to find a VA render node follows that
fallback; it is not evidence of the original cause.

PID 22962 is a renderer. Its error-level `DecoderStatus::0` record denotes
success, not a decoder error. The failed GPU processes have exited, so their
device descriptors cannot be reconstructed from the current process list.

## Source and device evidence

The matching [Chromium GBM wrapper](https://chromium.googlesource.com/chromium/src/+/152.0.7977.83/ui/gfx/linux/gbm_wrapper.cc)
reaches the reported error after its format-support check and a failed
`gbm_bo_import` with `GBM_BO_IMPORT_FD_MODIFIER`. Its error omits the descriptor,
importing device and `errno`. The log cannot distinguish a bad descriptor from
an unsupported modifier or device pairing.

Brave's ELF dynamic table names `libgbm.so.1`; its undefined dynamic symbols
include `gbm_bo_import`, `gbm_create_device` and `gbm_device_get_fd`. The current
GPU process maps system Mesa's `/usr/lib/libgbm.so.1.0.0`. This supports an
import-call interposer for diagnosis. It does not by itself establish which
compile-time branches Brave enabled.

| Node | PCI device | GPU |
| --- | --- | --- |
| renderD128 | 0000:03:00.0, 1002:744c | Navi 31, discrete |
| renderD129 | 0000:16:00.0, 1002:164e | Raphael, integrated |

A private VA allocation and GBM import probe exercised NV12 at 640×480 and
1920×1080, without an X connection or KMS changes. Same-device imports
succeeded on both nodes with flags 0 and SCANOUT. All cross-device imports
failed with `errno=38` (ENOSYS). Exported modifiers were
`0x0200000028a01f04` on renderD128 and `0x0200000000401b03` on renderD129.
These are different AMD tiling layouts.

This demonstrates an incompatibility, **not Brave crash attribution**. The
probe's NV12 format-support query returned false even where import succeeded;
Chromium checks support before the failing call. The browser's actual format,
modifier and importing node remain essential evidence.

The [VA wrapper](https://chromium.googlesource.com/chromium/src/+/152.0.7977.83/media/gpu/vaapi/vaapi_wrapper.cc)
filters render nodes against the active GPU only when passed a non-null
`GPUInfo`. Unconditional pinning has not been established. Also,
[platform_video_frame_utils.cc](https://chromium.googlesource.com/chromium/src/+/152.0.7977.83/media/gpu/chromeos/platform_video_frame_utils.cc)
reads `--render-node-override`: it is not a VA-only control. Two override runs
would not establish that only the VA display's device changed.

## Diagnostic prepared

Temporary artifacts are `/tmp/sophia-va-gbm-probe.py` and its `.log`, plus
`/tmp/sophia-brave-gbm.KpbC7NfW/` for a GBM interposer and its fixtures.
`/tmp/sophia-brave-gbm.sh` launches a fresh diagnostic Brave instance when the
user runs it. These are working artifacts, not a durable evidence archive.

The interposer records the importing DRM device number, dimensions, fourcc,
modifier, plane count, strides, offsets, result and returned `errno`.
It never maps buffer pixels. It records the first 64 imports and all subsequent
failures. A linked fake GBM fixture verified unchanged arguments, return values
and `errno`, the successful-log bound, and failure capture after that bound.
Compilation passed `-Wall -Wextra -Werror`.

The launcher creates a private, uniquely named log, retains existing preload
entries and browser sandbox settings, and uses the wrapper's `-no-update --`
argument boundary. Check-only mode was verified against the host process list:
it refuses while Brave remains open. The restricted tool environment hides
those host processes, so its apparent empty list was not used as evidence.
No browser was launched, stopped or reconfigured by this investigation. A real
browser run has not yet proved that sandboxed import calls reach the interposer.

## Instrumented restart and exact-format reproduction

The user restarted through the diagnostic launcher. The wrapper reached the
real import calls. GPU PIDs 17004, 18278 and 18401 imported on DRM device
226:128 (renderD128), with AR24 `0x34325241`, modifier
`0x0200000000401b03`, one plane, zero offset, and SCANOUT flags `0x1`.
The 640×360 buffers had stride 2560; the 1024×682 buffer had stride 4096.
All three imports returned null with `errno=38`, immediately before context
loss and GPU exit 8704. Replacement PID 19451 reached the later VA fallback.

A second private probe allocated **ARGB with explicit modifiers**, removing
the earlier NV12 mismatch. On renderD129 it produced exactly the browser's
format, modifier, dimensions, stride and offset. Import on renderD129 passed;
import on renderD128 reproduced ENOSYS. Both nodes reported ARGB format support.
The reverse device pairing also failed for the GFX11 modifier. These results
establish that the captured descriptor layout is incompatible with its importing
GPU. They do not identify who exported the browser's buffer. A width×4 stride
is valid for the observed tiled layout at these dimensions; the probe directly
refutes an inference that this pitch must indicate a malformed descriptor.

An initial ARGB probe using implicit modifiers returned DRM_FORMAT_MOD_INVALID
and imported successfully on both devices. It did not reproduce the browser
failure and was retained alongside the explicit-modifier result.

Chromium's [GBMSupportX11](https://chromium.googlesource.com/chromium/src/+/152.0.7977.83/ui/gfx/linux/gbm_support_x11.cc)
obtains its importing device from DRI3 Open. Sophia constructs its render-device
provider and server pixmap allocator from the same native-scanout device source;
the provider resolves the matching PCI render node. Source review found no
selection split in those two providers. This does not establish provenance for
a buffer imported by the browser.

The next requested user run adds `--render-node-override=/dev/dri/renderD128`
to the same diagnostic launcher. This is a local alignment experiment for the
current topology, not a portable desktop default. It changes the media device
selection paths and tests whether their buffers then import on the existing
X11 GBM device. The subsequent result is recorded below.

Redacted diagnostics, probe sources, fixtures and SHA-256 manifest are retained
under `~/.local/state/sophia/development-evidence/t068-gbm-import-20260907/attempt-001/`.
The source browser log was `/tmp/sophia-brave-gbm.KpbC7NfW/brave-VYYhYhM2.log`.

## Device-aligned run

The user relaunched with `--render-node-override=/dev/dri/renderD128`. The new
log, `/tmp/sophia-brave-gbm.KpbC7NfW/brave-wSKBw9l4.log`, captured 64 successful
imports and no failed imports or GPU exits. The interposer caps successful
records at 64 but continues recording every failure. Imported ARGB buffers now
use GFX11 modifier `0x0200000028a01f04` on node 226:128. GPU PID 4991 retained
seven DRM descriptors, all on renderD128; its command line carries the override
and does not carry the previous `--use-gl=disabled` fallback.

The failing import, exact-format reproduction and successful alignment run
support a browser media-device mismatch as the cause of these GPU crashes.
The importer uses the X server's advertised device; the media override produces
buffers whose layout that device accepts. The exact browser allocation call
that chose the other device was not instrumented, so this does not attribute
the exporter to a particular decoder or image-processing function.

The user reports that border flicker stopped after this relaunch, before any
Sophia repair was installed. Preserve that physical result separately from the
[repaint focus defect](h0vxis10-brave-gpu-watchdog-repeats-during-live-use.md).
It is not evidence that a pending code change has been physically accepted.

The successful comparison and manifest are retained in
`~/.local/state/sophia/development-evidence/t068-gbm-import-20260907/attempt-002/`.
This establishes a working launch override for the current machine. Ordinary
future launches still need that argument; no persistent launcher change was
made. A portable repair must choose a device matching the current X display or
negotiate an importable cross-device layout, rather than assume renderD128
always names the intended GPU.

## White video after device alignment

The user then reported white YouTube video. The aligned run still has no GBM
import failures or GPU restarts, but contains 3178 matching error chains:
`native_pixmap_egl_x11_binding.cc:98` reports rejected DRI3 PixmapFromBuffer;
`ozone_image_backing.cc:320` cannot create a GL representation; and
`shared_image_manager.cc:269` cannot produce a Skia image from that backing.
These failures were present in the aligned run before the white-video report;
the earlier import-success count did not establish complete image rendering.

Chromium selected its X11 pixmap fallback because direct EGL DMA-BUF import
was unavailable on that EGL display. The fallback supports BGRA8888, uses
plane zero and forwards its stored byte size. The matching VA wrapper at lines
2829–2832 deliberately stores zero as the plane size to prevent CPU mapping.
Sophia's pure DRI3 validation rejects stride×height greater than the declared
size. The browser wire size itself has not been captured; this chain is traced
from the matching source.

A private live probe sent the same real tiled ARGB buffer through DRI3 with
three declared sizes. Its kernel-reported length was 1572864 bytes, geometry
640×360 and stride 3072. Size zero produced core BadWindow (3), major 137,
minor 2. Size 1105920 (stride×height) and size 1572864 both succeeded. The probe
created only its own unmapped pixmaps and changed no application input or
window state. This establishes the zero-size compatibility failure independently
of the browser's graphics stack.

The log's `Glx::BadPixmapError` label does not identify a GLX request: Sophia's
GLX error base is zero, so Chromium labels core error 3 as GLX error index 3.
The DRI3 request identity and actual core error code were retained by the probe.
Also, an implicit DRM modifier does not mean linear pixels. The driver may
recover tiling from the FD; DRI3 PixmapFromBuffer permits nonlinear buffers.
The local X server reference forwards the FD with an implicit modifier and does
not use the request's size field.

The candidate handles only a zero declared size at the frontend FD boundary,
before pure dispatch, using the kernel-reported length. It preserves explicit
nonzero sizes and the existing descriptor, stride, dimension and namespace
checks. Failed or unusable size queries remain refusals. This avoids admitting
a pixmap first and discovering its bounds later, and does not map pixels or
seek a client-shared file description. Renderer-private import ownership stays
unchanged.

The real-socket regression passes all five cases: zero declared size with a
sufficient backing, short backing and empty backing; an explicitly short
nonzero declaration; and a valid nonzero declaration. Successful imports must
answer GetGeometry with 640×360 at depth 32. Refusals must return an X error
without a geometry reply. Every case preserves the shared file offset of 17.
Mutations that remove normalization, overwrite nonzero sizes, substitute an
unmeasured size, or seek the descriptor each fail the relevant assertion.
The X11 wire suite passes all 330 tests.

The combined candidate, based on `a44d1e163ceb`, passes `cargo xtask check`,
including workspace tests, clippy, archive verification and host buffer-age
equivalence. The gate also includes the separate repaint-focus repair recorded
in the [border investigation](h0vxis10-brave-gpu-watchdog-repeats-during-live-use.md).
Socket admission does not prove that Chromium subsequently displays the
imported image.

Evidence is retained under
`~/.local/state/sophia/development-evidence/t068-gbm-import-20260907/attempt-003/`.

## Playback observation on the installed repair

The user reported video playing in session
`00000001788830790138-53ca7f8e-e19c-4b19-8ede-698643da2576`, whose manifest
identifies commit `559b3907e5e2290ad1c47deb0f6a77567e3b1db1`.
However, the subsequent process inspection found GPU process 30652 running
with `--use-gl=disabled`. Browser process 26216 had no diagnostic logging or
render-node override flags, and stderr pointed to `/dev/null`. No new diagnostic
wrapper log existed. Playback is therefore confirmed, but accelerated playback
through the repaired import path is not. This observation does not establish
why that process uses disabled GL. A fresh diagnostic launch with the previously
successful device alignment is still needed to accept the GPU path.

## Accelerated playback still fails at EGL binding

The user restarted with the render-node override and again reported white
video. In `brave-TVVaImZk.log`, GPU process 2368 records 64 successful GBM
imports on renderD128 with the GFX11 modifier, no GBM failures and no GPU
exits. The old PixmapFromBuffer rejection is absent. Instead, 1483 failures
occur at `native_pixmap_egl_x11_binding.cc:205`, followed by the same GL
representation and shared-image errors. DRI3 admission now succeeds; the
remaining failure is initialization of the EGL pixmap binding.

That initializer either fails to select an RGBA8 pixmap config with texture
binding or fails to create its pixmap surface. Its single diagnostic does not
distinguish the two. Sophia offers GLX drawable mask `0x5` (windows and
pbuffers), no GLX pixmap constructors and no texture-from-pixmap extension.
The matching ANGLE revision `7df613367a1d4ca9aea9ece344d4580d32d132a9`
derives its GLX-backed EGL pixmap and binding support from those capabilities.
Brave maps Mesa GLX libraries and no external EGL library, consistent with
that backend; the browser's EGL vendor/backend was not directly queried.

A private connection to the same display confirms three GLX configurations,
all with drawable mask `0x5`. Separately, system Mesa EGL initializes there,
advertises DMA-BUF import and modifier import, and supplies a matching RGBA
pixmap config. These are different client implementations; the Mesa query is
not a query of Brave's EGL display.

The next controlled candidate adds `--use-angle=gl-egl` to the existing
render-node override. Matching Chromium source selects ANGLE's EGL device
backend for this flag; ANGLE's EGL backend forwards native DMA-BUF import
support. This could bypass the GLX pixmap fallback. It remains untested in
Brave and does not justify advertising unimplemented GLX capabilities.

## Reference code for the remaining interop work

Inspected local yserver commit `a1e33aa8176c65b418c1b6bef4c11f4598d5c2cc`.
Its current implementation includes GLX pixmap resource tracking,
texture-from-pixmap configuration attributes, capability-gated extension
advertisement, and modifier-aware DRI3 export. Useful references are
`crates/yserver-core/src/core_loop/process_request.rs` (GLX dispatch/configs),
`crates/yserver/src/kms/vk/target.rs` (export allocation), and
`crates/yserver/src/kms/vk/dri3.rs` (export metadata and sync-file operations).
The original design notes differ from current code; they are not proof of
implemented behavior.

`crates/yserver/tests/glx_tfp_export.rs` has concrete red-to-green pixel
liveness assertions and a backend lifetime test that retains an export across
FreePixmap and removes its registry entry after GLX release. Those tests are
ignored by default, require Vulkan, and can skip on unavailable setup. They
were read, not run. They do not prove the complete GLX wire path or concurrent
GL/Vulkan fence ordering. Adapt their assertions into Sophia's own tests rather
than treating their existence as acceptance evidence.

Also inspected local wayland-rs commit
`0813584ea50379bd22e95dc1e0b50f02a4b36ca2`. This is Smithay's protocol-library
repository, not the Smithay compositor toolkit. `wayland-egl` wraps client-side
`wl_egl_window` ownership; it supplies no reusable DMA-BUF importer for this
X11 failure. `wayland-backend/src/rs/socket.rs` is useful for owned received
FDs, close-on-exec reception and separate byte/FD accounting on partial sends.
`wayland-protocols/src/wp.rs` documents acquire/release timeline semantics;
the referenced protocol XML submodule is empty in this checkout, so its full
contracts were not inspected here.

Both repositories use MIT licenses; copied substantial code must retain the
respective notices. GLX/XID validation belongs in Sophia's X authority, buffer
storage and synchronization in its renderer/Engine, and neither in the WM.
No reference source was copied into production. Test native EGL selection
first; use yserver as an implementation reference if Sophia needs to support
the GLX pixmap fallback without browser-specific flags.

## Acceptance and architectural boundary

Capture a failing browser import, then reproduce its descriptor/device
combination in a bounded probe. Repair the fault where that evidence places
it: client allocation/import, frontend device advertisement, or Engine import.
The WM has no role in choosing VA devices or repairing buffer formats. Keep
namespace and sandbox boundaries intact.

Acceptance requires a fresh GPU-enabled Brave process, successful imports
through the formerly failing interaction, and no crash-driven software
fallback. Preserve the exact configuration and evidence. The local override
has passed the captured interaction. The subsequent GLX candidate passes its
private pixel gate; normal launch without graphics-selection flags and broader
daily-use acceptance remain open.

## Connections

`t068` in [todo.md](../../../todo.md) owns this work. The earlier
[delayed-publication investigation](vwo9wmie-window-switches-reveal-delayed-visual-updates.md)
concerns Sophia frame scheduling. A browser GPU crash is a separate failure;
neither diagnosis establishes the other.

The subsequent [GLX pixmap export investigation](uffn76nu-glx-pixmap-exports-need-coherent-backing-and-reply-ordering.md)
records the configuration, pixel-publication and retained-storage repair.
