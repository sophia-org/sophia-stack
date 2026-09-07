---
id: legacy-active-0190
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-26: Compositor Damage Follows KMS Retirement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6427–6453. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Display-list damage previously described the difference between two lists but
had no temporal identity in the production scanout path. Advancing a single
“last list” at composition time would be incorrect: latest-frame-wins queueing
can supersede pixels, native export can fail, and a submitted buffer remains
in flight until its page-flip callback.

Engine now owns a bounded per-output display-list presentation reducer with
pending, submitted, and presented slots. CPU and mixed CPU/DMA-BUF frames carry
the immutable list that generated their pixels. A queued list compares against
the submitted list when a flip is in flight and otherwise against the
presented list. Superseding or rejecting pending work cannot change the
presented baseline. Only an accepted KMS submit advances pending to submitted;
only the corresponding accepted callback advances submitted to presented.
Legacy frames without display-list identity explicitly clear pending chrome
state rather than inheriting an unrelated list.

The two-output QEMU xmonad gate passed this lifecycle. Both outputs established
empty initial baselines. Focus creation retired four border rectangles, focus
changes retired eight old/new rectangles, border removal retired four, and
stable client-only frames retired zero compositor rectangles. The gate now
rejects missing secondary-output initialization or missing nonzero retired
damage during the click-drag focus proof. Partial redraw, frame suppression,
and KMS damage-clip submission remain later scheduling optimizations; the new
ledger supplies the retirement-safe region those optimizations must consume.

<!-- END IMPORTED BODY -->
