---
id: legacy-active-0543
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-28: a CPU-backed client could not prove its own baseline

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16747–16841. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The physical input-latency harness could not start a measurement. Its session
reached full startup readiness in 111 ms and then waited forever: it never
emitted `sophia_live_session_input schema=1 status=ready source=physical`, so
injection never fired and every sample timed out.

The input proof arms on `input_baseline_is_presented`, which is
`focused_gpu_presented || cpu_baseline_presented`. The harness client is xterm,
which is CPU-backed, so only the second branch could ever answer. That branch
required the focused head's `presented_logical_checksum` to equal the CPU scene
report's `checksum`. Those are unrelated numbers. The head carries
`logical_scene_checksum` over engine surfaces, display-list commands, and the
cursor; the report carries a composition evidence hash over CPU output size,
flattened composited elements, and cursor float bits. They agree only on the
empty initial generation, by coincidence of a shared FNV basis, and never
again.

The equality was satisfiable when `eb30078d` introduced it on 2026-07-30,
because the CPU repaint then reached scanout through `queue_frame`, which
stamped the report checksum into the head. Per-head composition rerouted the
first call site in `f28685db` and removed the last one in `3614ecbd`, both on
2026-08-15/16. `queue_frame`, `queue_present_cpu_frame`, and
`queue_projected_frame` have had no callers since. From that day no live path
could stamp a CPU report checksum into a head, and the predicate became
unsatisfiable by construction rather than merely fragile.

Nothing caught it for two weeks. Every other physical gate runs GPU-backed
Kitty and answers the first branch of the disjunction, never consulting the
broken one. The single automated gate that asserts this exact line is the QEMU
session harness, and its last recorded run was 2026-07-31 — fifteen days before
the change that broke it. A gate that stops running stops protecting.

The baseline now asks two questions in submission counts, never checksums. The
focused surface's content must have crossed a page flip on every head it
occupies, which is the same barrier startup already uses and which fired
correctly in the failing log. And no head may have a submission outstanding
behind that: a frame still in flight would let the post-input correlation latch
onto a page flip that was carrying pre-input pixels, which is the fail-open the
original predicate existed to prevent. Both conditions hold in the failing
physical run and in QEMU, where every head submitted and retired every frame
before idling. Wiring software-Present retirements into the surface-keyed
evidence was rejected: xterm draws through core X, not the Present extension,
so that record never fires for the client that needs it.

Reviving the QEMU gate surfaced three more defects behind the first, each
hidden by the same silence.

Session completion refused a legitimately empty output. `independent_native_
output_presented` demanded `nonzero_exports > 0` from every head, and an output
holding no windows composes an all-black frame, which is the correct picture
for it and reports zero nonzero pixels. Under the shared-frame accounting that
predated per-head composition a blank head inherited nonzero-ness from the
whole desktop; per-head accounting reports the truth and the invariant became
unsatisfiable for a blank second monitor. Transport liveness is now asked of
every head and pixel evidence once of the session, matching the line this
codebase already draws elsewhere: pixel content is application-readiness
evidence, not transport liveness.

The persistent-evidence verifier did not know three fields. `present_complete_
routed`, `present_idle_routed`, and `present_route_failures` joined the
schema-16 session line on 2026-08-06; the verifier was last updated
2026-07-29. Both promoted native archives carry all three, so they are required
at schema 16 rather than tolerated.

The QEMU verifier still described the pre-per-head renderer. It required at
least two whole-frame CPU uploads, a mechanism per-head composition retired:
`native_frame_uploads` is zero in every post-per-head run, including archives
`0001` and `0002` that were already promoted on that behaviour. It also pinned
target and pipeline creations to exactly zero or two, one target per output,
where the archives now show 237 and 176 matched pairs. What survives is the
consistency claim — one pipeline per target — and the bound that targets are
not rebuilt behind the session's back.

Both relaxations are held by controls that name the message they must produce,
because a refusal that fires for the wrong reason proves nothing about the
clause under test. The first attempt at the desktop-wide pixel control was
vacuous exactly that way: zeroing the per-output counters also zeroed the
session counter, and the persistent verifier refused it first. The new pass
fixture is extracted from a real green run rather than written by hand, and its
second output is blank, so it is simultaneously the positive control for the
case the old per-output demand refused.

Two findings are recorded without being fixed. The three `queue_frame`
functions are dead code kept alive only by their own tests. And the pointer
proof's head-checksum branch in `lifecycle.rs` is unreachable:
`pointer_cursor_checksum` is read and cleared but never assigned, so live
pointer readiness comes entirely from the cursor-visible path.

`tools/run_sophia_input_latency_qemu.sh` now passes end to end on kernel
6.18.46_1: readiness, `text_match=true`, `pixel_change=true`, pointer select,
and a settled distribution of 7 samples at `p99_usec=148321` under TCG-paced
virtio. Those microseconds measure a virtual machine and are not a latency
claim; the physical p99 remains its own gate.

<!-- END IMPORTED BODY -->
