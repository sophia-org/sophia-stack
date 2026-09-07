---
id: legacy-active-0337
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-26: fixed-extent Vulkan smoke exposed a stale-control teardown race

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10618–10634. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Physical `vkcube --wsi xcb` evidence advanced far enough to create and frame
  the fixed-extent surface, then the session exited with
  `X11 route targets unknown client 3`.
- The final records show a pointer focus request for the new surface followed
  by the fatal route error. The client worker had disconnected between the
  Engine focus decision and the bounded control-broker delivery.
- A disappearing application is normal frontend lifecycle, not an X authority
  failure. The broker now returns a distinct `ClientGone` control acknowledgement
  for that race, and the live owner retires that stale target without ending
  the graphical session. Backpressure and registry corruption remain fatal.
- The physical run also created a blank frame without a vkcube presentation.
  The dedicated proof launcher now enables redacted X11 and Present tracing so
  the next run can distinguish client-side exit, rejected Present validation,
  and feedback delivery without adding application-specific engine policy.

<!-- END IMPORTED BODY -->
