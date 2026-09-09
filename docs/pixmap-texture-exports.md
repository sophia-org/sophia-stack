# Pixmap texture exports

Sophia's X frontend owns GLX pixmap resources. The renderer owns the storage
exported to direct GL clients. A pixmap has no workspace or output assignment;
creating one does not create an Engine surface or involve the WM.

## Capability

The frontend latches the provider's pixmap-texture capability at startup. The
same value controls the GLX pixmap drawable bit, texture-binding attributes,
additional depth/stencil configurations and `GLX_EXT_texture_from_pixmap`.
The capability remains off unless the provider can export imported buffers and
keep CPU-written exports coherent. Unsupported providers retain the base
configuration catalog.

The live provider probes allocation, export, import and successive writes for
XRGB8888 and ARGB8888 using a private GL consumer on the selected device. It
retains one imported image and verifies full and partial updates after rebinding
the texture. The probe creates no window and acquires no scanout resources.
A failed probe withholds the capability; it does not claim support from the
presence of an EGL extension alone.

The framebuffer catalog must describe configurations direct drivers can match.
Texture targets and orientation are part of that match, not decorative hints.
Existing configuration identifiers remain stable. New depth-24/stencil-8 rows
provide a configuration with pixmap and pbuffer support together, including
RGBA texture binding. Indirect GL rendering remains outside the contract. The Mesa-compatible target
mask includes 1D, but the local Mesa direct-GLX client does not decode that target.
Hardware qualification covers 2D and rectangle sampling only.

Native X visual depth and GL color-buffer size are separate. The default
depth-24 TrueColor visual supports RGBA8 GL buffers; its windows remain opaque.
Both GLX configuration-query versions agree on those color bits. Modern GLX
pixmaps may wrap either the native RGB storage or a full RGBA buffer, and report
the backing's actual depth, including after FreePixmap. The legacy visual-based
constructor requires the native visual depth. Tests cover selecting the first
default-visual configuration, retained depth-32 texture alpha, and opaque
depth-24 window pixels.

## Identity and ownership

Wire XIDs stay inside the X frontend. A GLX pixmap retains its backing identity,
so freeing and reusing its original XID cannot redirect an existing reference.
RENDER pictures, GLX pixmaps and in-flight exports share the backing's lifetime.
The renderer receives a separately allocated opaque buffer handle, dimensions,
format, revision and bounded pixel patches. A private X backing key is never
used as a renderer handle.

Imported DMA-BUF storage is re-exported through retained plane descriptors and
owned file descriptors. It is not copied into the provider's CPU-upload store.
An import through legacy DRI3 has an unspecified layout. Its modifier-aware
export must use DRM's `0x00ffffffffffffff` sentinel, with a zero vendor byte;
`u64::MAX` names a different, invalid modifier. The descriptor, driver import
and wire reply use the same DRM value. An unspecified layout never implies
linear storage.
SHM-backed pixmap exports are refused: writes through shared memory have no
authority damage boundary to synchronize. SHM uploads into ordinary CPU
drawables remain available.

Updating an unknown provider handle fails explicitly. Descriptor validation,
namespace checks and direct-driver import remain independent requirements.

A provider allocation has one persistent backing. Initial contents and later
CPU drawing must reach that backing before a reply permits the client to read
them. Keeping an initial snapshot alone does not satisfy this contract.
Provider ownership ends after the last backing reference and in-flight operation
are gone. Client-owned exported descriptors may keep the kernel allocation
alive after provider release.

## Ordering and bounds

Collect work under the authority lock, release the lock for renderer work, then
revalidate the retained identity before publishing completion. GPU allocation,
mapping and synchronization must not hold the frontend's global runtime lock.
An XID lookup after the call is insufficient because the client may have freed
and reused that XID in the meantime.

Only one update per backing may be outstanding. Partial patches are not
cumulative snapshots: a higher revision does not, by itself, contain an older
revision's damage. Failed publication retains its obligation. A read/export
request waits for the finite revision prefix required by that request; drawing
that arrives later must not extend the wait indefinitely.

The live renderer bounds its queue to four requests, each update to 32 packed
rectangles and 64 MiB, and retained storage to 1,024 allocations and 256 MiB.
The authority also bounds outstanding publication targets to 4,096 and owned
provider backings to 1,024. Cleanup debt counts against that capacity even while
a release is outside the lock; only a successful release acknowledgement frees
the slot. Driver-reported allocation sizes count against the storage budget. Requests
have an absolute four-second service deadline. Overflow and expiry are named
failures, not permission to return an old snapshot as current.

Allocation reply enqueue is not ownership transfer. The caller must atomically
adopt the result before its deadline; cancellation prevents late adoption and
the worker releases unclaimed storage. A release that has reached the worker
remains effective even if its caller times out. A release refused before enqueue
remains frontend cleanup debt and must be retried.

## Synchronization and evidence

CPU writes use the driver's GBM mapping path and bracket access with the
DMA-BUF exporter's CPU-access hooks. These hooks alone do not prove GPU
ordering. Direct clients must synchronize producer work and release/rebind the
texture after changing the drawable, as specified by
[GLX_EXT_texture_from_pixmap](https://registry.khronos.org/OpenGL/extensions/EXT/GLX_EXT_texture_from_pixmap.txt).
The renderer must preserve the driver's implicit synchronization; it must not
replace a native modifier with an assumed linear layout.

Deterministic tests cover validation, identity, ordering and cancellation.
Private hardware tests cover exact pixels through retained imports, direct
GLX clients, and EGL pixmap textures using the live provider. CPU-written and
GPU-produced imported pixmaps are checked across separate X connections, with
the original FD identity and the wire modifier preserved. Neither proves accelerated browser
playback in an installed session. That acceptance requires a normal browser
launch without graphics-selection flags, visible video and evidence that the
GPU process stayed accelerated.
