---
id: legacy-active-0256
date: 2026-07-17
recorded_date: 2026-07-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-17: GTK Submit Deadlock Removed

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8410–8431. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

A fresh local QEMU run reproduced the apparent post-submit blank screen after
Zenity had accepted exact physical text and pointer input. The client process
exited, but the CLI session loop synchronously called `read_to_end` on its
piped stdout. An inherited writer could therefore block the visual coordinator
forever, bypassing the 30-second session deadline and preventing native
retirement and console recovery. Application stdout now targets a private
mode-0700 capture directory and mode-0600 file. Once the child exits, the loop
reads at most 4,097 bytes from the regular file without waiting for every
inherited descriptor to close; a regression keeps a writer open while proving
the bounded read completes.

The rebuilt X13-hosted QEMU image passed both GTK profiles. Classic completed
in 4,617 ms and confined in 4,633 ms; both matched `sophia\n`, routed a
physical pointer button, reported `first_error=none`, retired both virtio-gpu
outputs, and ended with zero native cleanup debt. The initramfs builder also
requires xterm explicitly now: it can no longer silently produce a nominally
successful image whose default session scenario fails at boot-time readiness.
The guarded physical X13 resize captures remain the Milestone 5 promotion gate.


<!-- END IMPORTED BODY -->
