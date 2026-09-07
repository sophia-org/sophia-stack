---
id: legacy-active-0565
date: 2026-08-29
recorded_date: 2026-08-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-08-29: the production session is not a CLI test harness

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17593–17642. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The CLI looked large because it was carrying three different products under
one crate name: installed command presentation, the production session host,
and development conformance/archive logic. Counting it as one tool obscured the
dependency direction and made shell wrappers look like architecture.

The split now follows authority. `sophia-session` owns production lifecycle,
supervision, recovery, and the integration tests for those domains.
`sophia-conformance` owns typed profile and direct-scanout evidence/archive
logic and is reachable only from `xtask`. `sophia-cli` remains the installed
command and presentation boundary. `just` is an optional vocabulary for a
person at a terminal; it has no workflow implementation and nothing calls it.

Moving the session code out of the CLI immediately exposed 343 direct
`println!`/`eprintln!` sites. Recording those as new exceptions would have made
the crate split structurally honest and behaviorally worse. The session now
emits exact line evidence through process-wide host callbacks. The `sophia`
binary installs stdout and stderr callbacks once; worker threads preserve the
same evidence lines; a library caller that installs no presenter receives no
ambient output. This keeps the existing log schemas while restoring the
passive-library rule.

The installed spelling is now `sophia session run`;
`sophia-live-session` remains a delegating compatibility alias. Profile
validation crosses that canonical binary boundary in validation-only mode,
which stops after the real production argument parser and before DRM, input,
or display-manager work. A CLI integration test pins that property.

The direct-scanout slice now has one typed owner. Duplicate telemetry fields
fail closed, standalone evidence proves the bounded no-WM/no-chrome session
shape, archive identity and checksums are Rust data, and gate orchestration
returns a passive report for `xtask` to present. The five old shell entry points
delegate to that path. The TTY/display-manager takeover remains shell because
it is an OS adapter; reducing it further is explicit remaining debt, not a
reason to put validation back into shell.

Source-layout control also changed from a number to identities. A numeric
baseline can hide one new violation behind one retired violation. The exact
ledger cannot. Moving the production host changed the paths but not the debt
count, and the ledger records precisely that move. The remaining private test
modules and oversized units are still red under the raw audit and remain
roadmap work.

The important negative dependency claim is checked by construction: production
crates do not depend on `xtask` or `sophia-conformance`; installed launchers call
`sophia`; CI and new repository workflows call `cargo xtask`; and scripts never
call `just`. This is why `just` and `xtask` both help without becoming two
competing build systems.

<!-- END IMPORTED BODY -->
