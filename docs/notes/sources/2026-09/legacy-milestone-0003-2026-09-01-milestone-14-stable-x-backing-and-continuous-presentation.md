---
id: legacy-milestone-0003
date: 2026-09-01
recorded_date: 2026-09-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-09-01 Milestone 14 Stable X Backing And Continuous Presentation

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 47–74.

<!-- BEGIN IMPORTED BODY -->

- [x] Replaced full immutable CPU presentation replacement for stable
  software-rendered X toplevels with lease-safe `Arc` copy-on-write backing,
  bounded damage history, and patch-preserving density derivation.
- [x] Restored sustained real-xterm progress by delivering core X `NoExpose`,
  bounding authority sequencing and shutdown, and requiring exact accepted-
  update ownership through surface removal, native queueing, and retirement.
- [x] Unified page-flip and out-fence completion under one bounded card pump
  and absolute monotonic clock, preserving exact logical retirement even when
  cadence validation rejects a sample.
- [x] Bound latest-wins to the unqueued CPU cell; queued native frames retain
  exact owners until presentation, lifecycle settlement, or proof that every
  native owner released the frame.
- [x] Passed the single-attempt physical terminal gate on signed commit
  `b9f0735ae3de0ab3f963fe19d6d117e0cbe6d403`: all 7,116 accepted
  post-startup updates were accounted, 1,190 were presented, 5,926 were
  superseded, none remained pending, and machine and visual verdicts passed.

Run `20260902T002500Z` retained a 16.586 ms maximum source gap, 18.825 ms
maximum display gap, and 31.737 ms maximum update-to-retirement latency on the
60 Hz physical workload, with clean authority, protocol, native-presentation,
and teardown state. This closes CP-14.1 only. The comparison and soak were the
planned exit evidence at that point; the explicit 2026-09-04 retargeting above
supersedes that execution requirement without changing this physical result.

---

<!-- END IMPORTED BODY -->
