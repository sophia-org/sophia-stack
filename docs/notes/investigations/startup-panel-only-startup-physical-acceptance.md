---
id: startup
date: 2026-09-06
kind: investigation
status: awaiting-physical-acceptance
tags: [investigation, session, validation]
---
# Panel-only startup physical acceptance

## Question and evidence

Does the repaired ordinary lifecycle remain usable after a panel-only login,
then admit the first application through Super+Enter?

The [original incident](../sources/2026-09/legacy-active-0638-2026-09-06--retire-application-startup-proofs-from-normal-desktop-lifetime.md)
records installed candidate `f323323d`, the unspawned launch transaction, and the
eight-second `not_focused` shutdown. It retains the full diagnosis and check
results. A separate [GTK and stacking investigation](../sources/2026-09/legacy-active-0637-2026-09-06--maximized-stacking-and-gtk-startup-in-the-replacement-session.md)
distinguishes the Ghostty/Thunar RENDER failure from this launch-queue cycle.

## Finding and resolution

Sophia `3d023c07` separates ordinary lifecycle from application proofs. Its full
check passed 2,502 test executions and the retained checks listed in the source
note. Hagia `0e5e86f` separately repairs elevated-window stacking; its conformance
gate passed. The [ADR](../decisions/adr0001-separate-desktop-readiness-from-application-proofs.md)
records the lifecycle decision.

## 2026-09-06 follow-up: packaging and physical acceptance

Release `.artifacts/sophia-0.1.0-3d023c0772a2` was packaged successfully.
Installation required the user's sudo password and did not proceed in the agent
turn. Packaging is not evidence that the running session uses this candidate.
No subsequent physical acceptance has been reported in this investigation.

Continue the existing startup and ordinary-use gate in [todo.md](../../../todo.md)
with a matching installed candidate. Record the actual release identity and the
observed panel-only login and Super+Enter result here. Do not reset unrelated
completed evidence or treat this pending check as a new comparison campaign.

## 2026-09-12: panel-only login accepted on the installed candidate

The operator reinstalled from HEAD and logged in, then reported both checks
passing. Release identity, matching across all three at the time of the run:

| | |
| --- | --- |
| Installed release | `0.1.0-9807cecf6aee`, `/opt/sophia/current` |
| Repository | `9807cecf`, clean, signed, matching `origin/master` |
| Proof preconditions | `status=ready repositories=3`, exit 0 |

This is the matching installed candidate the 2026-09-06 follow-up asked for, so
the packaging-without-installation gap recorded then is closed.

**t005 — panel-only login: accepted.** The session starts on the active desktop
profile with `quickshell-panel` only.

**t008 — shell-owned panel startup: accepted**, on the operator's visual
confirmation that the automatic Tier-0 bar, its fixed top reservation and its
hit targets are gone.

### What this evidence is, and is not

Operator observation of a live session, which is what a `@physical` acceptance
is. It is not instrumented capture: no per-item log, screenshot or work-area
measurement was retained, so the individual sub-checks each task lists --
Super+Enter's terminal mapping, one bar per output, work-area restoration when
the panel stops -- are covered by the operator's overall pass rather than
recorded separately.

A later regression in one of those sub-checks would therefore not be
distinguishable from this record alone. If that granularity matters for a
subsequent comparison, it needs its own instrumented run rather than a reread of
this entry.
