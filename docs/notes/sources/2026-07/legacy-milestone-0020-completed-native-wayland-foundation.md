---
id: legacy-milestone-0020
date: 2026-07-12
recorded_date: 2026-07-12
date_basis: first-heading-commit
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
date_commit: a633d2acfddcddc4f1661b93dc53168082f3f9d0
committed_at: 2026-07-12T17:34:30-04:00
---
# Completed Native Wayland Foundation

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 465–484.
Date from the first addition of this heading in commit `a633d2acfddcddc4f1661b93dc53168082f3f9d0`
(2026-07-12T17:34:30-04:00); it does not date every event or later edit.

<!-- BEGIN IMPORTED BODY -->

- [x] Removed XLibre concepts from Engine input and surface records, replacing
  them with protocol-neutral surface routes and authority-local IDs.
- [x] Added a Smithay-backed private Wayland authority for `wl_surface`,
  `xdg_toplevel`, SHM, bounded linear DMA-BUFs, frame callbacks, buffer release,
  keyboard, and pointer delivery.
- [x] Ran real Kitty over the private Wayland socket with `DISPLAY` removed and
  changing nonzero SHM frames through Engine.
- [x] Made the installed Kitty launcher use the native Wayland authority and
  native KMS path while preserving the independent TTY recovery guard.
- [x] Made the XLibre bridge an opt-in research feature excluded from the
  production dependency graph and default workspace members, then moved the
  crate, bridge-only CLI, patches, scripts, fixtures, and protocol notes into
  the non-workspace `research/xlibre` archive.
- [x] Added direct EGL DMA-BUF import with no CPU readback and delayed Wayland
  presentation feedback until the matching KMS submission is observed.

---

<!-- END IMPORTED BODY -->
