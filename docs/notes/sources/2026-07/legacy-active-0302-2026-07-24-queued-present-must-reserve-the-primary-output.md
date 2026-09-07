---
id: legacy-active-0302
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-24: Queued Present Must Reserve The Primary Output

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9358–9374. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical run after VT takeover showed one working hardware cursor but
no Kitty pixels. The capture proved Kitty created and mapped its 2540x1390
window, submitted Present transaction 1, and routed pointer motion and button
events. It also proved exact KD and termios recovery. Mixed scanout never
submitted: after a primary CPU frame retired, the async service repeatedly
submitted another CPU frame until the startup watchdog expired.

The async coordinator orders Present before pending frames, but a Present phase
is an attempt rather than proof of submission. If that attempt observes a
transiently blocked primary, advancing to the pending-frame phase can fill the
same primary immediately and starve Present indefinitely. A queued Present now
reserves the primary from pending CPU submission. Idle secondary outputs remain
eligible, preserving independent multi-output progress. Regression coverage
proves both the primary reservation and secondary eligibility.

<!-- END IMPORTED BODY -->
