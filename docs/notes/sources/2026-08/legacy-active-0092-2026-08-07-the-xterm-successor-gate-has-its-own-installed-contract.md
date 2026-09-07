---
id: legacy-active-0092
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation", "tooling", "architecture"]
---
# 2026-08-07: The xterm successor gate has its own installed contract

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3055–3081. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The earlier xterm pass on `56dad4de` proves the intended physical shape but
predates the installed `4c312142` renderer-handoff successor. Recording the
next xterm launch as an ordinary login would preserve its logs while allowing
the generic cycle verifier to ignore terminal identity, work-area geometry,
and retained-image restoration. That is not a sufficient regression gate for
the auxiliary-pixmap failure that originally blocked startup.

The installed artifact now exposes `sophia-xterm-proof` as a text-VT command,
without adding a seventh greetd choice. It selects xterm and reserves a
schema-4 `record_kind=xterm` attempt before graphics takeover. Runtime identity
records xterm's historical `-version` output and executable digest; the common
identity verifier remains backward-compatible with archives created before
that field existed, while the xterm run-set verifier requires it.

The dedicated session verifier derives both work areas from runtime geometry,
requires one bounded top reservation on each output, and proves that xterm's
source pixels match a symmetrically inset target inside the primary work area.
It also requires ordered renderer capture, drained VT quiescence, equal-count
restore, seat reacquisition, and new xterm pixels plus a primary retirement
after resume. Normal logout, zero unexpected protocol errors, drained native
and application ownership, an untriggered guard, and exact KD/termios recovery
remain mandatory. Fixture mutations reject missing work-area, presentation,
handoff, resume, logout, cleanup, and identity evidence. A fresh installed
physical run remains the acceptance boundary.

<!-- END IMPORTED BODY -->
