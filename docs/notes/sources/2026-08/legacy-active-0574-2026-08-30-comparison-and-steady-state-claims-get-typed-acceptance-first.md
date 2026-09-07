---
id: legacy-active-0574
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-30: comparison and steady-state claims get typed acceptance first

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18038–18065. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The two-hour resource series used to be optional in the generic soak verifier,
which made absence indistinguishable from a flat population. Direct verification
now defaults to `current`: exactly one nonsaturated series, at least 120 samples,
an exact declared count, contiguous sequence identities, advancing uptime, and
flat settled resource peaks are mandatory. Accounted buffers, bytes, slots,
snapshots, and imported-image cache entries have no growth tolerance; RSS alone
gets 64 MiB for allocator arenas. The immutable installed archive wrapper passes
`archive` explicitly, preserving older evidence while applying every current
rule to any series it does contain.

The same-hardware comparison now has one typed owner in
`sophia-conformance`, surfaced as
`cargo xtask conformance desktop-comparison`. A clean signed preparation hashes
the three repository stack profiles and local Firefox input, records the common
topology and hardware/software identities, and creates a rotated schedule. Raw
adapters are bound one at a time only after prior checksums and the new typed
sample pass. Final verification requires 36 short captures and three two-hour
soaks; reporting emits diagnostic means with `verdict=none`, so a reference
desktop never becomes Sophia's correctness oracle.

Mutation tests retain incomplete schedules, modified raw logs, identity/backend
mismatch, crashes, sample loss, missing samples, short or saturated series, every
steady-state gauge growing, and the old startup-only terminal trace. These are
acceptance implementations, not observations. No comparison sample or fresh
two-hour current soak was captured in this non-TTY implementation session.

<!-- END IMPORTED BODY -->
