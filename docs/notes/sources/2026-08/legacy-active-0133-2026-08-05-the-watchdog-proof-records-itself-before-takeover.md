---
id: legacy-active-0133
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-05: the watchdog proof records itself before takeover

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4302–4323. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The dedicated recovery entry previously produced valid live logs but required
a later `sophia-record-watchdog-run` invocation. That post-session copy had no
pending state: a wrapper interruption or later graphical launch could erase
the only active proof before it entered the immutable archive.

`Sophia Recovery Proof` now selects the watchdog attempt profile explicitly.
The installed wrapper reserves a separate numbered directory before graphics
takeover, preserves a crash as pending, and finalizes only after the expected
status-124 display-manager handoff. The shared ledger now parameterizes the
expected session status, lifecycle mode, and whether the focused verifier also
consumes lifecycle evidence; normal and fallback semantics remain unchanged.

The repository-independent aggregate verifier checks archive digests, the
watchdog result and focused recovery contract, runtime identity, schema-3 kind,
release commit, start time, and launch-identity digest. Automatic-session
fixtures reject a wrong exit, failed latest attempt, and modified archive.
Packaging, installation, status, validation guidance, and the operator runbook
expose `sophia-verify-watchdog`; the old no-argument recorder remains only as a
compatibility importer for an unrecorded proof.

<!-- END IMPORTED BODY -->
