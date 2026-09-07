---
id: legacy-active-0498
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-21: signed physical head loss and return pass

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15221–15251. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- `/tmp/sophia-output-topology-20260821-233802.log` is the successful physical
  run from signed Sophia source
  `66bc0dd71a40e249eb00cd98f6080cf0f6aa9c54` and signed Hagia source
  `074e374c537b316b6bdf196ac8f3727004ba6549`. The one-shot gate built those
  clean checkouts as Sophia binary
  `f4a4b013d72203774b7e4ce0b616daab44df21b190211b2c6b35d121d7775d96`
  and Hagia binary
  `2d2440424094626e2c9df056a6badd8b1846d024552a1bb3a7c7040a8c684349`.
  The evidence SHA-256 is
  `619a8b692165ea4a88c9327546bc47e95232bd34b296c6eab611cc424e24e88b`.
- Far-left head loss advanced the input security epoch, drained scanout, and
  published topology epoch 2 / generation 2 with two outputs and
  `changed=true`. Hagia committed the matching policy projection; retirement 1
  supplied its later presentation and released input quarantine.
- Reconnection repeated the complete chain at topology epoch 3 / generation 3
  with all three outputs restored and `changed=true`. The second policy commit
  likewise forced a repaint, retired, and settled with input enabled.
- Kernel-monitor completion reports exactly two events observed and delivered,
  with none coalesced. Native completion selected the explicit
  `topology_replacement` profile at publication generation 3. The bounded
  session recorded 39 submissions, 36 retirements, zero submission or
  retirement failures, zero callback rejection or saturation, no WM restart or
  degradation, clean layout and topology health, and clean namespace/X
  authority teardown.
- The exact shell predicates passed. This promotes physical output loss and
  return and closes active critical-path step 1. Per-head mirror pacing is now
  the first open implementation step; it must rerun the affected physical gates
  before its own promotion.

<!-- END IMPORTED BODY -->
