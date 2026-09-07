---
id: legacy-active-0406
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation", "architecture"]
---
# 2026-08-08: complementary Alloy and Z3 gates bound early architecture choices

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12383–12415. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The prior architectural-alignment draft described an aspirational five-tool
  stack as though it were implemented and used absolute language about what
  models and fuzzers guarantee. It is replaced by an evidence policy: TLA+
  owns temporal behavior, Alloy owns bounded relational topology, Z3 owns
  arithmetic obligations, and executable tests own implementation behavior.
  Correspondence is explicit and none of these models is a Rust refinement
  proof.
- `AuthorityTopology.als` checks exact role admission, namespace-local access,
  portal-mediated cross-namespace authority, WM application-metadata blindness,
  and independently issued coordinate capability. `PresentedTargetTopology.als`
  checks visible allocation and occlusion, explicit trust and equal-priority
  selection, modal membership, authority/session/slot/generation uniqueness,
  and independent local grants.
- `TargetGeometryAndDisclosure.smt2` checks containment, intersection clipping,
  quantization, capability-epoch rate limits, and bounded target/outcome
  partitions without claiming zero telemetry. `WmV1WireBounds.smt2` consumes
  record widths, count bounds, message prefixes, and field maxima generated
  directly from `protocol/sophia-wm-v1.kdl`.
- Each protected Alloy assertion produced no counterexample in its explicit
  scope and each weakened attack predicate produced a witness. Each protected
  SMT query returned `unsat` and each weakened prefix, multiplication, clipping,
  quantization, rate-reset, or partition query returned `sat`. The unattended
  offline runner pins the official Alloy 6.2.0 archive by SHA-256, selects
  SAT4J with fixed symmetry, requires stable Z3 4.16.0, rejects errors and
  `unknown`, and optionally compares a local Z3 5.x build without replacing the
  stable gate.
- This evidence does not close the admitted application-input runtime debts or
  ratify shell wire limits. Spin/Promela, dependency-policy enforcement, and
  fuzzing remain follow-on candidates until a specific question, retained
  artifact, expected result, and reproducible runner exist.

<!-- END IMPORTED BODY -->
