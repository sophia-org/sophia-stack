---
id: legacy-active-0596
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-09-04: mutable tool drift aborted capture after takeover

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18854–18881. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The fresh `cp14-schema4-randr` run was prepared from signed candidate
`d5a1f7da`. Its first Sophia row passed native two-head startup, the standard
`xrandr` query, exact topology validation, and automatic session attestation.
The retained attestation identifies the expected candidate and Sophia engine;
there is no RANDR protocol-error record. Capture nevertheless stopped before
creating an incoming attempt, leaving the run at zero of 36 rows.

The host Firefox package had advanced from 154 to 155. The comparison contract
still required 154. Capture correctly rejected that mismatch, but only its
session-dependent preflight checked tool versions. Preparation recorded the
compiled constants without testing the installed executables, and the outer
gate checked tracefs before takeover but not Kitty, Firefox, or niri. The
failure therefore spent a physical transition to discover deterministic host
drift. The launcher's cleanup terminated Sophia, restored and verified tty3,
used the safe tty7 baseline after exact termios round-tripping diverged, then
verified a live tuigreet with text display and enabled keyboard modes before
activating tty7. No emergency chord or hard reboot was needed.

The comparison now pins Firefox 155 and admits all three mutable host tools at
preparation. The one-row gate repeats the same admission before release build
or graphical takeover, and capture retains the check so an upgrade between
rows cannot mix versions silently. Version recognition requires a complete
version token or its dotted continuation, avoiding substring acceptance such
as treating 1155 as 155. The failed run remains immutable zero-row evidence; a
new run requires this correction in a clean signed candidate.

<!-- END IMPORTED BODY -->
