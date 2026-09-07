---
id: legacy-active-0001
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering"]
---
# 2026-09-06: desktop composition belongs to the session

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6–46. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The user reported that native shortcut help worked as intended after installing
Sophia `47b6a53d`, Hagia `82103fa`, and Narthex `7210b4e`. This is a live usage
confirmation, not a new comparison matrix. Quickshell remains the X11 panel;
Narthex remains the native switcher and helper.

Put desktop component choices in the existing operator profile, discovered at
`sophia/desktop.kdl` before the legacy Hagia path. Session owns WM and native-shell
executable selection, private shell config selection, and login applications.
The launcher supplies defaults; an explicit profile or CLI selection can replace
them. Preserve the existing confined launch and recovery paths. Empty startup
means no login applications. WM reload never replays startup applications, and
component selection changes remain deferred until the next login.

The user-facing contract is [Desktop composition](../../../desktop-composition.md).
Offline policy validation now exports only the Policy authority for Hagia.
Session vocabulary no longer has to be admitted by Hagia's whole-profile parser.
Other WMs validate their own vocabulary at protocol activation. The existing
native shell wire is unchanged; multiple native-shell assignments and general
service supervision remain future work. No live process is replaced by this
implementation, and the user's existing config remains intact.

The first full check exposed tests that discovered the live personal application
registry. `cargo xtask check` now gives its workspace-test subprocess a private
empty XDG configuration directory and removes the private-shell-config override.
Tests of discovery still select their own fixtures; the user's home and state
directories are unchanged. This keeps desktop choices from changing unrelated
test results.

Validation: the canonical `cargo xtask check` passed with 2,457 successful Rust
test executions, zero failures, and one existing ignored test. Clippy reported
no warnings. The final launcher preflight and source-selection fixtures passed,
as did all 20 launcher-safety tests and the native-session release build.
The prepared personal migration at `/tmp/sophia-desktop-composition.kdl` passes
Sophia validation and Hagia's policy-only preflight. Its installer,
`/tmp/install-sophia-desktop-composition.sh`, requires clean committed sources,
checks both reviewed file hashes, installs the session, validates with the
installed parser, and publishes the new profile without replacing an existing
file. It has not been run. The next gate is one ordinary login with that profile.

<!-- END IMPORTED BODY -->
