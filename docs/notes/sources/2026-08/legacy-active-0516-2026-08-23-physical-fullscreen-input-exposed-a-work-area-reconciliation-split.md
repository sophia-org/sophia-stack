---
id: legacy-active-0516
date: 2026-08-23
recorded_date: 2026-08-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-23: Physical fullscreen input exposed a work-area reconciliation split

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15768–15783. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first signed Tier-0 physical candidate admitted every `Super+Shift+F`
shortcut but committed none. Live evidence showed each action-37 response being
reconciled from the full `2560x1440` output to the indicator-reserved
`2560x1426+0+14` work rectangle. Engine correctly requires a fullscreen policy
placement to equal the full output, so it rejected the reconciled geometry and
the guide remained at its first step.

Public-policy reconciliation now selects its authority boundary per placement.
Fullscreen uses the complete output and is occluded by the compositor-owned
strip; ordinary and maximized siblings use the reserved work rectangle. A
regression combines both kinds in one output and proves that fullscreen does
not widen its sibling's authority. The failed run produced no promotion
archive; a corrected signed physical run remains required.

<!-- END IMPORTED BODY -->
