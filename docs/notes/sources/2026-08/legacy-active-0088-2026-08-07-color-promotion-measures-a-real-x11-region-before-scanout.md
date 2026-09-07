---
id: legacy-active-0088
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-07: Color promotion measures a real X11 region before scanout

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2962–2991. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The physical TrueColor gate cannot rely on visual inspection or a screenshot
whose capture path is unrelated to the frame sent to KMS. The installed proof
therefore starts two ordinary clients. A small X11 client in the packaged
Sophia executable validates fixed-colormap allocation and query behavior,
draws an asymmetric RGB/CMY/gray palette through core `PutImage`, and requires
an exact `GetImage` round trip. A normal packaged Kitty independently renders
a 24-bit ANSI sample through its DRI3 DMA-BUF path.

The native renderer's opt-in composition trace now reads both the complete
framebuffer and the exact rectangle just drawn. Generic channel-population
metrics distinguish red, green, blue, yellow, cyan, magenta, gray, and other
pixels without learning X11 identities or application metadata. Unequal bar
widths make every expected population unique, so a channel swap, collapse, or
contamination fails deterministically. The palette and Kitty stay inside the
implemented primary-output projection, and each final rectangle must precede
a matching output-1 submission and KMS retirement. Output 2 independently
retains its nonzero startup baseline. The proof repeats only final-region
readback; it does not enable the older full-frame-after-every-layer diagnostic.
Ordinary sessions keep the previous cost and privacy boundary.

The same work closes the focused xmobar gate without repeating a physical
sequence. Checksummed xterm attempt `0003` already contains one exact 14-pixel
reservation on each output, ten exact 2560-by-14 primary repaints, fourteen
primary retirements, packaged xmobar identity, normal logout, and clean
recovery. A new archive verifier binds those facts to the existing immutable
record, while mutation fixtures reject a wrong repaint extent or unreduced work
area.

<!-- END IMPORTED BODY -->
