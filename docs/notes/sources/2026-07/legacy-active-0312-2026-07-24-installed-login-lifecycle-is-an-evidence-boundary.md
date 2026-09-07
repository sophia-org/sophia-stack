---
id: legacy-active-0312
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation", "tooling", "architecture"]
---
# 2026-07-24: Installed Login Lifecycle Is An Evidence Boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9690–9711. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The development TTY wrapper previously allowed `/tmp` as its runtime root and
did not distinguish build, input-guard, graphics-takeover, session, and return
phases in retained evidence. That was acceptable for bounded development but
could not prove an installed greetd login was independent of repository
fallbacks or identify the exact phase that failed.

The installed entry now declares its stricter contract explicitly: no source
build, no manual service control, an existing absolute user-owned
`XDG_RUNTIME_DIR`, and a real local Linux VT. The shared wrapper records ordered,
content-free lifecycle records from preflight through display-manager handoff.
A fixture-backed verifier rejects temporary runtime state, missing or reordered
phases, and emergency recovery presented as normal logout.

Normal, Firefox, and emergency recorders now retain that lifecycle beside the
release manifest, runtime identity, recovery, guard, and session evidence.
Repeated-cycle verification rechecks the lifecycle of every archived run.
This proves the installed-path contract and makes failures diagnosable; it does
not replace the remaining physical three-login, fallback-session, emergency,
ten-cycle, or soak captures.

<!-- END IMPORTED BODY -->
