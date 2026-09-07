---
id: legacy-roadmap-0004
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Production Readiness Infrastructure

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 438–470.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0001-current-position.md).

<!-- BEGIN IMPORTED BODY -->

This supporting tranche does not reorder the product critical path above.

- [x] Extract production session lifecycle and domain integration tests from
  `sophia-cli` into `sophia-session`. The installed binary now selects commands
  and owns concrete stdout/stderr presentation; the passive session library
  reports exact evidence through a host-installed callback.
- [x] Extract typed profile, direct-scanout evidence, archive, and gate logic
  from `xtask` into development-only `sophia-conformance`. Production crates
  and installed artifacts have no dependency on it.
- [x] Make `cargo xtask` the canonical offline developer/CI entry point and
  reduce `just` to optional one-line human aliases. Direct-scanout verifier and
  archive shell entry points are compatibility shims into the typed Rust path;
  no repository workflow calls `just`.
- [x] Add canonical `sophia session run` and `sophia session input-guard`
  commands. The old flat spellings remain delegating compatibility aliases.
- [x] Replace the numeric source-layout baseline with an exact identity ledger.
  A moved, added, or retired violation now requires review; every recorded row
  remains debt rather than becoming an exception.
- [ ] Move the remaining session-private test modules out of production source
  as visibility boundaries are made testable, and split the oversized cohesive
  units named in `docs/source-layout-debt.txt`. Do not weaken privacy or create
  test-only production APIs merely to move a file.
- [ ] Reduce `tools/start_sophia_tty3.sh` to the smallest necessary TTY/display-
  manager adapter around the production session entry point. Typed profile
  parsing, verification, archive handling, and gate orchestration already live
  in Rust and must not return to shell.

The ownership and command contract is in
[`docs/development-tooling.md`](../../../development-tooling.md). The next product
row remains direct-scanout return-to-composition on overlay/effect activation.

<!-- END IMPORTED BODY -->
