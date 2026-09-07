---
id: legacy-active-0204
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-24: Empty Desktop Input Remains Engine-Owned

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6969–6991. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical xmonad run after controlled stale-Present retirement proved
that Kitty exited normally and both outputs received correction frames, but the
remaining cursor appeared frozen. The live-session policy selected
`ShortcutsOnly` when the startup child had exited and both focus and proof
surface were absent. That mode intentionally discarded pointer events even
though no client surface remained.

An empty desktop now uses the ordinary full physical-input path. Engine still
consumes global shortcuts first, keyboard focus rejects application keys, and
scene hit-testing produces no client pointer target; only the session-owned
hardware cursor moves. This is protocol- and application-neutral and adds no
empty-desktop object or Kitty branch.

The same run exposed four `RANDR:GetOutputProperty` `BadValue` errors that also
occurred in the Kitty-only profile. Conventional RandR property atoms are now
created with the X server's shared atom table. Valid outputs return an empty
property for unavailable hardware identity and `CARDINAL(0)` for
`non-desktop`; Sophia does not invent EDID or connector data. Invalid outputs
and atoms remain protocol errors. A two-output real-Kitty smoke retains this
compatibility evidence without weakening normal-session error policy.

<!-- END IMPORTED BODY -->
