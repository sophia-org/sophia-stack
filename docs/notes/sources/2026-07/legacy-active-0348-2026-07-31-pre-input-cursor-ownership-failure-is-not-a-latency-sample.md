---
id: legacy-active-0348
date: 2026-07-31
recorded_date: 2026-07-31
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation", "architecture"]
---
# 2026-07-31: pre-input cursor ownership failure is not a latency sample

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10831–10846. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first damage-reuse physical rerun completed 16 independent input proofs
  with clean renderer/KMS teardown and a 2 ms maximum native upload. Sample 17
  exited before physical-input readiness or injection because the initial
  cursor-plane atomic update returned `EACCES`.
- No uinput trigger, injector timestamp, or latency record existed, so the
  failure did not describe input-to-photon performance. Treating it as a sample
  would conflate transient session startup ownership with the measured path.
- The physical runner now makes at most three session-start attempts while
  retaining the same uninjected uinput device. A retry requires the exact
  cursor `EACCES`, no physical-input readiness, no trigger or injector result,
  and no completed latency record. The rejected attempt log is retained beside
  the eventual sample. Any other failure, any failure after readiness, or
  exhaustion of the bound still stops the gate.

<!-- END IMPORTED BODY -->
