---
id: legacy-active-0059
date: 2026-08-14
recorded_date: 2026-08-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-14: continuous geometry is latest-value, cancellation is ordered

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1778–1795. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Engine's floating pointer capture now produces Begin, Update, and End for the
  public policy path while retaining the one-shot completion for API-v7 peers.
  Raw motion and client delivery remain outside the policy process.
- A queued Update replaces only the latest queued Update with the same opaque
  target and interaction kind. Begin, End, Cancel, actions, and unrelated
  targets retain order; the sixteen-request owner bound does not grow.
- Output-topology, VT, and seat security transitions clear Engine capture,
  remove its stale queued values, and prioritize a Cancel. Hagia accepts each
  continuous geometry phase and treats Cancel as a no-op, so revocation cannot
  accidentally commit the last sampled geometry.
- Policy restart increments a locally observed epoch before the next physical
  input drain; an active capture is cleared and a Cancel is prioritized on the
  fresh connection, so it cannot observe an orphan Update. This still does not
  claim the entire interaction row because drag and scroll have no live Engine
  producers.

<!-- END IMPORTED BODY -->
