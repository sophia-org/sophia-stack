---
id: legacy-milestone-0016
date: 2026-07-14
recorded_date: 2026-07-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-07-14 X11 Session Correctness

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 367–388.

<!-- BEGIN IMPORTED BODY -->

- [x] Implemented the XKB keymap/state path and bounded X11 focus, active and
  passive grabs, keyboard, pointer, and required XI2 delivery while preserving
  Engine-selected targets and authority-local coordinates.
- [x] Replaced fixed root/output facts with Engine-sourced topology and RandR
  snapshots, including authenticated update notification and normal
  configure-plus-pixels resize behavior.
- [x] Preserved client-targeted input/control acknowledgements, bounded
  backpressure, deterministic focus/stacking, and complete disconnect cleanup.
- [x] Retained and strictly verified the paired native two-xterm session on the
  X13 under classic shared-X and a fresh zero-capability confined namespace.

Both schema-13 runs used physical evdev keyboard and pointer input, flushed all
routed X11 events, retained two independently interactive CPU layers, applied
the Engine output update with four RandR notifications, committed resized
pixels, presented through Engine-owned KMS, and drained without cleanup debt.
Classic startup readiness was 94 ms and confined readiness was 90 ms; both had
13 ms maximum CPU composition and 0 ms measured input-to-presentation latency.
The retained ignored logs are
`.evidence/remote-target/tmp/sophia-milestone3-{classic,confined}.log`.

<!-- END IMPORTED BODY -->
