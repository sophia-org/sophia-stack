---
id: legacy-active-0149
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy", "security"]
---
# 2026-08-02: floating is WM placement, not an X-authority bypass

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4751–4811. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Repeated Firefox popup work exposed a boundary error in the earlier transient
and EWMH reductions recorded below. Sophia treated transient, dialog, utility,
menu, splash, and popup-like hints as `ClientPositioned`. That removed ordinary
non-override-redirect X toplevels from WM redirection instead of telling the WM
how they should be placed. It also coupled protocol classification to visual
authority, so a dialog could either become another tile or bypass the blind WM
entirely; both outcomes violated the intended architecture.

XLibre's root-window redirect path and yserver's native Rust request reduction
agree on the decisive rule: a non-override-redirect root child remains subject
to the WM's map/configure policy regardless of `WM_TRANSIENT_FOR` or
`_NET_WM_WINDOW_TYPE`. Override-redirect is the protocol bypass. Explicit
desktop/dock ownership is also client-positioned in Sophia because those
surfaces reserve output space rather than participate in application layout.
Transient ownership, functional type, floating preference, and stack order are
separate facts. This section supersedes the earlier log entries that describe
transient or dialog-like types as client-positioned.

The implemented boundary is protocol-neutral. Authority surface and
presentation packets now carry `LayoutNodeKind`, `SurfacePlacementPreference`,
an optional opaque presentation owner, and an explicit bottom-to-top stack
rank. X Authority decodes ordered EWMH types, keeps dialogs/utilities/popups
policy-managed with floating preference, preserves real override-redirect in
map delivery, and applies sibling/stack-mode ConfigureWindow requests through
one ranked stacking table. The wire regressions cover a genuine
`WM_TRANSIENT_FOR` dialog, EWMH dialog classification, override-redirect, and
raise/lower ordering.

Engine and the blind WM own the resulting desktop state. WM API v7 adds
persisted floating state, transactional `SetFloating`, and one completed
pointer-gesture packet containing only opaque surface/output/workspace IDs and
integer start/end positions. The xmonad adapter exposes standard
`WM_NORMAL_HINTS`, `WM_TRANSIENT_FOR`, and `_NET_WM_WINDOW_TYPE` properties to
its private synthetic server. `Super+Shift+Space` toggles floating;
`Super+left-drag` moves and `Super+right-drag` resizes through xmonad's stock
mouse policy. The private server preserves query/grab/warp ordering and real
bottom-to-top QueryTree order, so the adapter receives the final configure
only after the completed gesture.

Pointer capture is owned by Engine. While a drag is active, matching physical
events do not reach the client. Each motion renders a topmost compositor-owned
outline using retained committed pixels; the client geometry and buffer remain
unchanged. The outline and final bridge result clamp the entire frame to the
output containing the gesture start. Release retires the outline and sends one
WM request, so configure, pixels, placement, focus, and floating state still
cross the established atomic commit boundary. Reducer, multi-output,
compositor-border, codec, policy-persistence, and process-external xmonad
regressions lock these seams for later gesture coalescing and rendering
optimization.

The deterministic Firefox milestone no longer pretends that an HTML action is
an X11 dialog conformance test. Its final step uses an in-document `<dialog>`
with ordered ready/confirm checkpoints and no surface-count transition. The
genuine hinted X11 dialog remains a separate authority/bridge regression. This
keeps browser interaction evidence independent from transient-toplevel
protocol evidence and removes the popup admission loop that repeatedly blanked
the owner. A fresh physical Firefox workflow is still required before closing
Milestone 10.

<!-- END IMPORTED BODY -->
