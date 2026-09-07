---
id: legacy-active-0557
date: 2026-08-29
recorded_date: 2026-08-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-29: the shared renderer worker closes on archive 0003

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17315–17352. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Two heads of one card ran on one renderer thread on real hardware:
`renderer_workers=1`, `worker_result_misroutes=0`, `worker_max_service_skew=1`
inside the one-per-sibling bound, `max_in_flight_per_output=2` for two
presented heads, and 207 worker requests settling as 207 completions with no
deferral, no stale release, and no slot leased at completion. Signed archive
`0003` verifies independently.

The first physical run of this row failed verification and was right to. It
reported no KMS submission ever in flight, of a session that had just
completed 207 renders and 136 partial repaints. The counter read
`scanout_submission`, which only the mirror path sets -- a group parks each
head's owner there until the cohort joins -- so it had been structurally zero
for every session that was not mirroring. Every QEMU run today reads zero for
the same reason, and the promoted archives are schema 7 and 8, which predate
the field, so the check guarding it had never met real evidence. The row's
central claim, one submission in flight per head, had therefore never actually
been measured on an extended desktop until this run produced evidence to
measure. It now reads the submitted sequence, set at submit and taken at
retirement on both paths.

That is the second instrumentation defect this row surfaced only under real
conditions. The first was the output key, unique by an argument about scope
until a two-card guest reported `head=1` for both of its outputs and disproved
it. Both were invisible to every offline gate, and both were caught by
evidence rather than by review.

Worth recording about method, since it cost time. QEMU was pushed too far
here before the deterministic tests were written. Its two-device topology
cannot express the row at all; forcing one card exposed that it enables only
the scanout its UI owns; and its GL backend segfaulted outright. It did
eventually give the one integration answer available without an operator --
two heads on one card, one thread -- but the routing invariants that needed
pinning belonged in tests that need no VM, and those should have come first.
Physical runs are cheap on this host and prove things the emulator cannot;
nothing on the critical path should wait on it.

<!-- END IMPORTED BODY -->
