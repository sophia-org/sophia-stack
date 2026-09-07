---
id: legacy-milestone-0010
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-07-25 Physical Session Infrastructure

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 243–264.

<!-- BEGIN IMPORTED BODY -->

- [x] Added a fail-closed installed launcher requiring an owner-controlled
  runtime directory and real local VT.
- [x] Recorded ordered preflight, input-guard, graphics-takeover, session, and
  display-manager-handoff phases for normal and emergency exits.
- [x] Added immutable runtime identity and lifecycle evidence to promotion
  captures.
- [x] Installed versioned release binaries and a greetd session entry without
  compiling during login.
- [x] Replaced development takeover assumptions with explicit seat/VT
  ownership and bounded display-manager handoff.
- [x] Added the configured Firefox action, protocol-neutral wheel routing,
  deterministic physical verifier stages, and atomic no-focus behavior for
  hidden workspaces.

These mechanisms are established but do not constitute installed-session
promotion. The active roadmap still requires a repository-independent release
path and repeated physical workflow evidence.

---

<!-- END IMPORTED BODY -->
