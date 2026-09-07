---
id: legacy-active-0158
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-01: Milestone 9 commit-pinned promotion passed

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5141–5166. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Commit `727c716d2f762bbed47e1132d7770dc8b92f5015` passed the complete
Milestone 9 promotion ledger. The unattended gate retained the M7 xmonad, M8
Firefox/Vulkan mix, and dual-output libinput-to-kernel-page-flip QEMU evidence.
The physical gates retained native chrome and hot reload, four visible and
interactive Kitty surfaces with pointer focus and VT suspend/resume, xmobar
work-area and pointer behavior without keyboard-focus theft, and graceful
Ctrl-Alt-Backspace emergency recovery with exact TTY restoration.

The final four-Kitty run also resolved a verifier-version mismatch rather than
a runtime failure. Current schema-4 evidence reports one persistent composition
target and frame surface, 34 target reuses across 35 mixed exports, balanced
import-cache imports and evictions, zero replacements or recreation, and 35 of
35 renderer-worker completions. The verifier now checks those persistent
resource invariants and its mutation suite rejects reuse gaps, cache debt,
worker failure or incompletion, and excessive worker latency. Native chrome
and hardware evidence adopted from the immediately preceding runtime-identical
commit retain explicit source provenance; xmobar and emergency evidence were
captured directly on the promoted commit.

This is the development-session promotion point, not installed daily-driver
promotion. The recorded lifecycle still uses source builds and manual service
ownership. Physical Firefox, installed-session cycles, and workday soak remain
Milestones 10 through 12.

<!-- END IMPORTED BODY -->
