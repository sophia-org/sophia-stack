---
id: legacy-active-0610
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-09-04: software Present cannot reuse an identical retained scene

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19325–19357. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The short Firefox canary pinned to signed admission candidate `976e51d2`
proved the admission correction before exposing a separate presentation bug.
Firefox surface `4194307` received a 1266x1408 recovery extent, committed that
exact CPU admission, cleared the temporary extent, reached the loopback
page-ready marker, and rendered several 1266x1408 software-Present frames. The
owner loop then failed with `software Present cohort omitted its selected clock
output`. Bounded TTY and greetd recovery completed, but presentation cleanup
also reported retained surface-content ownership.

The software path had reused ordinary retained-scene projection queueing. That
queue is intentionally edge-triggered: if an equal logical checksum is already
pending, rendering, submitted, or displayed, it returns no new native frame.
That is valid for compositor projection but invalid for Present feedback. Every
accepted Present requires a distinct physical retirement on its selected CRTC
clock even when the pixels are unchanged. The selected output was therefore
correctly required by the feedback cohort but incorrectly removed by the scene
deduplicator.

Retained frame queueing now carries an explicit `LatestScene` or
`FreshRetirement` requirement. Ordinary projections retain their unchanged-scene
suppression; software Presents validate the complete output batch and force one
new retained-mixed native frame per logical output. The software staging path
also keeps its submission in the waiting queue until every fallible composition,
cohort, native-queue, and owner-collision check succeeds. A staging error is now
visible to the existing rejection path, which can settle feedback and release
the exact `SurfaceContentStream` owner instead of compounding the primary error
with cleanup debt. Deterministic coverage proves that identical pixels in every
owned frame phase still queue when a fresh retirement is required. A new signed
candidate must pass the same short physical Firefox canary before comparison
row collection resumes.

<!-- END IMPORTED BODY -->
