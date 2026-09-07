---
id: legacy-archive-0007
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-10: xmessage as Text And Cursor Probe

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 159–178. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`x-authority-xmessage-smoke` now launches `xmessage Sophia` against
Sophia X Authority and reaches Engine/Runtime committed authority transactions
with no X protocol error. The compatibility work stayed probe-driven: xmessage
added bounded `CreateGlyphCursor`, `FreeCursor`, `SetClipRectangles`, and
`PolyText8` handling. Cursor support is resource lifecycle only; text drawing
reduces to conservative core-draw damage rather than full font rasterization.

The external real-client harness now fails any observed X protocol error even
when a long-running drawing client already produced authority transactions.
This keeps `first_error=none` as an enforced smoke invariant instead of a
display-only field.

The passing reduced evidence was `outcome=proof_window_killed`, `requests=136`,
`opcode_count=23`,
`opcodes=1,2,8,9,16,18,20,43,45,47,49,50,53,54,55,59,60,61,70,72,74,94,98`,
`transactions=9`, `runtime_committed=9`, `runtime_surfaces=9`, and
`first_error=none`.

<!-- END IMPORTED BODY -->
