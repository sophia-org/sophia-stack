---
id: legacy-active-0346
date: 2026-07-31
recorded_date: 2026-07-31
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-07-31: physical latency must begin from the current presented baseline

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10793–10808. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first complete 20-sample physical input-to-photon capture used kernel
  page-flip timestamps with zero fallback or pending correlation and clean
  teardown in every sample, but missed the sub-refresh gate at 18 ms p95
  against a 17 ms budget.
- Input readiness could use focused CPU visual detail or any earlier nonzero
  native frame. Several samples therefore injected while a newer pre-input
  focus/chrome frame was still queued, forcing the measured input frame to wait
  behind unrelated presentation work.
- CPU-backed proofs now require the current scene checksum to be the primary
  output's presented checksum before announcing readiness. Stable GPU content
  retains its independent surface-keyed presentation path, and headless CPU
  proofs remain valid without a native frame. A regression rejects a stale
  native checksum and zero-pixel or zero-export baselines.

<!-- END IMPORTED BODY -->
