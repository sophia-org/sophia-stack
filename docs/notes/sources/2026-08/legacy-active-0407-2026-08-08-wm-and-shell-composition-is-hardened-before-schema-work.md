---
id: legacy-active-0407
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy", "shell"]
---
# 2026-08-08: WM and shell composition is hardened before schema work

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12416–12451. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Separate role sockets and exact UID/PID admission authenticate clients but do
  not preserve WM blindness when one process can combine WM geometry/focus
  authority with shell or broker metadata. The target contract now names the
  protection domain as the security boundary and forbids blind WM policy from
  sharing one with metadata-bearing shell, broker/portal, or application
  frontend roles. Shared source or executables remain possible only through
  separately supervised processes with no ambient cross-domain IPC.
- Opaque action integers also needed provenance. The ratified identity binds
  issuer role/authority and revocation epoch, recipient role/epoch, operation
  class, optional target slot/generation, and recipient-epoch activation
  identity. `ActionCapabilityTopology.als` rejects cross-issuer type confusion,
  stale/revoked use, recipient and target substitution, and activation replay;
  each weakened attack retains a satisfiable witness, and a valid scoped action
  retains a non-vacuous witness.
- A native shell exclusive zone cannot commit independently of application
  projection. The target sequence binds a ready shell candidate/reservation to
  the exact Engine work-area generation, WM snapshot and connection epoch, and
  answering projection before one logical presentation. Normal shell or WM
  failure preserves the previous complete bundle; security surfaces keep their
  independent preemptive path. Tier-0 indicator geometry is instead stable
  session/Engine chrome configuration established before WM policy, so
  descriptor loss clears content without changing the work area.
- `ShellWorkAreaCoordination.tla` checks that sequence across 12,278 distinct
  states to depth 23. Temporary removal of candidate readiness, reservation
  equality, and WM-epoch equality violated the corresponding readiness,
  coherent-bundle, and exact-epoch invariants. All weakenings were restored.
  The complete pinned TLA+ Tools 1.7.4 gate passed, including the existing
  5,518,840-state target-resolved input model.
- The pinned Alloy 6.2.0/SAT4J and stable Z3 4.16.0 architecture gate passed
  every positive property, non-vacuous valid-action witness, retained attack
  witness, and arithmetic query. No local executable Z3 5.x build was present,
  so the optional differential was not run. No production Rust, wire schema,
  shell runtime, sandbox, or application-input routing changed in this pass.

<!-- END IMPORTED BODY -->
