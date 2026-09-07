---
id: lqicnr4v
date: 2026-09-07
kind: investigation
status: closed
tags: [investigation, tooling, validation]
---
# Evidence readers must follow message identity and proof requirements

The August inventory behind t024 was stale. Most retired-policy readers had
already disappeared or been repaired. Two historical milestone readers still
stopped below the current completion schema. The packaged Firefox verifier
still expected the old WM-ready and restart records; its old fixtures passed
without exercising the native tuples. Milestone 4 read the whole completion
record, then rejected everything except schema 14 in a separate condition.

The existing schema tripwire passed while checking seven acceptance sites for
two record families. It did not cover completion or WM readiness, and a literal
`schema=N` source scan could not resolve the conditional completion emitter.

## Reader decisions

- Two-xterm and paired Milestone 3 are archive-only, as selected by the user.
  Their readers retain historical schemas and budgets. Their old live launchers
  exit with status 2 before devices, sudo, or service changes. The three-class
  verifier remains a historical aggregate.
- The packaged Firefox verifier reads complete native schema-4 and historical
  schema-1 readiness tuples. It rejects duplicates and mixtures, matches restart
  records to their readiness protocol, and applies the same admission-restart
  limits and reseed requirements to both. Required startup timing remains numeric.
- Milestone 4 remains a focused GPU diagnostic. Its schema-16 path requires a
  startup timing and separate Copy counter. Successful Copy plus Flip completions,
  together with Skip, must account for every Idle. Schema-14 archives keep their
  original accounting. Mixed exports, acquire waits, one controlled rejection,
  and clean teardown remain required.
- Persistent-session evidence is an input/startup proof, so its final schema
  admission now refuses normal-session schema 17 explicitly. QEMU's parsed
  schema admission is expressed as one reviewed condition rather than a chain
  that the source tripwire could not inspect.

## Guard ownership and limits

The guard inventories 22 shell completion readers, two WM readiness readers,
and the Rust direct-scanout completion reader. It checks 33 schema acceptance
sites across four message identities, preserving the two older guarded families.
The inventory distinguishes proof-required, normal-compatible, and archive-only
readers. WM identity includes `status=ready`; other WM statuses do not select its
schema. Completion has two branches: 16 for a requested startup proof, 17 without
one. A normal completion cannot substitute for a proof.

This is a bounded source check, not a general Rust or shell parser. It recognizes
literal schema selectors, the existing conditional completion expression, and
three named parsed acceptance conditions. Unknown emitter forms, unregistered
readers, missing reviewed conditions, and ambiguous readiness emitters fail for
review. End-to-end verifier fixtures own field and lifecycle semantics. A regex
matching a version alone is not proof that every assertion is correct.

The guard's self-tests, retirement checks, and Firefox/Milestone 4 fixture suites
run through `cargo xtask check`. Historical fixtures remain unchanged; current
fixtures and negative controls are derived separately.

## Validation

Focused checks pass: guard mutations, launcher retirement and historical
verification, native/historical Firefox admission and recovery, schema-16 GPU
Present accounting, normal schema-17 Hagia evidence, terminal performance, and
the broader atomic-scanout verifier fixtures. None exercises physical hardware.

The first full check in the shared checkout failed in the ongoing t059 work:
`xfixes_minors_without_an_implementation_are_refused_by_name` expected a four-byte
request to decode after a newly implemented minor required twelve bytes. It is
not a t024 reader failure. Independent validation uses base `9540dccb` plus only
the t024 changes, leaving the other agent's working files intact.

`cargo xtask check` passed on the isolated candidate, including workspace tests,
Clippy, 13 guard tests, the added verifier suites, all retained archives (Hagia
5/5, mirror-group 9/9, direct-scanout 6/6), and the host buffer-age equivalence
check. The broader atomic-scanout verifier fixtures also passed separately.

The exact changed-code identity is
`735c6308771ddb09b31209d0b58b3dd75e08e9e56c708dd6746d4bd1cf6e3671`.
It covers 18 code/test files whose bytes matched the working checkout after
validation. The base is signed commit `9540dccb`; the candidate consists of that
base plus these files. Source copies, per-file digests and modes, commands, setup,
and logs are retained at
`/home/niltempus/.local/state/sophia/development-evidence/t024-735c6308771d`.
`SHA256SUMS` binds the retained artifacts.

The temporary checkout required removing group/other write permission from its
tracked KDL fixtures and explicitly locating the existing Hagia and Narthex
repositories. Its built debug executable supplied the opaque binary-hash input
expected at `target/release/sophia` by the synthetic archive test. That alias was
only test setup, not an installed release or physical proof. Earlier setup
failures are retained beside the final passing log.

This completes the reader task's offline exit. It does not claim a passing full
check for the other agent's changing t059 tree, accept a new physical candidate,
or close any outstanding live-session workflow.

## Connections

The [parallel plan](../plans/queue-11-parallel-production-readiness.md#t024)
owns t024's exit. The [original schema-drift investigation](../sources/2026-08/legacy-active-0569-2026-08-30-a-schema-bump-that-silences-its-own-readers.md)
explains why acceptance must not disappear when a version changes.
[Validation](../../validation.md#evidence-reader-compatibility) describes the
maintained contract; [work tracking](../../work-tracking.md) keeps task state
out of these notes.
