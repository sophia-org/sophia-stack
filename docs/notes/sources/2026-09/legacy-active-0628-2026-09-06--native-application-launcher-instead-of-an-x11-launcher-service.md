---
id: legacy-active-0628
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "shell"]
---
# 2026-09-06 — Native application launcher instead of an X11 launcher service

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20233–20272. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Implemented revision 4 of `sophia_shell_v1` as a generic catalog and presented
activation exchange. Narthex supplies filtering, ordering and appearance; Engine
composes focus-scoped text, renders the GPU overlay and hit-tests the immutable
catalog labels. Hagia gains append-only action 176, mapped to session operation
slot 7. It receives neither application metadata nor execution commands.

The session owns explicit `trusted-host` catalogs, source precedence, bounded
desktop-entry parsing, terminal adapters and dispatch-time worker revalidation.
Catalog identities are display references. Only a current presented target and
one exact activation acknowledgement can enter the application admission queue.
Queue origin is retained independently of transaction numbers, so equal numeric
WM and shell transactions cannot consume each other's authority. Closing the
menu does not make it the parent supervisor of the launched application's life.
No new host paths or execution privileges are added to the native shell.

This is a host application launch policy, not application confinement. Per-app
namespace credentials, OS isolation, D-Bus activation, files/URIs and desktop
actions remain explicit follow-up work. Unsupported policy does not fall back
to host execution. The freedesktop specifications informed
[argument expansion](https://specifications.freedesktop.org/desktop-entry/latest/exec-variables.html),
[visibility](https://specifications.freedesktop.org/desktop-entry/latest/recognized-keys.html)
and [desktop-file precedence](https://specifications.freedesktop.org/desktop-entry/latest/file-naming.html).

Rust codec and Engine input tests, desktop-source mutation/masking tests, and
protected independent Nim/C exchanges cover the new boundary. The socket proof
transfers 4,096 entries and denies activation before presentation, replay and
activation while a query is pending. The final `cargo xtask check` passed after the admission-origin regression.
The updated shell protocol gate and both Hagia and Narthex `nimble verify`
passed. Downstream verification used isolated XDG configuration so the tests
did not read the personal profile. Physical acceptance remains unchecked.

A new installation script and both candidate configurations are staged under
`/tmp/sophia-native-launcher-*` and `/tmp/install-sophia-native-launcher.sh`. They
preserve the current browser and desktop components. The script checks hashes,
requires committed checkouts, rebuilds both Nim clients, installs through the
existing recipe, validates with the installed parser and publishes the desktop
selector last. The active configuration and live session have not been changed.

<!-- END IMPORTED BODY -->
