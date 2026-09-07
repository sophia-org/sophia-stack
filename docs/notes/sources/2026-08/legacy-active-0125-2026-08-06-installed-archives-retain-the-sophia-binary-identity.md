---
id: legacy-active-0125
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-06: Installed archives retain the Sophia binary identity

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4073–4091. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The installed recorder verified `/opt/sophia/current/SHA256SUMS` while a
session started, but an attempt archive retained only the release manifest and
runtime facts for Kitty, Firefox, and xmonad. Installing a later candidate
could therefore remove the only local file that named the exact Sophia binary
digest behind an older run. The commit remained known, but the durable
Milestone 12 identity contract was incomplete.

Runtime identity schema 2 now records the packaged Sophia version and
executable SHA-256 digest. The shared ledger verifies that value against the
installed file before takeover and finalization, then copies it into the
checksummed schema-4 attempt manifest. Normal-cycle, fallback, watchdog, and
emergency archive verifiers compare the manifest value with the retained
runtime identity without consulting the current installation. Fixtures cover
real capture, a missing or unavailable identity, a false expected digest, and
a self-consistently checksummed archive whose manifest lies about the binary.
No application content enters the record.

<!-- END IMPORTED BODY -->
