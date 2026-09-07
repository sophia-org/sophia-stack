---
id: legacy-active-0121
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-06: Consecutive-cycle evidence has a stable endpoint

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4010–4025. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed commit `4cc84913` passed normal archives `0003` through `0005`, then
passed fallback `0005`, watchdog `0003`, and emergency `0002`. The intentional
emergency correctly added a failed normal attempt. Because the cycle command
only selected the latest runs, that later evidence made the earlier passing
three-cycle gate impossible to reproduce even though its immutable inputs
remained intact.

`sophia-verify-cycles COUNT THROUGH_RUN` now selects the named direct ledger
child and its immediately preceding attempts. It applies the unchanged
checksum, result, identity, lifecycle, commit, and launch-uniqueness checks and
never skips an intervening failed or pending attempt. Fixtures retain an
earlier pass across later failures, reject a failed endpoint, reject an
endpoint outside the ledger, and keep latest-run behavior unchanged.

<!-- END IMPORTED BODY -->
