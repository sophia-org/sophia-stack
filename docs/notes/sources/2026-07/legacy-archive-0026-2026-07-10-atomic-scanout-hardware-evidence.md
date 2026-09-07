---
id: legacy-archive-0026
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-07-10: Atomic Scanout Hardware Evidence

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 1308–1447. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The local non-hardware atomic scanout gate passes, including GBM/EGL scanout
feature tests, backend-live scanout intake tests, and strict reduced verifier
fixtures.

Reduced atomic scanout evidence is now schema 6. The new `page_flip_wait` field
keeps the destructive hardware smoke diagnosable without native IDs: passing
captures must reduce both phases to `Retired`, while failed captures can now
distinguish missing callbacks, callback rejection, poll backpressure,
disconnected pollers, waiting retirement, missing retirement, and native
resource-retire failure.

Reduced atomic scanout evidence is now schema 8. The new `framebuffer` field
keeps repeated `FramebufferCreateFailed` hardware-smoke failures actionable
without leaking framebuffer IDs, GEM handles, or errno values. The reduced value
records whether backend-live created the framebuffer with AddFB2, AddFB2 with
modifiers, legacy AddFB fallback, or failed the attempted registration path.

After a real smoke reduced to `framebuffer=AddFb2ThenLegacyAddFbFailed`, the
backend-private scanout buffer adapter stopped normalizing explicit
`DRM_FORMAT_MOD_LINEAR` away. Framebuffer registration now tries modifier-aware
AddFB2 for explicit linear GBM buffers, then falls back to implicit AddFB2 and
legacy AddFB. The proof verifiers accept any reduced created-framebuffer path,
while still rejecting reduced framebuffer-registration failures.

Reduced atomic scanout evidence is now schema 10, and runtime rendered-scanout
submit evidence is now schema 6. Both lines include reduced scanout-buffer
layout fields: `buffer_format`, `buffer_modifier`, and `buffer_planes`. The
allowed values intentionally collapse the native descriptor to broad facts such
as `Xrgb8888`, `Argb8888`, `Implicit`, `Linear`, `NonLinear`, `Single`, and
`Multiple`. That gives the next framebuffer-registration failure enough shape
to distinguish unsupported format, modifier, and plane-count cases without
leaking GEM handles, fds, pitch/offset arrays, exact modifier values, or native
driver errors.

The selected primary-plane property discovery path now carries the optional
`IN_FORMATS` property handle privately and reduces it to `format_table=Present`
or `Missing` in atomic and runtime submit evidence. This does not parse the
kernel blob yet, but it proves whether the authority has the metadata source
needed for proper format/modifier admission before relying on AddFB failures.

The next hardware smoke failed with `buffer_format=Argb8888`,
`buffer_modifier=Invalid`, `buffer_planes=Single`, `format_table=Present`, and
`framebuffer=AddFb2ThenLegacyAddFbFailed`. That showed the renderer asked for a
scanout surface but accepted an EGL config whose native visual did not match the
requested GBM format, and it forwarded `DRM_FORMAT_MOD_INVALID` as if it were a
real explicit modifier. The native EGL scanout exporter now selects only configs
whose `EGL_NATIVE_VISUAL_ID` matches the requested GBM format and normalizes an
invalid GBM modifier to the implicit/no-modifier path.

A later smoke improved to `buffer_format=Xrgb8888`, `buffer_modifier=Implicit`,
and `buffer_planes=Single`, but still failed framebuffer registration. The
rendered scanout exporter now prefers linear GBM surfaces before the default
driver layout. This keeps the rendered path first while giving legacy AddFB2
and AddFB a buffer layout they are more likely to register without explicit
modifier metadata.

That flag-only linear request still produced `buffer_modifier=Implicit` on the
hardware proof. Backend-live now has a private, bounded parser for the DRM
`IN_FORMATS` blob so Sophia can reduce primary-plane format/modifier capability
without exposing property blob IDs or raw native tables. In parallel, the native
GBM/EGL rendered scanout exporter now tries an explicit
`DRM_FORMAT_MOD_LINEAR` surface before the flag-only linear and default
surfaces. If the driver accepts it, the exported descriptor should report
`buffer_modifier=Linear`, letting backend-live use modifier-aware AddFB2 instead
of the implicit AddFB path.

The primary-plane resource path initially admitted only one active plane for the
packed XRGB8888/ARGB8888 scanout formats Sophia supported. Multi-plane scanout
descriptors failed closed as `InvalidBuffer` before mode-blob creation,
framebuffer registration, or cleanup bookkeeping.

TTY3 hardware evidence changed that decision. Backend-live now admits explicit
non-linear multi-plane XRGB8888/ARGB8888 buffers to modifier-aware AddFB2 while
still rejecting implicit and linear multi-plane buffers before framebuffer
creation. The first retry moved the reduced failure from
`resources=InvalidBuffer framebuffer=NotAttempted` to
`resources=FramebufferCreateFailed framebuffer=AddFb2ModifiersFailed`, proving
Sophia reached the intended native framebuffer registration path.

Because the driver still rejected that multi-plane framebuffer, the rendered
GBM/EGL exporter now treats multi-plane exports as rejected candidates and keeps
searching for a single-plane scanout buffer. The next TTY3 smoke reached
`buffer_format=Xrgb8888`, `buffer_modifier=Implicit`, `buffer_planes=Single`,
and `framebuffer=AddFb2ThenLegacyAddFbFailed`. A temporary local diagnostic
reported AddFB2 and legacy AddFB failing with `ENOENT`, which points to the KMS
submit fd not seeing the GEM handle exported in the renderer descriptor. The
next production-shaped fix should be backend-private PRIME import: retain
renderer-owned DMA-BUF fds, import them into the KMS submit device, register the
framebuffer from KMS-local GEM handles, and close imported handles on failure or
after framebuffer registration transfers ownership.

The backend-private PRIME import path is now implemented. The renderer-native
GBM owner captures per-plane DMA-BUF fds while the rendered GBM/EGL front buffer
is still freshly locked, then hands out duplicated fds to backend-live submit.
Backend-live imports those fds into the KMS submit device with
`prime_fd_to_buffer`, builds AddFB2/AddFB from the imported KMS-local handles,
and keeps imported GEM handles in the existing resource cleanup debt path.

TTY3 evidence moved framebuffer registration past the previous `ENOENT` handle
visibility failure. One smoke run produced `InitialModeset status=Passed` with
`resources=Created`, `framebuffer=CreatedWithAddFb2`, `page_flip=Presented`,
and `retire=RetiredAfterPageFlip`. The same run then reached
`SteadyPageFlip resources=Created framebuffer=CreatedWithAddFb2` and failed at
the non-modeset atomic submit. Later retries submitted the imported initial
modeset but did not receive the first page-flip callback before timeout. The
next investigation is therefore post-import page-flip progression, not
framebuffer registration.

The post-import page-flip blocker was resource lifetime in the destructive
smoke harness. The smoke retired the just-presented initial framebuffer before
submitting the steady page flip. The runner now waits for the initial modeset
callback without destroying its scanout resources, submits the steady non-modeset
page flip while the initial framebuffer remains active, then retires the initial
resources after the steady callback. The default page-flip wait is 8 seconds and
the parent watchdog is 30 seconds. TTY3 now produces two passing reduced
evidence lines: `InitialModeset` with `request_scope=Modeset
commit_allow_modeset=true` and `SteadyPageFlip` with `request_scope=PageFlip
commit_allow_modeset=false`, both using `framebuffer=CreatedWithAddFb2` and
`retire_cleanup_pending=false`.

The combined TTY3 hardware proof now passes end to end. The proof command runs
preflight, destructive two-phase atomic scanout, runtime rendered-scanout
submit-to-retire capture, and all three offline verifiers. The preflight log
reduces to one atomic-ready primary card with scanout target and atomic
properties available. The destructive scanout evidence passes both phases with
`buffer_format=Xrgb8888`, `buffer_modifier=Implicit`, `buffer_planes=Single`,
`format_table=Present`, `framebuffer=CreatedWithAddFb2`,
`page_flip=Presented`, `retire=RetiredAfterPageFlip`, and no cleanup debt. The
runtime proof submits a rendered primary-plane page flip at the host output
size, `1920x1200`, with matching target size, then retires cleanly with
`destroy=Destroyed` and `cleanup_pending=false`.

The runtime proof exposed one validation bug: the CLI-side clean-evidence
predicate and shell verifier assumed the fixture mode `1280x720`. That was too
narrow for real hardware. The proof invariant is now that `output_size` and
`target_size` are valid reduced sizes and equal to each other, while all native
identity remains hidden.

<!-- END IMPORTED BODY -->
