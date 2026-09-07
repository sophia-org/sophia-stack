---
id: legacy-active-0089
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation", "tooling"]
---
# 2026-08-07: The installed xterm successor gate passes

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2992–3010. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed release `0.1.0-7e18ea3a01e6` completed automatic xterm attempt
`xterm-runs/0003` with a passing immutable result and matching release,
executable, runtime, and two-output identities. Xterm committed a
2556-by-1422 CPU backing snapshot at `2,16`, exactly inset inside the primary
2560-by-1426 work area. The native owner drained before the VT switch and
reacquired both outputs without abandoning scanout.

The recovery evidence follows the corrected ownership contract: the imported
renderer-image count remained exactly zero, while the Engine rehydrated two
nonzero output frames from its retained CPU scene. A new primary frame retired
after seat reacquisition. Super-Shift-Q then committed normal WM logout with no
unexpected protocol errors, no degraded state, exact KD and termios recovery,
clean namespace and X Authority teardown, and no Sophia, xmonad, xmobar, or
xterm process residue. This closes the installed xterm/work-area successor
gate; failed attempts `0001` and `0002` remain retained as launcher and verifier
regressions.

<!-- END IMPORTED BODY -->
