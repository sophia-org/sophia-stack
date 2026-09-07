---
id: legacy-active-0326
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-07-25: WM Socket Wait Leaves The Physical Owner

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10145–10169. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next two-output, four-Kitty hardware cycle completed without a crash,
AMDGPU rejection, resource replacement, or cleanup debt. It recorded a 180 ms
maximum external-WM request, a 100 ms physical-input phase, 246 ms input queue
dwell, and a 210 ms submit-to-page-flip observation. Child reaping peaked at
25 ms. The correlated peaks identify synchronous WM socket waiting as the owner
starvation source; renderer and target lifetimes remained stable.

WM transport now uses a capacity-one worker channel behind a sixteen-entry
owner request bound. Exactly one packet is in flight. Passive request and
completion records carry the transaction ID; the owner correlates them and
rejects a response when the current layout topology or geometry no longer
matches the request fingerprint. Surface removal remains serialized before its
relayout so the latter is planned from post-removal workspace state. The owner
alone validates and commits proposals and applies focus, workspace, launcher,
close, and logout effects. A neutral empty coordinator batch lets a WM-only
transaction reach Engine without waiting for unrelated X11 traffic; it is not
counted as an X authority batch.

Completion now reports owner timing separately from WM transport depth,
rejections, stale responses, queue dwell, and round-trip latency. The external
round trip retains its 500 ms fail-closed bound; it is no longer misclassified
as owner-thread execution time.

<!-- END IMPORTED BODY -->
