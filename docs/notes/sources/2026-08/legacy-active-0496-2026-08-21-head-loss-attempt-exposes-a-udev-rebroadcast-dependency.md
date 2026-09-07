---
id: legacy-active-0496
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-21: head-loss attempt exposes a udev rebroadcast dependency

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15183–15199. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- `/tmp/sophia-output-topology-20260821-231655.log` began with three connected
  heads, but unplugging and reconnecting the far-left physical monitor produced
  no topology notice, input-epoch barrier, or publication. The session retained
  its stale three-head scanout until the physical-input deadline and then
  completed bounded cleanup. This is failed evidence, not a promoted gate.
- The host has a populated udev database and `/run/udev/control`, so startup
  enumeration succeeds, but no `udevd` process is running. The monitor used
  `MonitorBuilder::new()`, which listens to the daemon's userspace rebroadcast
  group; the kernel connector event therefore had no producer on that channel.
- DRM connector hotplug is kernel authority, so the monitor now uses the udev
  crate's direct kernel-netlink source while retaining its DRM subsystem,
  `change`, and `HOTPLUG=1` filters plus capacity-one coalescing. Completion
  telemetry records kernel events observed, coalesced, and delivered. A new
  physical `3 -> 2 -> 3` run is still required before promotion.

<!-- END IMPORTED BODY -->
