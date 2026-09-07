---
id: legacy-active-0633
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-09-06 — Ghostty mask uploads and browser input failure containment

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20367–20410. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Super+Space dispatched Ghostty successfully at 19:26:44 UTC on `3fc0ab14d264`.
The process then exited before admission with X11 `BadWindow`, major 131,
minor 3: MIT-SHM PutImage. An isolated headless reproduction traced the rejection
to a 128×128 depth-1 ZPixmap mask. The shared-memory upload helper accepted
only depths 24 and 32, although setup advertised packed formats and the core
PutImage decoder already supported them.

MIT-SHM uploads now use that decoder before cropping into canonical pixels.
This retains the existing size bounds, attachment ownership checks, and GC
semantics while handling packed depths, scanline padding, and byte order.
The regression exercises depths 1, 4, 8, 16, 24, and 32 in both byte orders,
nonzero offsets and source crops, out-of-bounds reads, and foreign namespaces.
The patched headless Ghostty probe reaches a committed window and CPU pixels
with `first_error=none`; this is startup evidence, not physical rendering
acceptance.

The operator then reported a desktop crash while starting a Monkeytype test in
Brave. At 19:27:44 UTC, the session logged a fatal input-delivery `RouteRejected`
for client 4. The receipt lacked a rejection reason. Ordinary sessions now
retire rejected or failed client deliveries, release their pending barriers,
and log the failure instead of terminating the desktop. Failed deliveries never
count as flushed. Proof sessions retain strict failure behavior, and missing
receipts retain their timeout. Focus and namespace checks are unchanged.
Regressions cover both failure outcomes, duplicate receipts, and continued
delivery to another client. Route diagnostics record failure reasons without
key contents. The underlying Brave rejection still needs diagnosis if it recurs.

The later manual Brave transcript shows the configured unavailable D-Bus
endpoint (`unix:path=/dev/null`) and a VAAPI initialization warning. Its GPU
process errors follow Ctrl+C. Those messages do not establish the cause of the
earlier session failure; neither bus policy nor GPU rendering was changed here.
These repairs were tested in a separate checkout based on `3fc0ab14d264` while
another agent developed RENDER support.

The complete `cargo xtask check` passes in that checkout: 2,478 passing test
executions, Clippy, profile checks, retained archive verification, verifier
fixtures, and host buffer-age pixel equivalence. The temporary checkout needed
safe configuration-fixture modes, its shared build-directory path, and explicit
Hagia/Narthex checkout paths. The fixes are copied into the main checkout;
concurrent RENDER edits are preserved. Installation and physical acceptance
remain pending.

<!-- END IMPORTED BODY -->
