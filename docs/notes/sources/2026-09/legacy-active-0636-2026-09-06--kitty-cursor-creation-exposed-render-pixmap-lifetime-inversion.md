---
id: legacy-active-0636
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-09-06 — Kitty cursor creation exposed RENDER pixmap lifetime inversion

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20468–20508. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The installed `b25b29c1f23a` session started at 20:22:18 UTC. Kitty exited during
startup with `RenderBadPicture` for CreateCursor (major 144, minor 27), picture
`0x200004`, request serial 143. At 20:22:28, Super+Enter admitted a terminal
launch, but the original startup app still had no focused window. The
eight-second startup guard ended the session with `stage=not_focused`. Native
cleanup drained both heads with no abandoned scanouts or cleanup errors.

The frontend's FreePixmap path deleted every RENDER picture referencing that
pixmap. An existing regression asserted that behavior. It contradicts the
[X11 FreePixmap lifetime rule](https://www.x.org/docs/XProtocol/proto.pdf): freeing
the public identifier does not release storage still referenced by another
resource. The reference server's picture implementation increments the pixmap
reference when creating a picture and releases it when the picture is freed.
Kitty's headless probe reproduced failure before its required GLX stages.

FreePixmap now moves referenced backing to a private, non-wire-addressable key
and redirects its pictures to that one retained allocation. Reusing the freed
XID creates an independent pixmap. Existing pictures retain shared updates and
namespace checks; the final picture release drops the backing, including held
SHM mappings or DRI3 descriptors. Renderer registration release remains separate
from retained allocation lifetime; this adds no GPU sampling to software RENDER.
Window destruction keeps its separate picture-destruction semantics.

Replaced the incorrect regression with cursor/pixel, aliasing, XID reuse,
namespace rejection, and final-reference cleanup checks. Disconnect coverage
includes a surviving picture owned by another client in the same namespace,
both with and without an earlier explicit FreePixmap. Weak mapping references
check allocation retention and release. The real-client RENDER smoke now frees
its cursor pixmap before CreateCursor. It reports version 0.5, the expected
composited and glyph pixels, and zero errors. Kitty's patched headless probe
reaches one committed window with `first_error=none`; it does not prove physical
pixels or shortcut behavior. The startup guard and Hagia policy are unchanged.

The complete `cargo xtask check` passes on the main checkout with 2,500 passing
test executions, Clippy, source-layout and profile checks, retained archives,
verifier fixtures, and host buffer-age pixel equivalence. The gate used host
socket access. The repair is not installed; Kitty startup and Super+Enter still
need acceptance in a replacement physical session.

<!-- END IMPORTED BODY -->
