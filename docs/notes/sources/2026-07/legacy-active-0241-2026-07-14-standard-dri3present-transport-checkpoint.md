---
id: legacy-active-0241
date: 2026-07-14
recorded_date: 2026-07-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "tooling"]
---
# 2026-07-14: Standard DRI3/Present Transport Checkpoint

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8073–8091. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The X authority now negotiates the standard `DRI3` and `Present` extension
names at version 1.2 without routing them through the private
`SOPHIA-PRESENT` opcode. The Unix request reader uses `recvmsg` for the fixed
X11 header, captures up to four SCM_RIGHTS descriptors with close-on-exec set,
then completes the ordinary request payload read. Descriptor ownership is
RAII-bound and unexpected FD arity terminates the malformed connection instead
of leaking or guessing ownership.

The first admitted standard DMA-BUF request is DRI3 `PixmapFromBuffer`. Its
wire decoder preserves the pixmap/drawable and bounded storage metadata, marks
the request as requiring exactly one FD, accepts only 32-bpp XRGB8888/ARGB8888
shapes whose stride and declared storage cover the image, and records only the
authority-owned pixmap identity. The native FD remains borrowed through the
socket trace seam for renderer-side duplication and never enters authority
runtime state. DRI3 fences, PresentPixmap/events, and the live renderer handoff
remain the next transport checkpoint.

<!-- END IMPORTED BODY -->
