---
id: legacy-active-0308
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-07-24: Owner-Loop State And Oversized Tests Split By Domain

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9611–9626. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The remaining live-session owner-loop state now has explicit delivery,
observation, cursor-update, and metrics records. Input-delivery draining is an
owned phase with one state boundary instead of a macro mutating seven ambient
delivery variables. The 168-line owner-loop facade initializes resources and
state, then delegates lifecycle, authority, input proof, physical input, and
completion to bounded phase owners.

Oversized test programs were split along real ownership seams: live-session
presentation, Engine rendering transactions, runtime process supervision,
atomic-scanout retirement, and native page-flip decoding. The source-layout
audit no longer reports any test program at 800 lines. The split modules reuse
their parent fixtures and preserve the same behavioral assertions; focused
CLI, Engine, runtime, and all-feature backend suites pass.

<!-- END IMPORTED BODY -->
