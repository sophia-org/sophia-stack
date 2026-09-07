---
id: legacy-active-0097
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-07: GetImage replies are not bounded like image-upload requests

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3192–3231. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed release `0.1.0-53a213655a41` corrected committed-layout reseeding.
Physical run `0040` reasserted the stale Kitty geometry, admitted Firefox,
advanced the automated workflow through the dialog stage, retained correct
mixed presentation, and reached normal logout. The strict verifier rejected
the otherwise complete run because it observed eight X protocol errors. The
first was `BadValue` at sequence 414 for core major opcode 73 (`GetImage`).
Firefox also reported that its background page-thumbnail request failed.

The wire decoder was the root cause. Core `GetImage` is a fixed 20-byte request,
but Sophia computed the potential reply as `width * height * 4` while decoding
it and rejected anything above the 256 KiB `PutImage` request-data limit. A
normal Firefox readback around `1290x1050` is roughly 5.4 MiB, so a valid
request was rejected before drawable validation or readback. The request and
reply bounds are different protocol concerns.

XLibre's `DoGetImage` validates `XYPixmap` or `ZPixmap`, drawable access,
viewability, and bounds, then streams the computed reply through a bounded
intermediate buffer. Yserver retains the same request validation and computes
the format-specific reply length without applying a request-body ceiling.
Sophia now follows that division. The decoder validates only the fixed request
shape and legal format. X Authority derives a checked ZPixmap or XYPixmap
layout from the advertised depth, scanline pad, plane mask, and client byte
order; rejects invalid drawables, matches, formats, or allocations with the
corresponding X error; and caps authority-owned CPU image memory at 64 MiB.
Core X11 and MIT-SHM use the same validation, passive software-buffer readback,
and pixel packer. Missing CPU backing remains deterministic zero-filled data;
this change does not add a GPU screenshot path or move X11 semantics into
Engine.

Regressions cover both byte orders, a Firefox-sized decode, ZPixmap and
XYPixmap layout, empty replies, pixel preservation, drawable/access/bounds
errors, and allocation refusal. A real Unix-socket test reads a 320,000-byte
reply, above the retired ceiling, and the compiled Xlib smoke now performs
`XPutImage` followed by `XGetImage` and verifies the returned pixel. This is a
request/reply implementation boundary rather than concurrent authority state,
so it does not require a new TLA+ model. Physical promotion remains pending a
new installed Firefox run with zero protocol errors.

<!-- END IMPORTED BODY -->
