---
id: legacy-active-0387
date: 2026-08-03
recorded_date: 2026-08-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-03: install the immutable Milestone 11 release

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11751–11772. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Signed commit `ff8cb2f9aa76f7f46601891241b19ff947b2d67e` produced immutable
  release `0.1.0-ff8cb2f9aa76`. Packaging built the optimized Sophia CLI and
  generic X11 WM bridge offline, copied the resolved xmonad executable, and
  recorded the complete artifact manifest and SHA-256 ledger.
- The staged installer and rollback fixture passed, followed by an exact
  unprivileged installation of the release artifact into an isolated prefix.
  The system installation then promoted the same verified artifact to
  `/opt/sophia/releases/0.1.0-ff8cb2f9aa76`, made it the `current` target, and
  retained `0.1.0-21002fe74c2a` as `previous`.
- Every installed manifest digest passes. All twelve public operator commands
  resolve through `/usr/local/bin` into the immutable current release, and the
  xmonad, Kitty-baseline, and Firefox-proof greetd entries execute only
  `/opt/sophia/current/bin/*`. Ordinary login therefore requires no checkout,
  source build, temporary artifact path, privileged service operation, or
  process cleanup.
- This closes the installation mechanism gate, not the physical-login gate.
  The installed release still needs the retained chrome proof, normal login
  and logout captures, independent recovery and fallback evidence, and the
  documented operator handoff required by the remaining Milestone 11 items.

<!-- END IMPORTED BODY -->
