---
id: legacy-active-0482
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: b775612104feea0a33d4c3b53b7df34d9ef18e82
committed_at: 2026-08-21T11:40:40-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# What the mirror scaling choice actually is

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14657–14694. The heading has no date. Its first recorded addition is commit
`b775612104feea0a33d4c3b53b7df34d9ef18e82` (2026-08-21T11:40:40-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

A question about the fuzzy screen turned into the design space being written
down, so it is recorded here with the reasoning rather than only the outcome.

A mirror group has one logical size and places its members into it, so at most
one member is pixel-exact. That is not a Sophia limitation: X refuses to clone
unequal modes at all -- `xf86ModesEqual`, "different modes, can't clone" -- and
Windows Duplicate restricts the desktop to a mode every display supports.
Composing per head is what lets Sophia mirror a 1440p and a 1080p panel in the
first place, and the price of that ability is choosing who resamples. macOS
charges the same price and calls the choice "optimize for display"; its
letterboxing answers aspect-ratio mismatch, not resolution mismatch.

Three policies, not two. Optimizing for either member leaves the other
resampled; centring the smaller image inside a border on the larger head
resamples neither. Padding only works in that direction -- showing a 2560x1440
desktop unscaled on a 1080p panel would have to crop a quarter of the picture.

On FSR 1 and NIS: both are spatial upscalers, so they bear only on the
direction where content is smaller than the panel -- the group optimized for
its smaller member, where the larger head is currently on plain bilinear and a
1.33x upscale is well inside their range. Neither does anything for the default
configuration, where the smaller member downscales through a Catmull-Rom
bicubic that is already a decent kernel.

Which surfaced the finding that matters more than either: there is no sRGB
decode anywhere in the composition path. The bicubic weights gamma-encoded
bytes as though they were light, and the sampler sets no sRGB texture format.
Filtering in gamma space is the ordinary cause of muddy edges on resampled
text, it affects the configuration the gate runs by default, and it would
corrupt FSR or NIS exactly as thoroughly. The filter was never the first
problem.

Recorded in `docs/multi-monitor-composition.md` beside the per-head rules,
since it is a property of how a group is composed rather than a story about one
run.

<!-- END IMPORTED BODY -->
