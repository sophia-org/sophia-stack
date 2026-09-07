---
id: legacy-roadmap-0008
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Current Limitations

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 516–577.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0006-boundary-and-capability-ledger.md).

<!-- BEGIN IMPORTED BODY -->

- Release `0.1.0-4c3121421f12` remains installed. Automatic Firefox attempt
  `0002` passes the dedicated immutable gate, including exact renderer-image VT
  capture/restore, the browser and floating-dialog workflow, clean normal
  logout, zero unexpected protocol errors, and no retained profile. The
  remaining promotion work is outside this focused browser boundary.
- The xmonad bridge has one flattened `active_workspace` policy view even
  though the session descriptor can express output/workspace mappings. True
  independent per-output workspaces require output-scoped active-workspace
  state throughout the bridge and Engine transaction path.
- The xmonad compatibility profile now exposes opaque focus-master,
  swap-master/up/down, shrink/expand, master-count, reset-layout,
  toggle-floating, and sink actions without expanding the WM wire format.
  Focus-output, move-to-output, output-scoped layout state, and supervised WM
  restart remain compatibility work.
- `ThreeColMid`, `Tall`, `Mirror Tall`, `Full`, and `Spiral` have exact
  configured-bridge geometry coverage. Xmonad's `Tabbed` layout depends on
  title-aware, WM-drawn decorations and therefore does not fit the blind-WM
  contract. If tabs are admitted later, Engine must draw metadata-free native
  tabs.
- Xmobar can render, reserve a work area, update, and retire cleanly, but it has
  no private workspace/layout/focus feed. Such a feed must be emitted by
  Engine or a trusted shell broker and contain only workspace number, approved
  layout name, and focus state—never window titles or client identity.
- Application placement cannot use xmonad class/title rules. Requested launch
  placement, such as Firefox on workspace 2, must come from trusted launch
  provenance or explicit user action.
- The X setup catalog, passive colormap ownership, RGB16 allocation,
  named-color lookup, color query, and error paths now agree on fixed 24-bit
  XRGB and 32-bit ARGB TrueColor semantics. The remaining color gate is a
  physical captured-pixel proof on the successor installed candidate. The
  proof command and fail-closed archive verifier are implemented but do not
  count as physical evidence until a new installed run passes.
- The daily-driver session still uses the `classic-shared` X namespace. The
  confined-group architecture and most portal executors are not yet promoted
  into the normal Firefox session.
- Tray/XEmbed, lock, screenshots, wallpaper, audio control, and general prompt
  UI are shell or portal work. `xcompmgr` must never run under Sophia because
  Sophia is the compositor.
- Full classical-desktop parity remains explicitly deferred and ownership
  split. A trusted shell/session broker must own arbitrary launch, lock,
  screenshots, wallpaper, audio/media/eject, and launch-placement provenance.
  Engine chrome must own metadata-free tabs, decorations, and fullscreen
  presentation. A redacted shell feed must own workspace/layout/focus labels.
  The X compatibility layer still needs tray/XEmbed, output focus/move,
  optional input aliases such as Super+Tab and button-2 swap-master, and
  evidence-backed per-WM profiles. None of these may introduce titles,
  classes, XIDs, PIDs, namespace identity, or executable commands into the
  blind WM boundary.
- The compatibility bridge currently has a complete xmonad profile, not broad
  classical-WM compatibility. Other WMs such as i3, dwm, and qtile require
  separate evidence-backed profiles against the same synthetic-X and Sophia
  WM boundaries; no profile may grow into a proxy for the real X Authority.
- The small bundled native WM proves the direct API and native chrome path, but
  it is not the intended full desktop policy. Hagia is the planned first
  demanding Sophia-native WM and shell family: a blind spatial-policy process,
  an optional separately authorized shell, and ordinary Sophia session and
  portal services.

---

<!-- END IMPORTED BODY -->
