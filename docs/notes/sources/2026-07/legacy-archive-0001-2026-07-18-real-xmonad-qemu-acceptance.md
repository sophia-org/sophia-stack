---
id: legacy-archive-0001
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-07-18: Real xmonad QEMU Acceptance

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 6–25. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The Milestone 7 diskless QEMU gate boots the production Sophia session with
upstream xmonad, two virtio-gpu outputs, two initial xterms, and physical virtio
keyboard input injected only through QMP. It exercises focus, layout, workspace
view and move, approved terminal launch, close, logout, and forced bridge
restart.

The proof closed three transaction-lifecycle gaps: startup now admits one
unmanaged surface per resize transaction, post-restart input waits for recovery
relayout, and immediate action proposals install their accepted workspace and
session-action effects. The close proof also established the ICCCM fallback:
clients without `WM_DELETE_WINDOW` have their X connection terminated rather
than aborting the desktop session.

Commit `d6ee120` retains the implementation. The evidence contract is the
`xmonad-m7` QEMU scenario plus its strict verifier; machine-specific runs are
optional compatibility diagnostics.


<!-- END IMPORTED BODY -->
