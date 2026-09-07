---
id: legacy-active-0630
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "tooling"]
---
# 2026-09-06 — Migrated browser selection blocked installed login

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20300–20316. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next greetd login returned before graphics takeover. The recovery guard
armed, but `session-args-check.err` reported `UnavailableShortcutCapability`.
The migrated desktop selected `brave-origin` and bound Super+B to the browser
action; the wrapper registered Firefox under `browser`, and the core registered
only the panel. The selected browser therefore had no execution registration.
Checking each configuration file separately had missed the unresolved role.

Registered `brave-origin` explicitly in the user's core configuration, preserving
the selected browser and keeping executable authority in the session. The old
configuration is backed up as `config.kdl.before-browser-registration`. The
installed binary rejected the original assembled session arguments and accepted
them after the registration, including ordinary core discovery, the terminal
adapter, desktop components and launcher selection. Validation opened no hardware
and launched no applications. The next login remains the physical check.

<!-- END IMPORTED BODY -->
