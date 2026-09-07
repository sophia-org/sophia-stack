---
id: legacy-active-0622
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-09-04 — Physical VT return and recovery stage pass

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19996–20025. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The operator reports "done" after the VT-return procedure. The same frozen
candidate ran for 32,144 ms in
`.artifacts/diagnostics/cp14-3-mixed-source-20260905T005021Z/vt-return/` and
exited 0 after normal logout. Two seat releases and returns opened native
epochs 2 and 3. All three owners closed settled, with retirements of 27, 229,
and 19; the final session correctly retained their sum, 275, and all 281
submissions. There were no native submit, retirement, or settlement failures,
and no remaining in-flight or cleanup obligation.

Browser launch was committed after the first return and logout after the
second. The session routed 24 physical keys and flushed all 301 expected input
events. These records establish post-resume input actions and presentation;
the operator did not supply a separate typed-text or per-output visual report.
Logout quiescence completed in 46 ms with zero pending authority, coordinator,
CPU, or native work. Frontend/application cleanup was clean. TTY handoff
restored usable origin and manager input using the manager's safe baseline;
the manager was ready and no emergency recovery was required. The client XIO
message occurred after normal logout began, rather than preceding a session
failure.

Logs, launcher scripts, candidate identity, operator observation, and checksums
are preserved under the bundle's `attempts/04-vt-return-pass/`. Together with
`attempts/03-suspended-deadline-pass/`, this closes CP-14.3 stage 1. Stage 2
now establishes normal live-session use; broader workflow and two-output/tab
acceptance remain open. Neither recovery test nor the deferred comparison
matrix needs restarting to advance. This result changes documentation only;
the previously passing code gate is unchanged.

<!-- END IMPORTED BODY -->
