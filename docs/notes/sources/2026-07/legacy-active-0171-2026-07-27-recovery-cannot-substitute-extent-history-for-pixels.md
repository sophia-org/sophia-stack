---
id: legacy-active-0171
date: 2026-07-27
recorded_date: 2026-07-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering"]
---
# 2026-07-27: Recovery Cannot Substitute Extent History for Pixels

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5888–5914. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical vkcube run after retirement-gated DMA-BUF admission still
showed a small tiled border with no cube. The retained log proved that this was
not an XMonad floating decision: vkcube surface 6291456 reached generic
frontend admission, the first two-surface resize timed out, and recovery
transaction 3 published a three-layer layout and focus ring. It never emitted
`sophia_live_visual_admission status=armed`, and no Present retirement belonged
to that surface. All observed GPU retirements remained Kitty surface 2097168.

The escape was the recovery readiness reducer. It allowed
`layout_epochs.committed_size == requested_size` to satisfy a pending
admission, even when the proposal owned no staged transaction for that
surface. A WM proposal could also carry a bufferless planning node without a
size change, leaving the surface outside `admission_surfaces`. The layout then
published geometry, chrome, and focus from extent history alone.

Every `PolicyPending`, `ControlPending`, or `AwaitingPixels` layer now becomes
an explicit admission target whether or not the WM changed its size. Retained
size state remains valid recovery guidance only; admission readiness requires
the proposal's exact staged concrete transaction. The regression recreates an
acknowledged recovery surface with matching retained extent but no pixels and
proves that the proposal stays held, committed layers remain empty, and focus
is not published. A client that never supplies matching pixels is withdrawn by
the existing bounded timeout path instead of receiving an empty compositor
frame.

<!-- END IMPORTED BODY -->
