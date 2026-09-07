---
id: legacy-active-0411
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-08: private X input backpressure is client-local

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12521–12538. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- A routed worker's bounded input queue previously returned
  `ClientQueueFull` through `route_pending`, and both persistent frontend loops
  converted that client-local condition into termination of the shared X
  service. One non-reading client could therefore deny service to healthy
  peers without causing unbounded memory growth.
- Saturation now removes the stalled client's complete sender set. The failed
  tracked route receives `RouteRejected`; later routes cannot keep pressuring
  that endpoint, and sender disconnection leads its worker through ordinary
  cleanup. Unknown and already-disconnected input routes are likewise retired
  without widening the failure domain.
- Focused regressions fill both the Engine-resolved and already-client-addressed
  input paths, prove that the broker remains live, and deliver the next event
  to a separate healthy client. Shared registry corruption and shared
  acknowledgement pressure remain service-level failures rather than being
  mislabeled as endpoint backpressure.

<!-- END IMPORTED BODY -->
