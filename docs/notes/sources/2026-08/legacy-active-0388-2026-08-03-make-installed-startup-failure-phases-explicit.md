---
id: legacy-active-0388
date: 2026-08-03
recorded_date: 2026-08-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "tooling"]
---
# 2026-08-03: make installed startup failure phases explicit

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11773–11804. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Installed launch already emitted ordered entering/complete lifecycle phases,
  but every nonzero exit ended with the same handoff record. `sophia-status`
  tailed that record, so an operator could not distinguish preflight, input
  guard, graphics takeover, session, or restoration failure without reading
  the complete log.
- The session wrapper now carries the verified manifest version and commit into
  the runner. A shared lifecycle helper emits one bounded diagnostic containing
  only that release identity, the exact enumerated phase, installed flag, and
  exit status. The current phase advances before each phase's first side
  effect, while a failed TTY, keyboard, keyd, or termios restoration overrides
  the source phase with `handoff`.
- User-requested Ctrl-Alt-Backspace recovery remains an expected emergency and
  is not mislabeled as a startup failure. A watchdog deadline remains a session
  failure even though it follows the emergency cleanup path. The ordinary
  lifecycle and recovery schemas are unchanged, preserving the existing
  promotion verifiers.
- `sophia-status` now prints the newest diagnostic exactly once beside the
  verified installed manifest and final lifecycle result. The regression drives
  an installed-style noninteractive preflight failure, exercises all five
  allowed phase values, rejects an invalid phase, and proves the status output
  retains no duplicated diagnostic. Packaging carries the helper inside the
  immutable artifact instead of reaching back into the repository.
- Signed commit `09113a7da149a57558deea8076529913f9a62705` was packaged as
  `0.1.0-09113a7da149` and promoted to `/opt/sophia/current`. The complete
  installed digest ledger passes, `/usr/local/bin/sophia-status` resolves into
  that release, the packaged lifecycle helper is present, and
  `0.1.0-ff8cb2f9aa76` remains the immutable rollback target. This closes the
  diagnostic mechanism item; the next installed login supplies its physical
  lifecycle observation without a separate operator sequence.

<!-- END IMPORTED BODY -->
