---
id: urqkkdzp
date: 2026-09-09
kind: milestone
status: recorded
tags: [milestone, rendering, x11, validation]
---
# Exact copied Present evidence gates reallocation advice

## Result and boundary

This records an implementation checkpoint for the
[SuboptimalCopy obligations](../plans/6tewvlbh-universal-device-negotiation-across-sophia-clients.md#suboptimalcopy-gate),
based on signed checkpoint `cf2f070fd9b2c0d35dcbed10cda78e0424b9e070`.
The [installed XR24 comparison](../investigations/qfg6mjp2-atomic-test-history-is-not-alternate-layout-flip-proof.md#installed-framebuffer-comparison-on-99103568)
accepted the prerequisite proof. The installed `99103568` owner emitted ordinary
Copy during that run; it did not contain the signaling implemented here. AR24
remains unproven and cannot borrow XR24 evidence.

The frontend preserves effective permission in each accepted Present: Suboptimal
bit `0x8` must be set and ForceCopy bit `0x2` clear. The XLibre reference at
`Xext/present/present_vblank.c` likewise skips flip eligibility for ForceCopy.
No protocol options or application identities enter Engine. The existing exact
retirement comparison and current frontend state determine whether an actual
copied Complete qualifies. Retained and directly flipped buffers still report
Flip, without changing their release ordering.

One boolean in each accepted surface-preference row records whether advice was
claimed in that generation. Claiming occurs after the first Complete is accepted
and before subscriber routing, under the same locks as comparison. Duplicate
Complete, missing evidence and busy optional authority do not consume it.
Partial or failed subscriber delivery keeps it spent because another subscriber
may already have received the hint. This deliberately permits a missed optional
hint instead of repeated advice. Ordinary Copy leaves the claim intact; only an
accepted replacement preference generation resets it. Identical publisher
snapshots do not create new generations.

The session uses the frontend's effective mode for diagnostic records and
displayed-frame pacing. Ownership counters still describe the backend's actual
disposition. The diagnostic sanitizer admits only Copy, Flip, Skip and
SuboptimalCopy for that record. The bounded DRI3 probe can opt in and the evidence
reader can require the exact client-received SuboptimalCopy, recorded opt-in,
server mode and complete existing proof chain. The probe does not reallocate.

## Validation

Focused checks passed 23 frontend comparison/claim/real-wire tests, two session
feedback tests, one diagnostic-mode test, three probe CLI checks and fourteen
evidence-reader cases. They cover simultaneous transactions, cancellation,
invalid options, no permission, absent or stale proof, current surface lifetime,
generation replacement, both feedback orders, duplicate Complete, nonblocking
contention, full and partially delivered queues, and unchanged ownership clocks.
An independent read-only review found no concrete blocker and confirmed the
partial-delivery limitation above.

`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` passed with zero warnings
or failures. The run includes workspace tests, all-feature checks, the bounded
reader/probe regressions, twenty archived native-run fixtures, real offscreen
buffer-age equivalence and GLX/EGL first-frame/pixmap-export pixel checks.
Formatting, tracking links and all 74 task IDs were checked separately.

Artifacts are retained under `.artifacts/t071-present-advice/`. Its checkpoint
record identifies the signed implementation, validation logs and release binary.
Physical acceptance remains separate from these deterministic checks.

## Remaining gate

The [active tasks](../../../todo.md) retain their installed signaling gate. Run
the bounded opted-in probe after installing the candidate owner, retain exact
session identity and capture health, and join a received SuboptimalCopy to the
tested and retired source. Check all hints for no repeat per exact surface and
preference generation. Repeat the no-opt-in and no-proof controls. Neither this
checkpoint nor that narrow signaling test resolves the broader normal,
no-override client exit in t069 or the client-internal video selection in t068.
