---
id: legacy-active-0562
date: 2026-08-29
recorded_date: 2026-08-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-29: proving a frame needs no composition

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17460–17508. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Direct scanout's model and Engine proof are in. `PresentFlipOwnership.tla` is
the complement of `PresentCopyOwnership`: that one owns the composited path,
where a compositor snapshot reaches glass and the client source goes idle at
the flip, and this one owns the case where nothing is copied and the client's
own buffer is displayed. Three consequences follow from that inversion and are
what the module forbids getting wrong -- the buffer cannot be released at
submission or at the flip, its release waits for a successor's retirement, and
that successor may be a composed frame, because an overlay activating makes
the next frame ineligible while the direct frame is still on glass. Returning
to composition is a successor, not an eviction.

The test stamp was unfalsifiable in the first draft. Engine proves every frame
while the backend tests only on the eligibility edge, so a frame can carry a
fresh proof over a stale test -- and without an action expressing that
asymmetry, the proof stamp blocked every path the test stamp's control would
have used. Adding `ReProveAfterEpisodeChange` made the control fail as it
should and made the model match the implementation it describes. The
`~effectActive` conjunct on the flip is the opposite case: provably
unreachable given the episode stamps, kept and documented rather than deleted,
so a later decoupling of episode from activation cannot quietly make a flip
during an effect lawful.

Engine's verdict is a reason rather than a boolean, because the reason is what
evidence records and what an operator reads when a frame that looked eligible
was composed instead. It is computed from the finished plan and stored on it:
eligibility belongs to the exact frame that reaches the screen. The chrome
check classifies compositor commands exhaustively and without a wildcard, so a
primitive added later must be classified deliberately -- until it is, the
compiler stops the change rather than letting an unconsidered primitive ride
along invisibly on someone's screen. That is the rule scanout cloning already
states for plan fields, applied to commands.

One check is unreachable and says so: the letterbox fill can never be the
verdict, because letterboxing means the scene is smaller than the head, so the
layer inside it cannot cover the head either and the geometry check answers
first with something more precise. The test that covers it asserts the precise
verdict and separately asserts the letterbox rects are present, rather than
accepting either answer -- an earlier version accepted both and its mutation
went uncaught, which is exactly how a check that never runs looks healthy.

Worth stating for the row's own sake: under the current Hagia profile the
indicator strip is drawn above fullscreen windows deliberately, and the
physical guide asserts it stays visible. Such a frame is composed. Direct
scanout will therefore not engage in an ordinary session until that policy
changes, which is a product decision rather than a limit of this work; the
promotion gate proves the row with a strip-free profile.

<!-- END IMPORTED BODY -->
