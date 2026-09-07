---
id: legacy-active-0595
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-09-04: schema-4 first row exposes advertised RANDR query gaps

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18808–18853. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first `cp14-schema4` physical row used signed candidate `6f0b4ef2`. Sophia
started with the exact two native heads and committed the requested topology,
but the gate retained no attempt and reported an empty `xrandr` result. With
`SOPHIA_SESSION_STARTUP=none`, the measured application baseline was correctly
empty. That meant the operator saw an application-free desktop whose ordinary
keys were suppressed for lack of a focused surface while the outside controller
waited on its topology predicate. The resulting experience was an apparent
lock. Ctrl-Alt-Backspace remained live, triggered the input guard, shut Sophia
down gracefully, restored and verified TTY3, and returned control there.

The retained session error tally made the topology failure exact: 94 rejected
requests, all RANDR major opcode 132, minor opcode 28 (`GetPanning`). The gate
had discarded `xrandr` stderr and retried the deterministic protocol error, so
its saved output contained only a newline. Sophia advertised RANDR 1.5 without
implementing all read-only queries used by the standard client. The isolated
`x-authority-xrandr-query-smoke` reproduced `GetPanning`, then exposed the next
missing requests as each predecessor was implemented: minor 27
(`GetCrtcTransform`) and minor 23 (`GetCrtcGamma`). With bounded disabled-
panning, identity-transform, and zero-length-gamma replies, the same real
client now exits successfully with 19 requests and `first_error=none`.

Topology admission now retains combined xrandr output and exit status. A
reported X protocol error fails immediately instead of leaving the operator on
an empty display for a 30-second retry loop; other startup races are bounded to
five seconds. Unit coverage checks request decoding, valid bounded replies,
and invalid-CRTC rejection.

The outer handoff independently reported `manager_input=false` before it
restarted greetd. Its coarse record proved that exact tty7 restoration or
readback diverged, but did not retain which of KD mode, keyboard mode, or
termios caused the mismatch. It therefore cannot support a more specific root-
cause claim. Recovery now records expected and observed fields, attempts exact
captured restoration first, and falls back to a verified safe text-console
baseline when exact kernel round-tripping diverges. A separate post-start
health contract permits tuigreet's intentional termios transition while
requiring three stable samples of KD_TEXT, a non-`K_OFF` keyboard mode, and
readable termios, plus the existing proof that a live tuigreet owns the
configured VT. Only that verified VT is activated; any failure still stops
greetd and falls back to verified TTY3.

No comparison row was sealed. The `cp14-schema4` manifest remains bound to the
superseded candidate and must not be retried. A fresh run requires a clean,
signed commit containing these protocol and recovery corrections.

<!-- END IMPORTED BODY -->
