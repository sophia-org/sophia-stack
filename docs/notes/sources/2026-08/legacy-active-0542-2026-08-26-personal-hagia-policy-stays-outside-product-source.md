---
id: legacy-active-0542
date: 2026-08-26
recorded_date: 2026-08-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "tooling"]
---
# 2026-08-26: personal Hagia policy stays outside product source

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16731–16746. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Hagia's user-facing XDG profile is personal desktop policy and is not a Sophia
release input. Hagia now owns one generic tracked default which is also its
compiled fallback; it retains the freeze-profile actions and the bounded 28 px
visible switcher claim, but no named outputs, local program paths, or personal
shortcut choices.

Sophia release schema 5 packages that signed Hagia default and records its
source commit and SHA-256. Installed Hagia now has two explicit modes. The
ordinary entry discovers XDG, system, then packaged fallback configuration and
records only the selected mode, root hash, and activated effective digest. The
promotion entry ignores mutable configuration, archives the generic packaged
profile, and verifies its hash against the release. Separate ledgers prevent a
healthy personal session from being mistaken for immutable promotion evidence.

<!-- END IMPORTED BODY -->
