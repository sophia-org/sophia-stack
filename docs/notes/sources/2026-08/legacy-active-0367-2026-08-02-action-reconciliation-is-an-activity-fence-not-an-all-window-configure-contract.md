---
id: legacy-active-0367
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling", "architecture"]
---
# 2026-08-02: action reconciliation is an activity fence, not an all-window configure contract

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11265–11289. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next physical run advanced Firefox through the cross-client refocus
  checkpoint and created its attached popup as the fifth surface. Repeated
  `Super+J` then stalled for three seconds and restarted the compatibility
  bridge with `expected=3 configured=1`. The fifth surface remained
  client-positioned and outside the blind policy set; the failure was not
  transient classification or X metadata routing.
- The bridge had treated every synthetic root `ConfigureNotify` generated from
  the three existing layout nodes as a promise that xmonad would answer with a
  `ConfigureWindow` for every node. Core X11 makes no such promise. In the
  full-height layout xmonad emitted the one policy change it needed and then
  became quiet, so waiting for two invented replies turned a valid partial
  reconciliation into a fatal policy-transport timeout.
- Existing-node geometry is now applied as one ordered batch with one coalesced
  root notification. Profiled actions require pre-injection activity and a
  quiet boundary, while only new `MapRequest` windows remain mandatory
  configure admissions. Post-injection activity is still required and remains
  separate, so stale reconciliation cannot satisfy a later action.
- The process-external bridge regression manages three opaque surfaces, answers
  the coalesced pre-action notification with exactly one configure request,
  then proves a private focus chord returns the post-action focus result. A
  second root notification is rejected by the fixture, preserving a batching
  seam for future synthetic-server event-loop optimization.

<!-- END IMPORTED BODY -->
