---
id: legacy-active-0390
date: 2026-08-03
recorded_date: 2026-08-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-03: installation resolves the current release automatically

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11847–11866. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Requiring an operator to copy a commit-derived artifact directory into the
  privileged install command made the ordinary release path unnecessarily
  error-prone. The repository already had an exact-current-commit resolver,
  but it was exposed under a second command instead of the documented
  installer.
- `tools/install_live_session.sh` with no argument now delegates to that
  resolver. It packages only a clean current commit when its immutable artifact
  is absent, verifies the manifest commit and full SHA-256 ledger, requests
  privilege only for the default system prefix, and then performs the existing
  atomic install. Supplying one explicit artifact directory remains supported
  for staged validation and recovery tooling.
- The install regression constructs an exact-current-commit artifact in an
  isolated root and exercises the argument-free command through digest
  verification, release promotion, session entry installation, and final
  manifest identity. Existing explicit-artifact install and rollback coverage
  remains unchanged. The operator command is now simply
  `tools/install_live_session.sh`.

<!-- END IMPORTED BODY -->
