---
id: legacy-active-0235
date: 2026-07-14
recorded_date: 2026-07-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security", "tooling"]
---
# 2026-07-14: Engine Topology, Authority XKB, And Resize Quarantine

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7891–7911. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Milestone 3 now has three explicit boundaries. First, live Engine output records
become a validated, generation-bearing, at-most-16-output snapshot; X setup and
populated RandR CRTC/output/mode replies derive from it without exposing KMS
object identity. Dynamic RandR subscriptions and events remain separate work.

Second, Engine sends physical input as a `RoutedInputRequest` containing its
selected Sophia surface and global/local coordinates. The X frontend resolves
the owning worker, then a dedicated authority thread owns per-seat xkbcommon
state using a bounded explicit RMLVO configuration. `XKEYBOARD` remains
unadvertised until its map/name/state request surface is implemented.

Third, an X resize transaction whose pixels match a pending requested size is
quarantined with its CPU update. Neither can mutate the committed scene while
the old geometry is active. When every requested surface is ready, the staged
geometry and pixels replay together; timeout discards them and retains the last
committed scene. This closes the path that could display a large white drawing
update at the old top-left geometry, but hardware resize promotion still needs
an operator proof and rollback evidence.

<!-- END IMPORTED BODY -->
