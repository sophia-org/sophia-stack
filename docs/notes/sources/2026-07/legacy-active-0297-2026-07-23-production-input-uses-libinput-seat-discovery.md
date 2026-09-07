---
id: legacy-active-0297
date: 2026-07-23
recorded_date: 2026-07-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-23: Production Input Uses Libinput Seat Discovery

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9223–9236. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The guarded launcher previously guessed one keyboard and the first distinct
mouse from `/dev/input/by-id` and `/dev/input/by-path`. That could not correctly
represent composite receivers, multiple keyboards, touchpads, or hotplug.

Backend-live now supports libinput's udev context and assigns `seat0` by
default. Device-added events classify keyboard, pointer, and touch
capabilities, apply tap-to-click where supported, and update bounded policy
evidence; device-removed events maintain active counts without exposing device
names or paths. Both the independent recovery guard and the Sophia session use
seat discovery. Explicit `--input-devices` remains mutually exclusive with
`--input-seat` and is retained only for deterministic hardware/QEMU proofs.

<!-- END IMPORTED BODY -->
