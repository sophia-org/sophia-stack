---
id: v4ycp9ba
date: 2026-09-06
kind: milestone
status: recorded
tags: [milestone]
---
# Daily-session diagnostics accepted across logout and login

## Result

The installed-session acceptance for t015 and t016 is complete. The user marked
an event from tty3 while Sophia was suspended, returned to the desktop, logged
out, and logged in again. The installed CLI then retrieved the exact earlier
session, marker, build/profile identity, and surrounding VT events. Both its
live snapshot and a new snapshot of its final records passed checksum checks.
Task status lives in [the task ledger](../../../todo.md) and
[monthly completion history](../../../done.md).

## Evidence

Installed source: `4b4f28418829d03191d53e533d7903d07d433633`.
Binary SHA-256: `ef721e789aab1e06eee46ebde04dfcd65d818aa1a0953de83d2d1d665596358b`.
Root profile SHA-256: `73b41fe1f138b56df9f264d74ea275e6bc1fe24d7e2106d601e36764c26026ea`.
The identity journal also retains core and desktop profile digests and completed
WM and native-shell executable digests.

Marked session: `00000001788745936827-954d3556-800f-4929-b3c7-bdb25c873b25`.
Marker: `2c40685a-345f-4d33-bbac-2a0a0cd52aca`.
Subsequent login: `00000001788746137544-5f2416f8-5881-4130-8fea-82813c882929`.
The prior recorder finished at sequence 5440 with zero discarded records and
storage errors; the new recorder was running independently. The installed
`session inspect` command succeeded for the prior ID and marker; its output is
`/tmp/sophia-t015-t016-after-login-inspection.log`.

Under `$XDG_STATE_HOME/sophia/session-investigations/`, preserved copies of the
marked session have suffixes `b9114a77-47d4-418b-b926-069d120b59f1` (live,
nine checksummed files) and `c37d8c2d-184e-4554-bd45-7119c6113790` (after
logout and subsequent login, ten files). Both retain the exact marker and pass
all SHA-256 checks. Earlier crashed-session records also survived these logins.

The [diagnostics investigation](../investigations/e84g9ivq-durable-daily-session-diagnostics-and-incident-markers.md)
records the implementation and deterministic checks. The
[VT investigation](../investigations/tnf5xqrb-vt-handoff-failure-exposed-missing-diagnostic-causes.md)
records the failure exposed by this canary, its correction, and the successful
physical round trip.

## Limits and remaining work

The prior session reports exit status 1 despite orderly quiescence and TTY
restoration. This acceptance proves retained diagnostics across an actual
logout/login, including truthful failure reporting; it does not certify a
clean session exit. The [logout investigation](../investigations/64o6l37g-normal-logout-reports-failure-after-x-protocol-errors.md)
tracks that separate session-health issue under the existing t014/t019 scope.
The broader daily-driver milestone and its earlier gates remain open.
