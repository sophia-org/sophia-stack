---
id: legacy-active-0599
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation", "architecture"]
---
# 2026-09-04: comparison preparation must make its own immutability boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18956–18976. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first post-correction run was prepared successfully, but an explicit mode
check before physical takeover found its root, sample root, and attempt root at
`0775` and its manifest, schedule, and checksum ledger at `0664`. Preparation
had used ordinary directory creation and `fs::write`, so the host's `0002`
umask silently made the claimed immutable evidence group-writable. Checksums do
not establish integrity when the same writer can replace both an artifact and
its checksum.

Preparation now creates the three directories with mode `0700` and the three
identity files with mode `0600`, then sets those exact modes independently of
umask. Status, gate, capture, binding, verification, and reporting all pass
through the same storage check and reject a symlink, wrong owner, wrong file
kind, or later mode drift before trusting the ledger. Current-UID discovery is
shared with session-attestation admission rather than duplicated. A regression
checks both creation and post-preparation widening. Its temporary roots include
a time nonce because sandboxed test processes may reuse a PID; a retained
failure therefore cannot poison the next run. The zero-row permissive run is
retained under a rejected name and must never receive physical evidence.

<!-- END IMPORTED BODY -->
