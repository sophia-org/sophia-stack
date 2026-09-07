---
id: legacy-active-0489
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: 4009ab70a004ccc4c640518816b62d5fd0b99ffe
committed_at: 2026-08-21T15:28:18-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# The correction, measured, and the region that could not be read

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14935–14979. The heading has no date. Its first recorded addition is commit
`4009ab70a004ccc4c640518816b62d5fd0b99ffe` (2026-08-21T15:28:18-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The physical result first. On the two regions where the two runs put comparable
content on screen -- lit populations of 8882 against 8884, and 4722 against 4724,
which is the same frame to within two pixels -- the edge centroid rose from 8494
to 9079 and from 7333 to 8410, seven and fifteen percent. The histogram shape is
the one predicted before the change: the dim buckets halve, the bright ones gain
by a factor of three, and the top bucket is populated for the first time at all.
The controls held exactly: the `exact_nearest` draw stayed at 12000 and the
uniform solid bands at 10000, both to the digit. Both gate modes passed with
`linear_fallback_draws=0`, so the shader was genuinely running, in both
directions.

The part worth recording is that the first region I read said the opposite, and
said it convincingly. Records one through twelve of the 1920x1080 region were
byte-identical between the runs -- not close, identical -- which reads as a
correction that never reached the draw.

It was the measurement. The composition-region trace carries a rect and no
output identity, and this topology has two heads composing a `1920x1080_0_0`
rect: the extended head at `mapping=exact` and the mirror member at
`mapping=fit`. One is filtered and one is not, their records are
indistinguishable, and the collector's last-record-wins kept whichever came
last. The identical histograms were the unfiltered head being compared with
itself, which is exactly what should happen and proves nothing about the change.

I had found this in the reading before any of it was written -- the note said the
region line carries no `head=` or `output=` field -- and then designed a
comparison keyed on the rect anyway. It is the third instance this session of one
record lacking the fact needed to read it, after the raster that could not say
what extent it spanned and the counter that could not say whether a draw was
degraded or enlarged.

Two smaller things the run corrected. The evidence pair was not comparable to
begin with: the second run emitted twelve region records to the first's nineteen
and never reached the frame with the full directory listing, so the headline
region was comparing unlike content on top of everything else. And the solid
bands are not the control I called them -- the trace reads the final framebuffer
at that rect, not the clear that was written there, so anything composed nearby
contributes. The controls that did hold, and that were worth having, were the
exact-sampled draw and the uniform bands.

The conclusion stands on the regions whose geometry only one head has. That is
luck, not method, and the method is the follow-up.

<!-- END IMPORTED BODY -->
