---
id: legacy-active-0071
date: 2026-08-11
recorded_date: 2026-08-11
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-11: The freeze surface is bounded, and reserved space is asymmetric

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2170–2228. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The roadmap had been treating "additive wire records are cheap" as a standing
  property. It is a pre-freeze-only property, and nothing recorded that. This
  pass enumerated the risk and wrote `docs/wm-v1-freeze-surface.md`.
- Two rejection behaviors set the cost. Unknown record kinds fail closed in both
  transfer decoders, and unknown enum discriminants fail closed everywhere, so a
  pre-freeze client cannot tolerate a post-freeze value. Both `*Begin` messages
  are fixed-layout with a strict terminal length check, so a new record kind
  needs a sixth count field, which is an existing-layout change and therefore a
  new interface family. The indicator descriptor is the worked precedent: it
  landed pre-freeze exactly so `indicator_count` and `status_count` could be
  added.
- A previous reading of this boundary was wrong in a way worth recording.
  Grepping the generated record module for `reserved` returns nothing, which
  looks like proof that no reserved space exists. The generated Rust omits
  reserved fields while the wire still carries and validates them:
  `WmV1ProjectionOutputRecord` has four fields, `PROJECTION_OUTPUT_RECORD_SIZE`
  is 24 bytes, and the codec pushes a zero it later rejects if non-zero. Struct
  shape is not wire shape.
- Reserved space is real but unevenly distributed. Fourteen reserved fields
  exist: twelve messages carry a `u16`, and `ProjectionOutput` and
  `ProjectionOutputStatus` carry a `u32` each. Six of the eight record kinds have
  none, so any new per-surface, per-placement, per-indicator, or per-output fact
  is either a bitfield extension or a layout change. `capabilities` is a `u64`
  with bits 0 through 9 assigned.
- Enumerating all 27 retained port-ledger rows found 23 that need no
  `sophia_wm_v1` change. The residue is four decisions, three of which must be
  settled before the freeze forecloses the cheaper option:
  configured workspace and view names should project as indicator labels, since
  a policy-authored label is blind-safe by construction and the record already
  carries a bounded label; broker-issued classification should commit to a
  closed set of policy classes that fits `SnapshotSurface.kind` or spare
  `capability_bits`, because expiring per-surface grants are the only thing in
  the ledger that forces a layout change; the continuous-pointer payload fits
  the four existing `interaction_*` integers with `reserved_cause` spent on an
  axis discriminant, so drag and scroll need enum values but no layout change;
  and the output logical-space contract needs writing down so the output
  authority does not widen `SnapshotOutput`, which has no reserved space.
- Two coupling facts constrain when, not what. `Cancel` is a lease-revocation
  contract and must be specified alongside the lock and security epoch barrier
  or it will be specified twice. Per-motion `Update` requests are currently
  dropped because the queue deduplicates by pointer-gesture source, so
  continuous updates need a replaceable latest-value coalescing rule rather
  than more queue capacity.
- Two items remain open product calls: the forward-compatibility rule
  (skip-unknown behind an extension chunk, or declare revision 3 final for
  WM-side records) and whether native output mirroring is implemented with
  evidence or explicitly rejected. The ledger forbids leaving the second
  implicit, since "not yet implemented" is not an exclusion.
- Corrected in passing: the claim that the bootstrap profile used 39 of 64
  binding slots. Every constant in both repositories is 256. The bootstrap emits
  39 key plus 2 pointer bindings from Triad's 132 key plus 5 pointer baseline,
  and the remaining bindings are classified into authorities that do not exist
  yet. The open question is the authority split, not capacity.
- Milestone 12 is archived to `docs/roadmap-history.md`. Its last open item was
  the X11 compatibility matrix update, whose evidence had already landed with the
  passing TrueColor proof; only the checkbox was stale.

<!-- END IMPORTED BODY -->
