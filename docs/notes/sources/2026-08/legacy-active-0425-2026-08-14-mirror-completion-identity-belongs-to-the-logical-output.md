---
id: legacy-active-0425
date: 2026-08-14
recorded_date: 2026-08-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "tooling"]
---
# 2026-08-14: mirror completion identity belongs to the logical output

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12873–12887. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Physical mirror attempt `0005` on signed source `265d94dc` exercised the
  lossless authority policy under an `ll` burst: 891 batches entered Engine with
  zero drops, bounded waits resumed in one millisecond, both connectors joined
  through frame 8, and native teardown drained with zero abandoned scanouts.
- The remaining exit 1 came after clean frontend join from a completion check
  that required every physical head checksum to differ. That invariant belongs
  to independent logical outputs. Mirror heads intentionally carry the same
  logical scene checksum even when their native-size scanout buffers differ.
- Completion now requires equality among heads sharing an `OutputId` and
  uniqueness between distinct `OutputId` values. The physical verifier locks
  the same contract while connector-qualified submit, callback, and retirement
  records remain the evidence that each physical chain presented independently.

<!-- END IMPORTED BODY -->
