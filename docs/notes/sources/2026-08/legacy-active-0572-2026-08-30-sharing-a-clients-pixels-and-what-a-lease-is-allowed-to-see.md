---
id: legacy-active-0572
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-30: sharing a client's pixels, and what a lease is allowed to see

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17896–17977. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

CP-14.1 replaces full immutable CPU presentation replacement for stable
software-rendered X toplevels. The path copied four times per update: the
authority cloned its snapshot into the transport, the session cloned it again
into the registry, the scene cloned it per density variant, and the backend
cloned it once more per head. For a 1080p toplevel with a few kilobytes of real
damage that is tens of megabytes per second per head, and every copy bought one
property -- a presentation handed a buffer keeps reading what it was handed.

Copy-on-write buys that property directly, so the choice was between it and
per-handle `(generation, bytes)` history rings. The rings were rejected before
any code: they change buffer identity from a handle to a `(handle, generation)`
pair across head composition, the present scheduler's residency roots, and the
pinned exact-handle-no-fallback regression, and they need an eviction policy
that collides with the fail-closed missing-buffer accounting. The wire already
is a damage-generation stream -- `PatchBatch` carries `(handle, generation,
patches)` and the registry refuses stale generations -- so what was missing was
senders off the happy path and a store whose in-place mutation cannot corrupt
leased history. `Arc::make_mut` is exactly that, and it reuses the
`Arc::strong_count == 1` lease test the output-frame path already used.

`StableBackingLease.tla` came first, because this is an ownership change. It is
the same shape as `VisualDamageHistory` one level down and one property further,
and it is a separate model rather than an extension because that model's stated
exclusion -- no lease incarnation, since a slot's buffer keeps its content
across release -- is load-bearing there and false here. Six negative controls
each produced a counterexample, including the reachability control proving a
split can actually happen, so the model is not vacuously safe. Two of them
corrected a guess made before they were run: reclaiming a still-read allocation
trips that allocation's liveness before it trips the allocation bound, which is
the more specific counterexample.

Four things changed in the code. The authority stopped building a patch and
cloning a snapshot for callers that read only the handle -- but kept the
validation that was hiding inside the discarded patch, which is what makes a bad
damage rectangle `InvalidResource` rather than a commit that touched nothing.
The bytes became `Arc`-backed end to end. A damage list longer than the
transport's thirty-two rectangles is now coalesced rather than triggering a full
replacement, which was the browser case: the client with the largest buffers and
the longest damage lists was the one whose window got resent. And a derived
density variant is patched by the draw it replays, where it used to publish its
whole buffer per command per density.

Two decisions inside that are worth keeping. The coalescer over-approximates on
purpose: patches are read from the presentation buffer after the client's pixels
were composed into it, so a merged rectangle carries more already-correct bytes,
while a cover short of the damage leaves a region stale in a frame that is
otherwise presentable, self-consistent, and correctly generation-ordered. And
`apply_command` reports where it painted from beside each branch's own
projection rather than from a second function reading the same command, because
an extent calculation that disagreed with the paint would under-report by
exactly the region it forgot.

The transport bound did not move. Thirty-two is validated identically on both
sides of the wire, so raising it would have to move the encoder, both guards,
and the renderer's capacity refusal together.

First real numbers, from the headless session gate on this change: 31 of 32 CPU
updates were patches, `cpu_cow_splits=13`, and the registry peaked at one buffer
and 1,366,912 bytes. Thirteen splits against thirty-one patches says
presentations do hold buffers when updates arrive in this workload, so the
copies are real -- and they are thirteen copies where the old path made
thirty-one replacements plus a transport clone plus a clone per variant per
head.

The milestone's other unmeasured clause is closed alongside it. "No steady-state
allocation growth" had no instrumentation at all: every resource figure is
emitted once at completion, and one record cannot tell a session that leaked a
buffer a minute for two hours and freed them at teardown from one that never
held more than three. The session now samples the same gauges every five
seconds, bounded at an hour and saying so when it stops, and the verifier
compares the run against itself -- warmup quarter dropped, then the later half's
peak against the earlier half's. Only resident size carries a tolerance, because
glibc returns freed memory to its arenas rather than to the kernel; the
accounted gauges carry none.

A session too short to sample fails rather than passing quietly, which is the
deliberate half of that rule: the headless gate runs three hundred ticks and
records `samples=0`, and a run that cannot show a settled population cannot make
the claim the milestone exits on.

<!-- END IMPORTED BODY -->
