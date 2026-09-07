---
id: legacy-active-0485
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: ece6b4f96f33092645d177a0c5c7b5f408f48f2e
committed_at: 2026-08-21T14:31:03-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# A measurement that could not have seen what it was cited for

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14751–14793. The heading has no date. Its first recorded addition is commit
`ece6b4f96f33092645d177a0c5c7b5f408f48f2e` (2026-08-21T14:31:03-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Before touching the shader I checked what the evidence would show, and it would
have shown nothing. Two sessions of this document, and a paragraph in
`docs/multi-monitor-composition.md`, said the linear-light correction was
measurable from the composition-region pixel populations the renderer already
reports. It was not, and the reason is written in the metric's own comment:
those buckets key on which channels are lit, deliberately, so that a diagnostic
palette stays legible across an intensity conversion while still exposing a
channel swap. Gamma changes intensity and nothing else. Every bucket would have
held still while every resampled pixel underneath it moved.

What remained was `checksum`, an FNV hash over all four bytes. It is maximally
sensitive and completely uninformative: it says the frame differs, never which
direction it went. Had the shader landed first, a wrong gamma and a right one
would have produced the same evidence -- a changed checksum and unmoved
populations -- and the only thing left to judge by would have been whether the
screen looked better to me, which is the impression the measurement existed to
replace.

`luminance_sum` and `luminance_buckets` were added ahead of the shader, alone,
so the baseline is banked with the old filter still in place. The weights are
integer and sum to exactly 256, so the shift is a division that never rounds and
two runs of one frame agree bit for bit the way the checksum beside them does; a
float luma would not have survived that comparison. The histogram is the metric
that judges the change and the sum is for a one-number comparison, because a
mean holds still while the population behind it splits, and a split is precisely
the shape gamma-space filtering makes: edge pixels piled into the low-mid
buckets instead of spread through the middle.

The prediction is recorded before the change rather than after: on a head that
downscales, mean luminance rises and the low-mid buckets depopulate toward the
middle; on a head sampling exactly one-to-one, every metric including the
checksum is byte-identical. That second head is a control the topology hands us
for free, and it is what makes the first head's movement mean something. The
alpha populations are a second control -- a correction to RGB must not move
them.

The rule this leaves behind: a claim that a change is measurable is itself a
claim to check, and the cheapest time to check it is before the change, when the
answer can still reorder the work. This one had been asserted in two documents
and repeated to the user before anyone asked the metric whether it could see.

<!-- END IMPORTED BODY -->
