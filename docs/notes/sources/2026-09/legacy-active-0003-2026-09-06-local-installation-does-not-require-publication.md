---
id: legacy-active-0003
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-09-06: local installation does not require publication

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 93–108. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Remove the Hagia `HEAD == origin/master` packaging prerequisite. A local
development install should not require a network push, and a remote-tracking
ref is not release identity. Keep the existing source-commit signature and
default-profile checks, clean Sophia worktree requirement, manifest commit and
binary hashes, and installed artifact verification. Signed local Hagia commits
are installable even when ahead of the remote or without a tracking ref.
Historical physical-proof workflows retain their separate prerequisites.

The installer still accepts existing policy executables, so changed Hagia
sources must be rebuilt before packaging. This change removes the publication
dependency without changing that build behavior or reloading the live session.
Validation: shell syntax and diff checks pass, as does the existing temporary
install/activation/rollback regression, including artifact rejection cases.

<!-- END IMPORTED BODY -->
