---
id: legacy-active-0551
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-08-28: readiness stops accepting a pre-content picture

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17071–17117. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The same staleness class a third time, and this instance was the latency.
The startup barrier pins its per-head submission requirement when the
focused surface first shows visual detail, and the input baseline reuses it.
In every latency session the pin happened while a render for the
pre-content composition was already in the head's pipeline; that render
finished into a submission that satisfied the count, readiness fired with
the pre-content picture on glass, and injection began mid-way through the
first contentful render -- the one that also pays the session's single
bounded pixel-proof capture, a full-frame readback and hash that measures
about forty-five milliseconds on this host. The keystroke then waited out a
render it should never have raced.

The requirement now pins a second quantity at the same moment: the newest
composition the head holds anywhere in its pipeline. A head that owes a
submission is presented only when its presented composition exceeds that
frame -- a picture planned at or after the content, not merely a flip that
happened after it. The pin runs in the authority phase, before the same
cycle's compose queues the first contentful plan, so the content frame
itself satisfies the requirement and only genuinely stale pictures are
refused. Heads the focused surface does not intersect owe no submission and
are exempt, because a blank output never advances its content and would
otherwise be waited on forever. Startup readiness and the input baseline
inherit the honesty together, and the pixel-proof capture lands where its
cost belongs: before readiness, outside anything a latency sample measures.

This supersedes the plan's original mechanism for the capture. Widening the
capture predicate could not work -- no layered composition exists before
client content, so the capture must land on a content render -- and the
asynchronous readback became unnecessary once readiness stopped inviting
input into the middle of it.

With the wait behind an earlier state's flip gone, the injector's burst
spacing lost its reason: zero-interval typing (c8bfc781) coalesced fourteen
events into one visual transaction to hide exactly that wait. Keys now space
at fifty milliseconds, so each press gets its own frame and vsync phase and
the distribution's two hundred forty-five samples are two hundred forty-five
measurements rather than one burst measured seven times per session.

Two QEMU verifier bounds moved for the same reason the numbers did: the
100 ms miscorrelation guards on the one-shot chain predated the honest
correlation, which under TCG includes the software render the input
actually waited on -- 173 ms in the passing run. Both are 400 ms guards
now, still miscorrelation checks, never latency claims; the physical gate
keeps the real budgets.

<!-- END IMPORTED BODY -->
