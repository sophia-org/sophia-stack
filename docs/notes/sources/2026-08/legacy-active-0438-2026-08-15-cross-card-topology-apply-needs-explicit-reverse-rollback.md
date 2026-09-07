---
id: legacy-active-0438
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-15: cross-card topology apply needs explicit reverse rollback

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13194–13208. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- A DRM atomic request is transactional only inside one card. Live output IPC
  may change heads on several cards, so a sequence of successful ioctls is not
  one implicit transaction: a later refusal otherwise leaves a mixed desktop.
- The backend now applies cards in stable index order, preserves `Busy` as a
  retry of the same card, and rolls an accepted prefix back in reverse order. A
  first-card refusal is distinguished as failure without physical mutation.
  Each card submit is one blocking `ALLOW_MODESET` request without a page-flip
  event, so its result is synchronous at the coordinator boundary.
- Rollback composition is derived separately from the still-published authority
  snapshot and the live head generations/native sizes. Provisional candidate
  geometry never becomes rollback input. The next slice attaches affine
  candidate and rollback renderer owners to this coordinator.

<!-- END IMPORTED BODY -->
