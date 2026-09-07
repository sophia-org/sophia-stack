---
id: legacy-active-0181
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-07-26: Promotion Separates Semantic Automation From Hardware Proof

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6186–6215. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The accumulated Milestone 9 operator sequence had become both tedious and
weakly reproducible. It asked one person to remember clipboard, workspace,
focus, repeat, launch, close, VT, bar, and teardown gestures across many
physical sessions even though most of those state machines already have
deterministic two-output QEMU or reducer coverage. A missed gesture could fail
the ledger without identifying a product defect.

Promotion now begins with one commit-pinned unattended semantic gate. It runs
the canonical offline local regression suite and the retained M7 xmonad and M8
mixed-application QEMU scenarios, then verifies their exact evidence. Policy, protocol,
application, focus, workspace, clipboard, damage, and teardown semantics are
therefore machine-driven and replayable. The promotion driver can run this
first gate outside TTY3.

Physical evidence is reduced to facts virtualization cannot establish:
native chrome/hot reload, real KMS pixels and lifetime, actual keyboard and
pointer routing, one libseat VT round-trip, client-positioned bar geometry and
pointer behavior, normal TTY recovery, and independent emergency recovery.
The four-Kitty and bar proofs each have a short dedicated verifier. Exhaustive
keyboard/VT, pointer-edge, launch-burst, clipboard, and long xmobar workflows
remain focused diagnostics after their owning subsystem changes; they are no
longer memorization-heavy per-candidate rituals.

This is an evidence-boundary change, not a runtime exception. QEMU never
substitutes for AMDGPU, libinput, monitor, greetd, or emergency-recovery proof.
The Engine, X frontend, WM bridge, and renderer remain unaware of promotion
policy.

<!-- END IMPORTED BODY -->
