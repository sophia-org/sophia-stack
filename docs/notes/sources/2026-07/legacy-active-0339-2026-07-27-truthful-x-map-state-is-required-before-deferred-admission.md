---
id: legacy-active-0339
date: 2026-07-27
recorded_date: 2026-07-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-27: truthful X map state is required before deferred admission

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10653–10673. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- A guarded xmonad run started Kitty and xmobar, but no managed surface became
  focused. Super-Enter queued another launch immediately before the startup
  watchdog exited at `stage=not_focused`.
- The isolated real-Kitty authority probe reproduced the failure without DRM,
  a WM, or a display manager. Sophia observed Kitty's created
  `PolicyManaged` windows and Present buffers, but no map intent.
- `GetWindowAttributes` had always reported every known window as viewable.
  Kitty trusted that reply and omitted `MapWindow`, so the new deferred
  admission protocol had no lifecycle edge from which to emit its request.
- X authority now derives the reply from its stored `XMapState`: created and
  policy-pending windows report unmapped, and only admitted/mapped windows
  report viewable. The real-Kitty probe has a deferred mode that requires one
  map intent, one delivered `AdmitSurface`, continuing Present feedback,
  delivered focus, and consumed routed text.
- The corrected probe passed end to end, and the full offline all-feature
  suite passed. Physical xmonad/vkcube verification remains open.
- The contemporaneous elogind diagnostic was unrelated. Session 193 was the
  valid online greetd `_greeter` session on tty7, not a stale Sophia session.

<!-- END IMPORTED BODY -->
