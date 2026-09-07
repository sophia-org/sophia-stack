---
id: legacy-active-0124
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation", "tooling"]
---
# 2026-08-06: The installed soak gate owns its archive

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4056–4072. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The packaged `sophia-verify-soak` command previously accepted a mutable
session-log path. It could prove duration, application exercise, health, and
resource drain, but it did not prove which installed attempt, commit, binary,
launch, or lifecycle produced that log. It also required the operator to find
and paste a path after a long run.

The command now selects the latest normal-run ledger entry without arguments
and verifies its checksums, passed result, schema-4 kind, normal login and
lifecycle, launch digest and timestamp, release commit, and exact Sophia,
Kitty, Firefox, and xmonad identities before applying the focused soak budgets.
Numeric arguments adjust the duration and action thresholds without restoring
a log-path choreography; an explicit archive remains available for historical
checks. Fail-closed fixtures cover a failed latest attempt, unavailable Firefox
identity, a checksummed false Sophia digest, and post-checksum log mutation.

<!-- END IMPORTED BODY -->
