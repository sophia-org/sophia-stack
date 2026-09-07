---
id: legacy-active-0583
date: 2026-09-01
recorded_date: 2026-09-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-09-01: latest-wins ends when native queueing acquires the frame

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18386–18442. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The physical terminal gate on signed commit
`59ea3b002f2df2a30974c325724b5c7969861842` passed visual confirmation and
proved the completion-clock repair: 2,391 native retirements completed with
zero overlap rejections, phase rejections, callback rejections, protocol
errors, or cleanup errors. The sole machine failure was
`presented_updates is not positive`.

The CPU evidence accounted all 7,107 accepted post-startup updates as
superseded. It recorded 3,204 compositions and native-target bindings, 1,192
primary retirements, 1,191 changed primary retirements, one lifecycle
supersession, zero pending updates, and zero presentations. Display cadence was
healthy at an 18.858 ms maximum gap. The screen changed continuously because
native frames really retired; only the logical presentation ledger had erased
their owners.

The ledger had one pending update and one checksum target. Each accepted update
latest-wins superseded that pending cell even after composition had queued its
frame into native scanout. Under sustained one-update-per-16-ms input, the next
accept normally arrived before the older frame's callback reduction. The
callback could name the displayed checksum but no longer the update it
belonged to. The previous gate's one reported presentation depended on an
occasional callback winning that race.

Production now publishes a passive `LiveProductionCpuTarget` containing the
exact `LiveProductionNativeFrameId` returned by native queue admission and
the logical checksum rendered into that frame. The session tracker separates
one unbound latest-wins update from up to 16 queued exact owners. New intake
settles only a prior unbound update. Retirement presents only an exact
frame-and-checksum match. Surface removal settles matching unbound and queued
owners through the lifecycle path. Reconciliation settles a queued frame as
superseded only after an allocation-free, mirror-aware native query confirms
that no deferred generation, head state, or presentation-group state owns it.
The owner loop reconciles immediately after production, during ordinary owner
turns, and after final native drain. Capacity exhaustion is fatal rather than
an unbounded allocation path.

External tracker regressions hold two exact frames across successor intake,
retire them independently, and require two presented updates with no
supersession. A complementary regression removes a queued frame from every
native owner and requires explicit supersession. The prepared-authority public
test surface remains unchanged; tracker-only queued targets stay internal.

`ContinuousContentPresentation.tla` now distinguishes the unbound cell from
composed, in-flight, and callback-owned generations. Multiple native owners may
coexist, while latest-wins is confined to the unbound cell. The
`NativeOwnersAreNotSuperseded` invariant rejects any settlement that leaves
the same generation in a native owner. The retained
`ContinuousContentPresentationNativeOwnerSupersession` control deliberately
restores the failed behavior and must violate that invariant. The positive
model passes 142 generated / 82 distinct states to depth 16, and the complete
pinned TLA+ corpus passes. The exact candidate also passes the canonical
`cargo xtask check` workspace, clippy, layout, archive, pixel-equivalence, and
offline-verifier gate. No timeout, synthetic presentation, or weaker reporter
threshold is part of the repair.

<!-- END IMPORTED BODY -->
