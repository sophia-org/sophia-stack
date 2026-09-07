---
id: legacy-active-0300
date: 2026-07-23
recorded_date: 2026-07-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-23: Secondary Scanout Cannot Block Primary Present

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9319–9336. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical run with stable Present provenance showed one correct
hardware cursor but no Kitty pixels. Kitty created a 2540x1390 window and
issued Present, yet startup ended with zero committed surfaces. The primary
output was idle while the secondary output still had an unrelated CPU frame in
flight. The async service used one global in-flight bit, so the secondary
retirement prevented the primary mixed frame from ever being scheduled.

Native async service decisions are now output-scoped. Retirement still polls
every in-flight or cleanup-pending output, but a queued mixed Present is blocked
only by the registered primary output. Pending background frames are submitted
only on outputs that are individually idle, so one blocked output cannot cause
another exporter to run or prevent useful work. Submit, callback, and
retirement diagnostics now carry output identity and submission provenance.
The next guarded Kitty-only capture must prove the primary Present commits
while secondary retirement remains independent.

<!-- END IMPORTED BODY -->
