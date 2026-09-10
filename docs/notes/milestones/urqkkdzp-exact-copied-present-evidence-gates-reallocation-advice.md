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

## Installed acceptance on 2026-09-10

Installed `fb8fe8be7960bbe0939f828dfa20d7a98eba00b9` ran in session
`00000001789034825810-f9a9bacf-fa94-4c0c-872b-de815cacae6c`, owner 1750,
start tick 51671773. Its binary SHA-256 was
`d67506e3520cda22fa69e155d293cd9972bfd5a8a90edf026a56eb494961c735`, matching
the validated release. Both direct-scanout and visual-progress gates were enabled.

Five bounded XR24 runs completed 120 Presents each:

| Control | Received mode | Idle before teardown |
| --- | --- | --- |
| Opted-in compressed allocation, isolated output | 1 SuboptimalCopy, 119 Copy | 120 |
| Same compressed layout without opt-in | 120 Copy | 120 |
| Opted-in LINEAR, isolated output | 120 Flip | 119 |
| Opted-in compressed layout with ForceCopy | 120 Copy | 120 |
| Opted-in compressed layout, small ineligible window | 120 Copy | 120 |

The hint at client serial 58, transaction 67158, UST 516920236091 and MSC
30943943 joins an actual framebuffer-stage refusal, successful alternative
TEST_ONLY, exact copied retirement and current preference match. It belongs to
surface 65011713, output 2, native generation 13 and preference generation 13.
Later transaction 67708, serial 104, independently has all the same qualifying
stages and generations but receives ordinary Copy. This proves suppression
despite another qualifying candidate, rather than merely no further proof.

No-opt-in transactions 137623 and 138210 and ForceCopy transactions 140420 and
140952 also retain full qualifying proof chains while receiving Copy. The
ForceCopy executable was built from the retained probe with only the Copy option
bit added and an explicit log marker; its source and binary identities are saved.
The LINEAR control's first transaction 138584 includes the direct-client atomic
test record, beyond the wire Flip mode. Its final retained buffer is released at
connection teardown. The small control has no layout-comparison records.

The independent reviewer verified both positive joins, same-generation suppression
and the three isolated negative controls. The small control was checked separately.
All five saved capture intervals are contiguous. Health sequence 233553 covers
every run with zero discarded records, rotation or storage errors. Later rotation
occurred after the bounded logs had been copied. All owned probe windows were
gone at the final check, the original owner remained alive, and temporarily hidden
panel/browser windows were restored without input injection or process restart.

Evidence is retained under `.artifacts/t071-live-fb8fe8be/`, including identities,
invocations, bounded logs, exact-transaction reader outputs and
`acceptance-summary.json`. This accepts the narrow installed t071–t074 signaling
gate. It does not establish AR24 advice or physical screenshot correctness.

The [active queue](../../../todo.md) retains the broader normal, no-override
client exit in t069 and client-internal video selection in t068. The user's
Super+B browser in this session still carried a render-node override; a live GPU
process under that override cannot satisfy the no-override exit.
