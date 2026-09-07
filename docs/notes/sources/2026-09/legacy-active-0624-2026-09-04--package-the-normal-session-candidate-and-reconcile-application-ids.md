---
id: legacy-active-0624
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "tooling"]
---
# 2026-09-04 — Package the normal-session candidate and reconcile application IDs

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20074–20098. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Signed Sophia `417e97d2` and Hagia `38ea8da` are packaged as
`.artifacts/sophia-0.1.0-417e97d2e25b/`, with the current Narthex executable,
canonical default profile, manifest, and SHA-256 inventory. Package policy
verification and the complete installed command/session surface pass. No running
session or display manager was changed. `/opt/sophia/current` still points to
`0.1.0-2823807e2ecd`: the installation attempt stopped because sudo requires
local password authentication. `/tmp/i` installs the prepared artifact, verifies
the selected commit and checksums, and prints start, logout, emergency, stop,
and rollback instructions. Activation retains the former current release as
previous through the existing installer.

Complete session-argument preflight found a separate session-authority mismatch:
the personal profile selected application IDs `kitty` and `helium`, whereas the
launcher registers `terminal` and `browser`. Both parsers correctly accepted
the syntax, but session capability preparation rejected the unavailable IDs.
The operator selected the recommended two-ID correction. Only those two values
were changed, with a private backup retained in the parser-delegation diagnostic
bundle. Layouts, shortcuts, input, and output settings were preserved. Paired
checks and complete normal-session argument validation now pass, using the
packaged Sophia/Hagia/Narthex executables. This is application-registry alignment,
not WM policy interpretation or a broader profile rewrite. Physical stage 2
acceptance remains open.

<!-- END IMPORTED BODY -->
