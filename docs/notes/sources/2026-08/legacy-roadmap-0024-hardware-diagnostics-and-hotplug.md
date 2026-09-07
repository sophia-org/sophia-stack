---
id: legacy-roadmap-0024
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Hardware Diagnostics And Hotplug

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 2643–2670.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0017-post-promotion-capability-roadmap.md).

<!-- BEGIN IMPORTED BODY -->

- [ ] Retain the exhaustive pc105 US shifted-punctuation and Ctrl-Alt-F1
  through Ctrl-Alt-F12 physical runner as a focused diagnostic. Repeat it after
  input/seat changes or for release burn-in; ordinary candidate promotion
  requires one real VT round-trip plus the deterministic XKB suite.
- [ ] After work-area, output, or seat changes, re-run the exhaustive xmobar
  reservation lifecycle and require no stale gap, overlap, resize timeout, or
  focus change. Pair dynamic output-topology behavior with the later physical
  multi-output hotplug gate. `tools/output_topology_physical_gate.sh` now arms
  that exact multi-output loss/return procedure and requires two input-epoch
  barriers, generation-advancing complete publications, policy settlement,
  later page flips, client survival, and clean final topology health. The
  recovery-safe one-command entry point is
  `tools/run_output_topology_gate_tty4.sh`; it supplies the routine arm, seat,
  matching signed Hagia and Sophia builds, and timestamped evidence defaults so
  the operator carries no shell state between TTY sessions. Failed attempts
  `/tmp/sophia-output-topology-20260821-231655.log` and
  `/tmp/sophia-output-topology-20260821-232830.log` exposed the missing-`udevd`
  rebroadcast and stale connector-cache dependencies respectively. Signed run
  `/tmp/sophia-output-topology-20260821-233802.log` passed the exact changed
  `3 -> 2 -> 3` publication, policy, presentation, completion, and clean-health
  predicates. The dynamic-output half is promoted; this checkbox remains open
  for the exhaustive xmobar reservation lifecycle after a relevant work-area,
  output-policy, or seat change, not for head-loss/return.

---

<!-- END IMPORTED BODY -->
