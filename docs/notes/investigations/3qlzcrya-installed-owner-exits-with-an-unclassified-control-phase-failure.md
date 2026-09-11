---
id: 3qlzcrya
date: 2026-09-10
kind: investigation
status: investigating
tags: [investigation, session, input]
---
# Installed owner exits with an unclassified control-phase failure

## Observed failure

The operator reported the session disappearing during the signing-dialog
troubleshooting that followed desktop acceptance. The last user observation
before the crash was that pinentry-egui opened but did not accept pasted input.
A terminal loopback GPG command was suggested; the exact action immediately
before the crash has not yet been confirmed.

This was installed Sophia `171345049bf620a40b24c48d738329b1f63decaf`, session
`00000001789079343163-ae587303-077c-4f55-b0b6-209bcde4edcc`, owner PID 22902.
The [policy reload investigation](v4geoq2j-policy-reload-compares-independent-configuration-generations.md)
retains the paired binary hashes. The new reload/cursor/VT repairs had been
built and tested but were not installed. Original core bytes had been restored,
and the cleaned desktop profile retained only the verified direct Brave command.

## Retained evidence

All available session logs, identity, manifest and health records were copied
to `.artifacts/t076-desktop-acceptance/session-crash-1789093390/` before another
login could rotate them. The directory and files are private. The ordinary
journal is bounded and its health reports `discarded=110`; this is not a claim
that the complete session history survived.

Record 4833187 at epoch millisecond 1789093346787 reports an owner-loop fatal
error with bounded cleanup and `failure_code=unclassified`. Record 4833198 says
frontend intake stopped, native work drained, renderer images cleared and
presentations shut down. Record 4833209 identifies `phase=control`; the final
result is failed. The installed lifecycle reports exit status 1 and handoff to
the display manager, with `emergency=false`. Recovery reports keyboard and
termios restoration. Owner, Hagia, Narthex and Quickshell processes were gone.

A WM transaction (155) completed around 1789093346287, about 500 ms before the
fatal error, and one surface disappeared from the recorded geometry set.
Both Session control queue and acknowledgement deadlines are 500 ms. A control
timeout following a window action is therefore a hypothesis, not a confirmed
failure kind. The control phase also services layout progress, and the retained
record does not distinguish those fallible branches. Rendering feedback continued
until the fatal error. No emergency chord or new-owner activation is recorded.

## Limits and next gate

The original error text is excluded from the privacy-filtered ordinary journal;
the installed owner wrote its raw output to `/dev/null`. The exact failed
control, rejection or timeout cannot be recovered from these records. Do not
attribute the session exit to an incorrect passphrase, the loopback command,
the browser GPU issue, or the uninstalled picker repairs without further evidence.

Recover the operator's last action, reproduce with a precise candidate, retain
bounded typed control-failure evidence, and repair the confirmed failure path.
The acceptance gate is the same window/control interaction completing while
the owner, input and rendering remain usable. This incident is tracked by
[todo.md](../../../todo.md), independently of the completed t075/t076 launch
checks and the [picker input repair](yifbnqjz-launcher-capture-loses-visible-cursor-updates-during-modal-input.md).

The existing GPG agent was left running. A post-crash no-dialog signing probe
with the configured key succeeded, so cached signing is available again.
