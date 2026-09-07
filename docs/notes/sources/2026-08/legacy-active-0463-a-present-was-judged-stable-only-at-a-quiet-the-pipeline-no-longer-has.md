---
id: legacy-active-0463
date: 2026-08-18
recorded_date: 2026-08-18
date_basis: first-heading-commit
date_commit: 2ccdc31c24e4e60b81f96094c62272c686dd4d83
committed_at: 2026-08-18T08:03:29-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# A present was judged stable only at a quiet the pipeline no longer has

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14042–14098. The heading has no date. Its first recorded addition is commit
`2ccdc31c24e4e60b81f96094c62272c686dd4d83` (2026-08-18T08:03:29-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The mixed-output gate reached the end of the topology lifecycle -- committed,
policy-committed, settled, input restored, no layout timeout, both surfaces
drawing -- and then failed with "persistent live session never reached startup
readiness". Eleven presents retired over the run and every one of them was
reported `superseded`. None was reported `stable`, which is the evidence
readiness waits for.

The predicate asked for three things: that the displayed content name this
transaction and carry nonzero pixels, that nothing newer be submitted, and that
no head be busy with an exporter frame, a prepared scanout, or a scanout
submission. Only the first is about the flip being judged. The other two are
about the instant of judgement, and that instant is chosen badly: stability is
evaluated after a whole `service_native` pass, and the pass that polls a
retirement goes on to submit the successor before it returns. For a mirror
group it is worse still -- retirement itself promotes the coalesced successor
into every head's exporter slot, so the third condition is falsified by the very
retirement being reported. A pipeline that is flowing can satisfy the predicate
only in the gaps between frames, and two kitty clients with the owner pacing at
1 ms under pointer motion leave no such gap for anyone to observe.

The first explanation offered here was wrong, and worth recording as wrong: I
assumed the strict form had been satisfiable before because the frame reducer's
primary reservation kept things quiet. It did not keep anything quiet -- it kept
the exporter slot permanently full, which is precisely the arbitration deadlock
described above. The strict predicate passed in old runs only at genuinely idle
instants, when no present was runnable and nothing composed across the flip.
Fixing the deadlock did not break stability detection; it removed the stall
that had been masquerading as it.

The conjuncts were never argued for. `e2cfe3ac` introduced "no newer primary
frame queued or submitted" when the reducer's reservation made the clause nearly
free, and the two busy terms arrived later under one-line messages with no
recorded rationale. Nothing downstream wants them: startup evidence, the
readiness gate, and the input-pixel proof all want "this transaction's pixels
reached the screen", and the input proof in particular means "the post-input
transaction was displayed", not "the compositor then stopped". The model was
already the looser of the two -- `PresentFrameOwnership.tla` explicitly permits
a successor frame submitted after a Present's retirement is observed, requiring
only that the successor cannot steal or block the captured retirement. The code
was stricter than the specification it is meant to implement.

So a present is stably presented when its page flip retired with it as the
displayed content carrying real pixels, and successors do not unsettle it.
Evaluation can stay where it is because the successor's own flip callback
cannot arrive inside the pass that submitted it -- events are polled at pass
start -- so the displayed content still names the retired transaction when the
question is asked. The two conditions that raced are the two that are gone.

Starvation was ruled out before the predicate was touched: the eleven flips
were spread across the run rather than clustered at its start, so presents were
reaching the kernel throughout. The telemetry line went to `schema=2` at the
same time. Its `pending_primary` field was `!stable` written twice under a name
that suggested an independent fact; it is replaced by the pixel count, which is
what actually distinguishes "shown, but blank" from "not shown".

<!-- END IMPORTED BODY -->
