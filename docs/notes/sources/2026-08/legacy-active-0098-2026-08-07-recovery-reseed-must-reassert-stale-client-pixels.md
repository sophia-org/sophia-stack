---
id: legacy-active-0098
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering"]
---
# 2026-08-07: Recovery reseed must reassert stale client pixels

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3232–3260. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed release `0.1.0-a50dfb672794` received Super+F and launched Firefox,
but did not complete admission. The first three-surface layout configured both
Kitty surfaces from `1276x1422` to `636x1422` and targeted Firefox at
`1276x1422`. It narrowly timed out before the new Kitty pixels arrived, then
correctly restored the committed rectangles. The aborted `636x1422` Kitty
frames retired after that rollback. The committed-layout reseed therefore held
two `1276x1422` pixel obligations while Engine already retained those exact
rectangles.

The full-geometry correction derived X controls only from an Engine rectangle
change or an admission-owned candidate. It did not include an ordinary resize
obligation whose target rectangle already matched Engine. Consequently every
reseed waited for `1276x1422` pixels but emitted no ConfigureSurface for either
Kitty, timed out, restarted xmonad, and repeated. The live run recorded ten
restarts, a dropped focus handoff, and emergency exit. Super+F routing and
process launch were not the failing boundary.

Geometry-control derivation now also includes every retained resize
obligation. That preserves the separation between `moved_surfaces` and pixel
readiness while ensuring a rollback/reseed can reassert the target even when
logical Engine geometry is unchanged. A deterministic regression reproduces
the exact `Engine=1276`, `pixels=636`, `requested=1276` state and requires one
full-rectangle control with `moved_surfaces=0`. The physical recovery canary
requires every committed-layout reseed control to receive its correlated X
Authority acknowledgement. The installed release remains failure evidence; a
new immutable successor must repeat Super+F once before broader gates resume.

<!-- END IMPORTED BODY -->
