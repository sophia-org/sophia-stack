---
id: legacy-active-0491
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: c9a2dc800ae101686b79407ce65a853b8cea18f8
committed_at: 2026-08-21T20:09:29-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# Three places wrote it, one place read it, and that place was unreachable

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15024–15068. The heading has no date. Its first recorded addition is commit
`c9a2dc800ae101686b79407ce65a853b8cea18f8` (2026-08-21T20:09:29-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The reported symptom was window borders "messed up". The description was the
thing that cracked it: the far-left monitor showed only one border, which
belonged to a window the operator could see on the centre monitor, and the
far-right monitor -- the one actually holding a terminal -- showed no border at
all.

Both halves come from the same fact. Chrome here is a focus ring, drawn outside
the window geometry the way an X border_width is. The focused window's geometry
was exactly its whole output, so its ring lay entirely outside that output and
nothing of it survived on its own screen. Root space is continuous, so the ring's
left edge at root x=1918 fell inside the neighbouring mirror output's viewport,
and the mirror group drew that two-pixel sliver at its right edge -- on both
mirrored panels, since they show one logical output.

Why the window filled its output is the actual defect. Clearance is zero early in
the run and two from the moment a focus ring first appears, and the change is
supposed to raise `work_area_relayout_required` and force a relayout that insets
windows to make room. The flag was raised. Nothing read it. `poll_request`
checked it immediately after an early return into the public policy path, so the
check ran only for a private policy -- and the reference WM, which every physical
gate runs, is a public policy client. Three writers, one reader, and the reader
behind a branch that the sessions which matter always take.

`enqueue_relayout` opens with `if let Some(public) = self.public.as_mut()`. The
public case was written, and complete, and dead from that call site. That is the
shape worth remembering: not a missing capability, a capability the control flow
never reached, which reads as an unimplemented feature and diagnoses as a bug in
whatever it touched last.

Three attempts preceded this one and none of them were this. The first fixed a
real clipping defect that was not the reported one. The second corrected my own
explanation of why bands must be clipped individually. Both were spent inferring
geometry from composed rectangles, because window chrome had no telemetry of its
own and the rectangles it became were traced by the renderer, which is blind to
head identity by design and reports a rect two same-sized heads both produce.
Adding `sophia_live_head_border` turned the fourth attempt into reading a log.

The lesson is about where the time went rather than about the fix, which is one
statement moved above another. Three diagnoses stalled on a missing fact, and I
kept paying for a physical run to test a guess instead of paying once to make the
system able to answer. Evidence that names the thing you are debugging is cheaper
than the third guess about it.

<!-- END IMPORTED BODY -->
