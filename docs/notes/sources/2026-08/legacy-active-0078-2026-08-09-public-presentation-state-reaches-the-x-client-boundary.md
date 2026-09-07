---
id: legacy-active-0078
date: 2026-08-09
recorded_date: 2026-08-09
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "architecture"]
---
# 2026-08-09: Public presentation state reaches the X client boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2662–2685. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The public-policy path now carries each changed fullscreen, maximized,
minimized, or ordinary state through a dedicated bounded frontend control.
X Authority atomically installs `_NET_WM_STATE` and ICCCM `WM_STATE`, routes
selected `PropertyNotify` events, flushes the socket records, and acknowledges
only afterward. The owner keeps the policy candidate staged until that exact
acknowledgement. Timeout or invalidation queues the last committed state as a
separate ordered restoration control.

Those properties become Engine-owned after the first state delivery. Client
replacement, deletion, and delete-on-read are rejected or suppressed, so a
client cannot make later same-state policy commits skip necessary correction.
The protocol-neutral state remains in Sophia/Hagia; X atoms and EWMH rules stay
inside the frontend.

Focused tests cover atomic little-endian property values, invalid combination
rejection without partial mutation, the layout acknowledgement barrier,
rollback, and a routed Unix-socket client observing exact property events and
values while its overwrite and deletion attempts fail with `BadAccess`.
`PolicyLifecycle.tla` already abstracts this as frontend settlement; its
correspondence comment now names presentation acknowledgement. The installed
physical Hagia gate remains required before the roadmap item closes.

<!-- END IMPORTED BODY -->
