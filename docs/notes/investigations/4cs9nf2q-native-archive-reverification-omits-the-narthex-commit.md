---
id: 4cs9nf2q
date: 2026-09-12
kind: investigation
status: closed
tags: [investigation]
---
# Native archive re-verification omits the Narthex commit

## Finding

Source review at `4507c741` found that
`tools/verify_hagia_native_session_archive.sh` requires exactly one
`narthex_commit` field for schema 2 but does not verify that commit's existence
or signature. Its signature loop checks Sophia and Hagia only. The later
manifest-to-evidence comparison also omits the Narthex commit, although it
compares the shell executable hash.

The creation path checks all three source identities. Re-verification is an
independent boundary: checking the field count and recalculated file checksums
cannot establish that its Narthex identity still matches the recorded run.
The contained reproduction at `617651ae` confirms the omission. A fabricated
archive with genuine signed Sophia, Hagia and Narthex commits first passes.
Changing only its manifest Narthex commit to forty `f` characters and
recomputing `SHA256SUMS` still passes the original verifier. Evidence is
`.artifacts/t095-baseline-617651ae-v2`. No retained physical archive has been
changed, no native executable ran, and no historical hardware result is being
reassigned. The repair and schema-1 compatibility tests now pass.

## Discovery and scope

The contained canonical attempt at `4507c741` stopped earlier, because
`check_hagia_native_matchers.sh` expected sibling Hagia and Narthex repositories
that the private source snapshot did not contain. Its report is
`.artifacts/offline-input-full-4507c741/report.json`: full check FAIL, hardware
NOT_RUN. That missing fixture dependency is distinct from this verifier defect.

The fixture uses synthesized archive identities and harmless executable hash
inputs. It must receive explicit committed source snapshots and genuine public
signature verification material; exposing host directories or bypassing
signature verification would not repair it.

## Required evidence

Retain a valid schema-2 archive as the positive control. Change only its
Narthex manifest commit and recompute `SHA256SUMS`; the verifier must refuse
the identity mismatch. Independently test nonexistent or unsigned Narthex
commits with matching manifest/evidence fields, and valid signed identities.
Schema-1 archives predate the Narthex split and must retain their documented
verification path. Mutations must operate on fabricated copies, never retained
operator evidence.

## Connections

Task t095 records this repair and its independent negatives. The contained
canonical attempt described above exposed the missing fixture dependency.


## Resolution

Repair `e942566b` is integrated on master as `fd1045be`. Schema 2 now compares
its Narthex manifest commit with the evidence and verifies that the named
commit exists and has a valid signature in the supplied Narthex repository.
Schema 1 retains its pre-split verification path without a Narthex dependency.

Contained evidence in `.artifacts/t095-fixed-617651ae/evidence/report.json`
records the positive archive, rejection of the retained false-pass mutation,
and a failing restored-original-verifier mutation. The matcher also verifies
missing and unsigned objects, unavailable repositories and schema-1
compatibility. The full matcher passes in 12.08 seconds. These are verifier
fixture results, not native-session or hardware acceptance.

Task t095 is complete. Explicit sibling identity inputs for the canonical
wrapper are a separate prerequisite repair, integrated as `70087af7`.
