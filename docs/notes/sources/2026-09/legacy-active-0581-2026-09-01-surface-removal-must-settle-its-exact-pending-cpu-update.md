---
id: legacy-active-0581
date: 2026-09-01
recorded_date: 2026-09-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-09-01: surface removal must settle its exact pending CPU update

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18270–18326. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The physical terminal gate on signed commit
`ad7e56eb65f6e5b78304e2bfa3f36ef3f88e3f78` displayed continuous scrolling
and passed operator visual confirmation. Machine completion still failed after
the bounded two-second quiescence interval. The final record was decisive:
`authority_pending=0 cpu_pending=1 native_pending=false`. X Authority had
drained in 14 ms and native presentation needed no further owner service, so
neither additional frontend pumping nor a larger timeout could settle the last
CPU update.

The CPU tracker previously inferred a target from the cycle report's logical
checksum after accepting a count of anonymous updates. During xterm teardown,
the accepted update belonged to a surface that was removed before another
logical CPU scene reached the primary head. The removal path queued
`RetainedMixed`; by contract, mixed and retained-mixed content have no logical
checksum. The pending update therefore outlived its owner and had no possible
retirement transition.

Production intake now preserves an update's exact transaction, surface, handle,
and generation identity. Progress is derived only from groups that pass the
ready/admission fence. The CPU and GPU paths publish a target only after they
successfully queue logical CPU or head-composition content for the primary
output. Retirement reads that presented content variant's own logical checksum
rather than a numeric head field that mixed content may retain from an older
scene; `MixedPresent` and `RetainedMixed` therefore cannot acquire or inherit
a logical target. A committed removal lifecycle-supersedes a pending update
only when the surface identity matches. Primary retirement occurs once at the
bottom of each owner-loop turn, so phase ordering cannot
retire or count the same update twice. The complete ledger remains bounded to
one pending owner and one pending target.

Telemetry advances CPU visual progress to schema 3 and terminal performance to
schema 6. It separately reports native-target bindings and lifecycle
supersessions. Quiescence schema 2 adds the exact pending transaction, surface,
handle, generation, and target checksum to completion and timeout records.
Historical progress schemas remain readable with zero identity-specific
counters; only schema 3 carries the new identity assertions.

`XAuthorityShutdown.tla` now includes surface lifetime and exact settlement on
removal. The positive model preserves bounded ownership and complete
accounting. A dedicated removal-without-settlement control must violate
`PendingHasLiveOwner`, ensuring the original teardown state remains
detectable. External Rust regressions cover same-cycle admission and removal,
unrelated removal, deferred work, logical versus mixed targets, latest-wins
supersession, mixed-content checksum isolation, and exactly-once retirement. No
timeout extension, dummy frame, idle-state clearing, or synthetic checksum is
part of the fix.

The canonical `cargo xtask check` gate passes with the complete workspace test
suite, all-target clippy, profile and layout checks, retained archive
verification, hardware buffer-age equivalence, and every offline verifier. The
complete pinned TLA+ corpus also passes. The updated positive shutdown model
generates 1,003 states / 456 distinct states to depth 19, and the new negative
control is accepted only when it violates `PendingHasLiveOwner`. One fresh
signed physical terminal gate remains required.

<!-- END IMPORTED BODY -->
