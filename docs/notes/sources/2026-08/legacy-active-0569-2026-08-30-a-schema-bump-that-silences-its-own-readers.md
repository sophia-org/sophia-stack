---
id: legacy-active-0569
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-30: a schema bump that silences its own readers

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17762–17806. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Three Milestone 14 rules stopped being checked without anything failing. The
one-KMS-submission bound was guarded by `[[ "$resource_schema" == 9 ||
"$resource_schema" == 10 ]]` and the renderer-misroute and service-skew rules
by `== 10`, so all three went unasserted the moment `sophia_live_native_resources`
reached schema 11. The QEMU shared-worker expectation and the input-latency
reporter's `renderer_workers` field matched `schema=10` exactly and could not
read a current session at all, which is why `renderer_workers` has never once
populated in an input-latency report -- the field added specifically to
distinguish shared-worker from per-output measurements. Five more verifiers
stopped at schema 9, including the warmed-resource assertions in
`verify_sophia_xmonad_four_kitty.sh`.

The shape of the defect is that a missing line is indistinguishable from a
satisfied rule. A reader greps for a record, finds nothing because the schema
moved, and skips the block that would have failed. Nothing in the output says
a rule was skipped, and the run passes with fewer assertions than the operator
believes they bought.

Guards now compare with `>=` and acceptance patterns admit any schema the
emitter writes. Evidence too old to carry a field is refused by the field check
rather than by the schema, which puts the refusal where the missing information
actually is: `resource record is missing renderer_workers` names the problem,
where `no schema-10 record` names only a number.

`tools/check_live_record_schema_readers.sh` makes the next bump fail offline.
It reads the schema the emitter writes and refuses any reader that can match
only older ones. It guards one record on purpose. A record name does not
identify a message -- `sophia_live_wm` writes schema 4 for `status=ready` and
schema 1 for `status=session_action_committed`, and `sophia_session_app` writes
schema 1 and 2 for the same status under different sources -- so guarding a
record means having first checked that its emitters agree. Fixture builders
under `tools/check_*.sh` are excluded because they write old-schema evidence
deliberately, to prove a verifier still accepts an archive.

Running the general form of that check surfaced two further pinned readers not
repaired here, because each needs a per-gate decision rather than a sweep:
`sophia_live_session status=bounded_complete` is emitted only at schema 16 while
ten readers accept at most 15, and `sophia_live_wm status=ready` is emitted only
at schema 4 while nine readers accept only 1. Those are xmonad-era physical and
QEMU gates. They fail loudly rather than silently, so they are broken rather
than misleading, and they are recorded here rather than fixed inside a
Milestone 14 change.

<!-- END IMPORTED BODY -->
