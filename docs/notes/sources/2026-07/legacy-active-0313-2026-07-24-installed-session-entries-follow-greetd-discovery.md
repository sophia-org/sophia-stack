---
id: legacy-active-0313
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "tooling"]
---
# 2026-07-24: Installed Session Entries Follow Greetd Discovery

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9712–9720. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first immutable lifecycle release installed its desktop entries below
`/usr/local/share/wayland-sessions`, but this host's explicit tuigreet command
scans `/usr/share/wayland-sessions`. The files were valid yet could not appear
in the menu. The system installer and current-release verifier now use
`/usr/share/wayland-sessions`, matching the configured greetd discovery
boundary. Staging tests continue to override the directory explicitly.

<!-- END IMPORTED BODY -->
