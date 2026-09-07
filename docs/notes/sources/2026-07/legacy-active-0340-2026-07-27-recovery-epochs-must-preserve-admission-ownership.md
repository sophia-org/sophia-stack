---
id: legacy-active-0340
date: 2026-07-27
recorded_date: 2026-07-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "architecture"]
---
# 2026-07-27: recovery epochs must preserve admission ownership

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10674–10693. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first physical run after exact queued-Present ownership reached healthy
  KMS output and committed Kitty's recovery layout, but retired zero Kitty
  Presents and exited through the eight-second `not_focused` startup watchdog.
  This was a bounded session failure, not a renderer or kernel crash.
- The initial layout epoch timed out after the frontend had acknowledged
  admission, leaving the surface correctly in `AwaitingPixels`. The retry
  epoch staged and committed the retained pixels, but classified only a fresh
  `PolicyPending` surface as admission-owned. It therefore left both the
  transaction and its Present submission permanently in pre-admission
  quarantine.
- Surface-control staging now treats `ControlPending` and `AwaitingPixels` as
  continuing phases of the same admission. A retry may deliver the necessary
  configure, but its atomic commit also marks the surface managed and releases
  the retained transaction and Present exactly once.
- An all-feature regression reproduces an acknowledged admission entering a
  recovery transaction and requires the retry's finalization set, managed
  transition, transaction release, and Present release.

<!-- END IMPORTED BODY -->
