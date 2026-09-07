---
id: legacy-active-0174
date: 2026-07-27
recorded_date: 2026-07-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-27: Present Requests Are Not Persistent Scene State

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5970–5998. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical run after truthful deferred admission reached
`frontend_admitted`, but every Kitty Present was rejected before rendering.
The live visual runtime had stored xmobar and Kitty as historical
`SurfaceTransaction` values, cloned the entire table for each Present, and
asked Engine to prepare it under Kitty's newest transaction ID. Engine
correctly rejected mixed batches such as expected transaction 403 with actual
transaction 198. Kitty never entered committed state or focus, and the startup
watchdog exited at `stage=not_focused`; KMS and protocol transport remained
healthy.

Xserver's Present implementation keeps each queued request's window, pixmap,
serial, fences, and timing separate from persistent window state. Niri likewise
uses client transactions as readiness blockers and builds output render
elements from current compositor state. Sophia retains its stronger
`PreparedSurfaceCommit` contract but applies the same ownership lesson: a
queued Present owns exactly one matching surface transaction, while unrelated
surfaces come from Engine's committed baseline.

The production coordinator now prepares one Present candidate and rebases only
its causal Engine generation. Backend input and compositor projections derive
from committed state rather than pending transactions, and page-flip retirement
promotes the candidate before successful feedback. The Present hot path no
longer clones and validates every historical scene transaction. Mixed
xmobar/Kitty identity, malformed-candidate rejection, generation preservation,
and exact retirement are covered by offline regressions; physical startup
reproof remains open.

<!-- END IMPORTED BODY -->
