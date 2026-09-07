---
id: legacy-milestone-0015
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-07-18 Interactive WM API And Legacy Bridge

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 346–366.

<!-- BEGIN IMPORTED BODY -->

- [x] Negotiated WM API version 2 with fail-closed capability and binding
  validation before any layout or action traffic.
- [x] Added Engine-owned physical shortcut routing, nine Engine-owned
  workspaces, atomic focus/workspace/layout effects, and approved named session
  actions.
- [x] Exercised the same blind API through the native demo WM and the generic
  legacy-WM bridge's metadata-free xmonad profile.
- [x] Passed the unattended two-output QEMU gate with real xmonad, three real
  xterm surfaces, physical virtio input, focus/layout/workspace operations,
  terminal launch, close, logout, bridge restart, and preserved committed state.

Commit `d6ee120` satisfies the milestone exit. The retained `xmonad-m7`
scenario and strict evidence verifier remain the regression contract.
Machine-specific DRM and input runs are optional compatibility diagnostics.

---

---

<!-- END IMPORTED BODY -->
