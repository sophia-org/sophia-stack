---
id: legacy-active-0464
date: 2026-08-18
recorded_date: 2026-08-18
date_basis: first-heading-commit
date_commit: a37c945a5cc9464a885990f87f1ae49b6eb010db
committed_at: 2026-08-18T20:02:14-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# The pixel proof was spent before there were any pixels to prove

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14099–14126. The heading has no date. Its first recorded addition is commit
`a37c945a5cc9464a885990f87f1ae49b6eb010db` (2026-08-18T20:02:14-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

With stability narrowed to "this transaction was displayed carrying real
pixels", the next run still called every retirement superseded, and now said
why: `nonzero_rgb_pixels=0` on all of them. The presented content was right --
`MixedPresent { transaction: 493 }` on the head that flipped it -- so only the
pixel term was failing.

That term is not a measurement of the frame it is attached to. A renderer
context reads its composition back at most three times and keeps the last
result; a full-framebuffer readback is far too expensive to run per frame. The
log names all three: `sophia_native_composition_frame status=verified` appears
exactly three times per head, all of them in the first hundred milliseconds,
all of them `nonzero_rgb_pixels=0`. Every one measured a composition with zero
layers -- a clear to black, which cannot show anything by construction. The
budget was gone before the first client had drawn, and every present for the
rest of the session carried the zero those three attempts latched. No present
could be judged to have put light on a screen, so startup readiness was
unreachable no matter how well the pipeline ran.

The proof is now spent only where light could appear: an attempt requires a
composition with at least one layer. The budget is named
(`NATIVE_COMPOSITION_PIXEL_PROOF_ATTEMPTS`) rather than written as a literal in
three places, and the stamping site says what the value is -- the head's proof
that it has shown light, not this frame's pixel count. Which is what every
consumer wanted: readiness asks "has this client's content reached a screen",
and answers it with the displayed transaction plus that proof.

<!-- END IMPORTED BODY -->
