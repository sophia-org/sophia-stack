---
id: legacy-active-0011
date: 2026-09-05
recorded_date: 2026-09-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-09-05: scrolling policy and Engine presentation motion

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 355–396. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The operator reported jumps and inconsistent navigation after opening a third
Kitty in installed Sophia `84c109c6`, Hagia
`0c8261f21ab1c6b59eef3f1e48e78dcf205e8fdd`. The audit compared local Niri
`dd75865f` and Triad `fb8fb27`. Live placement logs showed immediate 1,268-pixel
camera steps. Hagia had no presentation timeline, and its overflow calculation
selected a previously visited column by strip/history order rather than the
last committed focused column. An unequal-width three-column probe returned
offset 1,260 where the Niri incoming-neighbor rule selected 1,894. Directional
navigation also wrapped at an outer edge.

The chosen policy reference is Niri, retaining adjacent-monitor handoff and
explicit cycling as separate behavior. Hagia now stores committed camera
anchors per axis, inserts after focus, preserves the view across earlier
removal, restores newly opened column context on close, and selects adjacent
focus on ordinary close. A zero-focus removal snapshot preserves that selected
fallback; an explicit later clear still clears it. Checkpoint v12 migrates v11
and earlier state without inventing camera history.

The rendering boundary is generic: optional WM translation groups describe
shared positional targets, while Engine owns spring time, per-member movement,
GPU composition, output scheduling and presented input geometry. No WM camera
or workspace policy moved into Engine. No shell wire change is required.
Capability bit 12 and extension records `0xFF03`/`0xFF04` leave frozen records
unchanged. Cross-language testing exposed an existing assembly defect that
counted extensions in frozen begin/end counts; assembly now retains the
ordinary-prefix count. Retained source discovery covers all applicable outputs.

The [translation contract](../../../window-transitions.md) records lifecycle, motion off,
limits and the focused physical check. Offline checks cover target continuity,
pixel and frame-geometry identity, negotiated transport, malformed membership,
camera traces, close reconciliation and checkpoint migration. The installed
session remains unchanged; physical smoothness and moving-content clicks are
pending acceptance with the rebuilt Sophia/Hagia pair.

Validation: `cargo xtask check` passed 2,418 Rust test executions, Clippy,
protocol generation, source-layout checks, archive verification, GPU buffer-age
pixel equivalence and verifier fixtures. Hagia `nimble verify` passed 181
Nim cases, shared sequential/restart socket conformance, pregraphics admission,
formatting/layout checks and its existing Alloy, Z3 and TLA+ model checks.

<!-- END IMPORTED BODY -->
