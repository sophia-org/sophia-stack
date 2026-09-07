---
id: legacy-roadmap-0011
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 13.2 Publish A Dependency-Free Wire Contract

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 606–663.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0009-milestone-13-public-policy-protocol-and-hagia.md).

<!-- BEGIN IMPORTED BODY -->

- [x] Keep the bounded 24-byte little-endian Sophia envelope and owner-only
  Unix transport. Make the session host role-specific sockets beneath its
  private runtime directory and admit exactly the expected supervised peer.
- [x] Define stable layouts in a narrow checked-in KDL schema. Generate and
  retain dependency-free Rust and C99 codecs, normative byte tables, and
  golden vectors; normal builds and third-party clients must not run or link
  the generator.
- [x] Add strict begin/chunk/end transfers for complete snapshots and
  projections above the 64-KiB frame limit. Bound the first WM interface to 16
  outputs, 1,024 manageable surfaces, and 256 registered bindings.
- [x] Compile an independent C client and run it against the same golden and
  malformed-frame corpus as the Rust codec. Reject unknown, excessive,
  partial, duplicate, reordered, stale, and trailing data without mutation.
- [x] Complete the draft revision line before stability (currently revision 3):
  add output work rectangles;
  reduced surface kind, presentation request/current state, and exact-size
  constraints; projection presentation decisions; request causes; policy
  configuration; Engine chrome; session-operation tokens; reduced
  interactions; and a bounded policy-dirty request.
- [x] Preserve non-idempotent activation order with the existing bounded
  sixteen-request owner queue. Coalesce only replaceable scene refreshes and
  continuous interaction geometry; saturation consumes the shortcut, fails
  closed, and emits a bounded diagnostic.
- [x] Regenerate and re-run the Rust/C golden and malformed corpora, then update
  Hagia's independent Nim codec without adding a Sophia build dependency.
- [x] Add the indicator descriptor before the 13.4 freeze.
  `capability "indicators" bit=8`, record kinds `ProjectionIndicator` (max 256)
  and `ProjectionOutputStatus` (max 16), and `indicator_count`/`status_count`
  fields in `ProjectionBegin` are in the schema with generated Rust and C99
  codecs and golden vectors. The generator gained a fixed-octet field type so
  records could carry bounded labels while staying fixed width. Wire bounds are
  permanent: 256 indicators, 16 status records, 32-byte UTF-8 labels and layout
  names. The 32-per-output limit is Engine validation, not a wire constant.
  See `docs/sophia-indicator-descriptor.md`.
- [x] Model the descriptor before changing the schema. Revise
  `validation/tla/ShellObservation.tla` so the descriptor rides the proposal and
  its invariants hold with no explicit publish or invalidate step, and add
  `validation/tla/IndicatorTransfer.tla` for declared-count, ordinal, and
  bounds integrity across begin/chunk/end.
- [x] Regenerate the Rust and C99 codecs, wire tables, and golden corpora for
  the new records. `tools/check_policy_protocol.sh` passes end to end. Closing
  it also repaired a pre-existing gap: the C conformance harness had never been
  taught `snapshot_session_operation` or the five policy/session messages, so
  its valid-frame and record gates had been failing before this work began.
- [x] Update Hagia's independent Nim codec for the new records so the
  cross-repository conformance gate stays green, without adding a Sophia build
  dependency. Hagia decodes both records, rejects an over-long label length and
  non-zero padding, and declares zero indicators until it advertises the
  capability. `SOPHIA_STACK_ROOT=… nimble test` passes.
- [x] Defer the tier-1 texture question rather than blocking on it. Whether the
  shared transport can carry shell texture traffic under the 64-KiB frame limit,
  single in-flight transfer, and bytes-only wire binds `sophia_shell_v1` only.
  Tier-0 Engine chrome renders the descriptor with no client interface, which
  removes that question from the freeze path; see
  `docs/sophia-shell-v1-direction.md` open question 2.

<!-- END IMPORTED BODY -->
