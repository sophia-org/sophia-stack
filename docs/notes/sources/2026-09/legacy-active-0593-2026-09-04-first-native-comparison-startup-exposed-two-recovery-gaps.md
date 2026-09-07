---
id: legacy-active-0593
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-09-04: first native comparison startup exposed two recovery gaps

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18740–18787. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical row crossed TTY, candidate, tracefs, and stack admission and
started the terminal-free Sophia/Hagia session. Native evidence proved two
ready heads, output 1 on DP-1 at 2560x1440 and output 2 on DP-2 at 1920x1080,
followed by a committed and settled two-output topology. The gate nevertheless
timed out in `sophia-launch` before admitting a capture.

The timeout was deterministic adapter drift. Sophia's X authority deliberately
publishes connector-neutral RandR output names (`SOPHIA-1` and `SOPHIA-2`),
while the shared X topology predicate accepted only XLibre's physical `DP-1`
and `DP-2` names. The gate now admits either complete naming domain with the
same primary, modes, and logical positions. A failed predicate retains the last
raw query in the owner-only `xrandr-last.log` instead of discarding the fact
needed to diagnose it.

Failure teardown then restored greetd on its configured tty7, but the visible
greeter did not accept keyboard input. The machine required a hard reboot. The
retained inner recovery record proved only tty3's display mode and termios; it
did not verify tty3's keyboard mode, any previously running keyd service, or
the tty7 state to which control was handed. The launcher also requested greetd
startup and immediately raced it with activation, treating tty7 as an error
because it expected to remain on tty3. Those omissions made the earlier
successful handoff record unsound. The reboot removed the live kernel state, so
the exact stale tty7 field cannot be reconstructed; the missing verification
boundary is independently sufficient to explain why the failure was admitted.

The session launcher now reads back the restored keyboard mode, waits for a
previously running keyd instance, and records those facts without changing the
established schema-3 record consumed by retained verifiers. The outer launcher
captures tty3 and greetd tty7 display, keyboard, and termios state before
takeover. On exit it restores and verifies tty3 first, restores tty7 before
starting greetd, waits for a live tuigreet on the configured VT, re-verifies
tty7, and only then activates it. Any manager readiness or input-state failure
stops the manager and returns to the verified originating text VT instead of
presenting an unverified greeter. A persistent `tty-handoff.log` makes that
decision attributable after `/tmp` loss or reboot.

The launcher authenticates sudo while both input and the display manager are
healthy, refreshes only that existing timestamp for its bounded lifetime, and
uses noninteractive sudo during recovery. This prevents the two-hour soak from
ending at an invisible password prompt; a lost lease becomes an immediate,
recorded recovery failure rather than an unbounded wait.

This is a safety fix, not comparison evidence. No row was sealed, the prepared
run remains non-promotable, and another physical attempt is prohibited until
the corrected candidate is signed and the full offline gate passes.

<!-- END IMPORTED BODY -->
