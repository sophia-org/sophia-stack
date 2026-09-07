---
id: legacy-roadmap-0019
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Classical X11 WM Compatibility

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 2344–2375.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0017-post-promotion-capability-roadmap.md).

<!-- BEGIN IMPORTED BODY -->

- [ ] After native promotion, reinstall the practical xmonad profile and pass
  its bounded physical scenario corpus on one immutable candidate. Require
  exact action and pointer commits, correct Kitty, Firefox,
  xmobar, chrome, and TrueColor behavior, zero lifecycle debt, redacted health
  summaries, and checksummed artifacts.
- [ ] Migrate that profile through the public projection transport without
  changing retained behavior; it must use the same Engine reducer as Hagia but
  may keep its profile translation behind the compatibility adapter.
- [ ] Separate profile-independent synthetic-X lifecycle, layout translation,
  validation, supervision, and recovery from xmonad-specific bindings and
  request patterns. Keep one shared conformance suite for every compatibility
  profile.
- [ ] Define profile admission criteria: a named upstream WM and version,
  frozen configuration, minimal captured synthetic-X request surface,
  complete opaque-action map, deterministic layout/focus/workspace/restart
  tests, and one real installed-session proof.
- [ ] Add classical WMs incrementally from retained user workflows. Likely
  candidates include i3, dwm, and qtile, but ordering follows user demand and
  evidence rather than nominal X11 compatibility.
- [ ] Consider a conventional GTK3 desktop profile such as Xfce as the driver
  for X11 compatibility completeness: EWMH coverage, `_NET_WM_STRUT_PARTIAL`
  work-area reservation, and tray/XEmbed admission. Such a profile draws its
  own pixels and can never exercise a display-list interface, so it is
  compatibility evidence only and must not be cited as `sophia_shell_v1`
  evidence; see `docs/sophia-shell-v1-direction.md`.
- [ ] Reject profiles that require real client metadata, global X server
  ownership, drawing through the fake server, raw input, arbitrary command
  execution, or protocol-specific authority below Engine. Supply missing
  metadata, shell, and session behavior through their proper bounded brokers.

<!-- END IMPORTED BODY -->
