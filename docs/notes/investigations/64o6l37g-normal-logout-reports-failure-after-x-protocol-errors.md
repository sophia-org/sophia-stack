---
id: 64o6l37g
date: 2026-09-06
kind: investigation
status: investigating
tags: [investigation, session, x11]
---
# Normal logout reports failure after X protocol errors

## Question

Why does an operator-requested logout return status 1 after apparently orderly
quiescence and TTY recovery?

## Evidence

The marked installed `4b4f2841` session
`00000001788745936827-954d3556-800f-4929-b3c7-bdb25c873b25` completed its VT
round trip. At logout, events 5409–5427 show quiescence completing in 88 ms;
event 5428 records drained native scanout. No owner-loop fatal event appears.
Event 5430 reports five X protocol errors. The wrapper returned status 1,
with emergency=false and termios, keyboard mode, and keyd restored. A new login
started normally afterward.

The final evidence is preserved with suffix
`c37d8c2d-184e-4554-bd45-7119c6113790`, as described in the
[diagnostics acceptance record](../milestones/v4ycp9ba-daily-session-diagnostics-accepted-across-logout-and-login.md).

## Finding and next diagnosis

`live_session/owner_loop/completion.rs` returns an error when
`session_protocol_errors_are_fatal` sees a nonzero tally for a normal session.
This is consistent with the recorded five errors and failed logout outcome.
The reduced log does not identify those requests or prove that this was the
first failing completion check. Inspect the authority's reduced error
classification and distinguish compatibility/probing responses from internal
session failures before changing completion policy. Keep raw application
identity and payloads out of ordinary diagnostics.

The X11 extension implementation is being changed independently; correlate any
repair with the installed executable rather than attributing it to current
working-tree code.

## Scope and validation

No logout behavior has been changed here. The evidence is an observed logout
failure with successful recovery, not another VT crash. Retain it under the
existing [t019 session-health work](../plans/queue-06-4-exercise-real-development-workflows.md#t019)
and the [t014 clean-logout gate](../plans/queue-04-2-establish-the-live-session.md#t014).
It does not reopen the completed diagnostics and incident-marker checks.
