---
id: legacy-active-0391
date: 2026-08-03
recorded_date: 2026-08-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-03: cursor motion must not serialize animated primary flips

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11867–11906. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The installed retained-recovery repair made a 300-by-300 GLX surface animate,
  but continuous pointer motion reduced presentation from a stable 60 FPS to
  16--46 FPS. The clean completion recorded 5,460 physical pointer events,
  1,376 coalesced moves, 342 hardware updates, 1,859 primary-in-flight cursor
  deferrals, a 16 ms maximum cursor update, zero native submission failures,
  and a 41.390 FPS mean with a 66.718 ms p95 frame interval.
- Backend-live was issuing a synchronous cursor-plane atomic commit for each
  admitted move. That commit waited for a vblank and alternated with the
  nonblocking primary page flip on the same DRM device, so correctness
  serialization itself consumed the animated client's presentation cadence.
- XLibre's modesetting driver uses `drmModeSetCursor2` to install a cursor and
  `drmModeMoveCursor` for steady motion, after querying
  `DRM_CAP_CURSOR_WIDTH` and `DRM_CAP_CURSOR_HEIGHT`. Yserver independently
  documents the same primary/cursor atomic serialization failure and landed
  the same legacy-ioctl repair. Niri avoids the race through one compositor
  frame/KMS owner that folds cursor plane state into output presentation;
  river delegates the equivalent ownership to wlroots.
- Sophia now retains one-time synchronous atomic detachment solely to sanitize
  inherited cursor planes before presentation begins. It then hides every
  selected CRTC through the legacy interface, installs the canonical raster
  with `set_cursor2`, uses `move_cursor` on the active CRTC, and performs an
  ordered hide/install/move when crossing outputs. Controller state advances
  only after each successful ioctl, and teardown hides the cursor before
  destroying its dumb buffer.
- Primary in-flight state may defer this one-time initialization, but it no
  longer defers steady legacy cursor movement. The public backend seam contains
  no atomic commit method, and crate-boundary regressions lock driver-cap
  fallback, ioctl ordering, retryable initialization, partial-failure state,
  and primary-in-flight admission.
- Cursor completion schema 4 reports `path=legacy_ioctl`, initialization time
  and deferrals, steady update time, and successful updates while a primary
  flip is in flight. The bounded GLX proof now requires continuous pointer
  motion, at least 55 FPS, at most a 25 ms p95 frame interval, a steady cursor
  update at most 20 ms, positive cursor/primary overlap, and zero cursor or
  native failures. A future Milestone 13 all-atomic implementation must unify
  primary and cursor state under one per-output transaction owner rather than
  restore independent atomic cursor commits.

<!-- END IMPORTED BODY -->
