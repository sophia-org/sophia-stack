---
id: legacy-active-0251
date: 2026-07-16
recorded_date: 2026-07-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "validation"]
---
# 2026-07-16: Sophia X TTY Recovery Is An Acceptance Gate

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8313–8328. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first GTK hardware attempt could leave the active text VT black until a
power cycle because the X proof called the raw persistent KMS runner without
the guarded TTY lifecycle already used by native Wayland. The GTK runner now
builds and preflights before takeover, requires an independent
Ctrl-Alt-Backspace guard to arm, saves KD and termios state, runs each Sophia
session in a bounded process group, restores keyd and the console on every exit
path, and records a strict durable recovery line.

The isolated QEMU emergency gate then exposed five modifier deliveries queued
before the final Backspace trigger. Emergency completion now waits for those
deliveries to flush before frontend teardown. The repeated gate proves guard
arm/trigger, exact five-of-five settlement, clean two-head KMS retirement, zero
native cleanup debt, and clean guest shutdown.

<!-- END IMPORTED BODY -->
