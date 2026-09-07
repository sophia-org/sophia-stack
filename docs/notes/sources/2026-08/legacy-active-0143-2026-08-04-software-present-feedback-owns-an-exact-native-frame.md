---
id: legacy-active-0143
date: 2026-08-04
recorded_date: 2026-08-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-04: software Present feedback owns an exact native frame

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4565–4617. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The live mixed-session trace disproved FIFO association between software
Present and native page flips. Software transaction 599 remained pending until
an unrelated Kitty DMA frame 704 retired; transaction 715 behaved the same way
behind frame 742. The runtime marked and settled the oldest software submission
on whichever native callback arrived next. CPU output work was also suppressed
while a GPU projection existed, so vkcube could advance a few frames only when
unrelated desktop damage happened to drive KMS.

Native frames now carry monotonic typed identities from queueing through submit
and page-flip retirement. A software Present first owns an immutable CPU or
retained-mixed frame, then only callbacks naming that frame may mark its
resources submitted or route clocked Copy/Idle feedback. Mixed owner batches
serialize an unrelated DMA frame and the software follow-up frame; the latter
uses the DMA transaction's prepared candidate so the new Vulkan surface cannot
disappear between flips. Same-owner coalescing excludes software Present work.

The deterministic reducer regression injects unrelated submission and
retirement observations and requires them to be no-ops. The physical verifier
joins every software retirement to its nonzero native frame and submission,
and `PresentFrameOwnership.tla` checks both the safety relation and eventual
retirement under weak fairness. The offline gates establish the lifecycle; a
fresh installed xmonad/vkcube run remains the physical acceptance boundary.

The first installed run exposed a legal callback/submission overlap in the new
guard. Retained software frame 30 retired while the same backend tick had
already submitted the next DMA Present as frame 31. The retirement finalizer
compared frame 30 with the scheduler's newer current frame and terminated the
session before settling transaction 699. Frame identity was correct; the
reducer had confused captured retirement ownership with current submission
state.

Retirement reduction now treats an exact CPU or retained frame as independent
of a newer submitted DMA frame, settles only bindings owned by the captured
frame, and leaves the successor scheduler entry intact. A `MixedPresent`
retirement without the matching scheduler frame still fails closed. The Rust
regression reproduces frames 30 and 31, and `PresentFrameOwnership.tla` now
allows successor submission between native retirement observation and feedback
settlement.

The installed `ad84d88a` rerun visibly animated vkcube and shut down cleanly
after 411 Copy completions, 21 Flip completions, and 435 Idle/fence signals,
with zero live Present resources, native failures, or protocol errors. The
first verifier result was a false failure: it enrolled startup Kitty DMA
transaction 410 and demanded software feedback records from it. The verifier
now enrolls only an armed admission with an exact schema-4 software retirement.
Its pass fixture includes the unrelated DMA admission and the legal sequence in
which frame 30 retires, DMA successor 31 submits, and frame 30 then settles. It
also rejects an unrelated-only log, a successor frame stolen for software
feedback, short animation, and insufficient diagnostic or aggregate feedback.
The corrected verifier passes the retained physical session.

<!-- END IMPORTED BODY -->
