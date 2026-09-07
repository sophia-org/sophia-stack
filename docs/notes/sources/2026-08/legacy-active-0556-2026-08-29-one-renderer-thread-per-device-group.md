---
id: legacy-active-0556
date: 2026-08-29
recorded_date: 2026-08-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-29: one renderer thread per device group

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17258–17314. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The shared-worker row is implemented behind
`SOPHIA_ENABLE_SHARED_RENDERER_WORKER`. A native group is a card session,
which is exactly the DRM device group the row names, so the coalescing unit
already existed; what had to be built was everything a per-head worker got for
free by position.

The renderer context was the load-bearing part. It held one three-slot array
and one pixel-proof budget, which is right while a context serves one head and
catastrophic when it serves two: both outputs use slot indices zero through
two at their own sizes, so a shared array rebuilds a bundle on every
alternation -- the rebuild `target_recreations` counts and the gates require to
stay at zero. Slots and proof are now keyed per output, and the slot swap
carries the set it swaps within, because the inline target is all the render
path sees and proof recorded against whoever rendered last would report one
screen's pixels as another's.

Results are routed on a channel per output rather than correlated by position,
and the misroute check is kept anyway: a claim nobody verifies is not
evidence. Slots, damage, leases, and reusable buffers are per output; buffer
reuse matches on size alone, so a shared pool would hand one screen's content
to the other.

Service skew is measured rather than asserted, and honestly bounded: the
worker cannot see its own queue, so the figure is sampled on the tick and is a
lower bound that can miss a peak but never invent one. The verifier applies
the bound only where outputs actually share a thread, since independent
threads interleave as the GPU allows and bounding those would be a claim about
parallelism rather than about fairness.

Two findings came out of trying to exercise this headlessly. QEMU gives the
guest two GPU devices, so its outputs sit in two device groups and a shared
worker has nothing to coalesce -- `renderer_workers=2` under the flag is
correct there, not a bug. Putting both outputs on one device then exposed
that QEMU enables only the scanout its UI owns: `discovered=1` under both the
VNC and the egl-headless backends, and the latter segfaulted QEMU outright.
DRM's `e` mode suffix forces the second connector on regardless of detection,
which finally produced two heads on one card and the first proof of the row:
`renderer_workers=1` with zero misroutes, zero stale releases, and no target
recreations.

The same evidence caught the output key. Head identities repeat across cards
-- the two-card guest reports `head=1` for both outputs -- so a key built from
the head alone was unique only by an argument about scope. It now composes
group with head, and the worker refuses a duplicate registration rather than
trusting the argument.

QEMU's place here is worth stating plainly, because it was overreached
earlier in the day. It is the only proof available without an operator at a
console, and that is its whole value; it is not a proxy for hardware, its
timing figures are explicitly not claims, and two of its bounds had to be
widened today to accommodate the emulator rather than the product. The
routing invariants belong in deterministic tests that need no VM at all, and
now have them. Physical runs remain the only thing that promotes anything,
and they are cheap enough here that nothing should wait on the emulator.

<!-- END IMPORTED BODY -->
