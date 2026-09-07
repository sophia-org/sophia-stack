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
