---
id: legacy-active-0458
date: 2026-08-17
recorded_date: 2026-08-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-17: a stale projection rejection stranded the public policy owner

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13784–13827. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The mixed-output gate on `131efc15` failed with "public WM supervisor did not
  restart Hagia" and lit only two of three monitors. The topology apply itself
  succeeded — `sophia_output_v1_reference status=settled kind=Committed
  topology_epoch=2 heads=3 groups=2` — from a correct three-head start.
- Chain: the commit enqueued one relayout cause; the owner issued one cycle;
  scene facts advanced again before the response landed, so the owner rejected
  it as stale. `settle_rejected_projection` cleared its in-flight state and
  enqueued nothing, leaving an empty queue and no outstanding request. The
  client had mapped that rejection to "retry from a fresh snapshot" and blocked
  in `receive_snapshot`, so it sat until its four-second socket deadline fired,
  exited, and was restarted. Three restarts exhausted
  `RestartPolicy { max_attempts: 3 }` and the session died.
- The two-monitor symptom was downstream of the same cause, not separate. After
  the apply the mirror pair retired three page flips each while the extended
  head retired exactly one: the window manager died before it ever placed a
  surface on the extended output, so that head showed only its
  first-presentation frame.
- Root cause stated as an invariant: the owner re-offers a cycle for exactly
  the outcomes a stateless client retries. The client half already existed; the
  owner half was missing. An invalid rejection deliberately does not re-arm,
  because the scene did not move and re-offering would spin.
- A second site had the identical defect and would have reproduced the same
  death through a layout timeout: `apply_public_commit_result` in
  `wm/commit.rs` cleared the same three fields and queued nothing. Both now
  route through one `settle_public_projection`, so the decision cannot drift
  between them again.
- Re-arming goes through the pending-dirty path rather than a direct enqueue.
  That path already merges into a queued relayout and already defers while one
  is in flight, and it cannot report a full queue — there is no recovery from
  owing a cycle that will not fit.
- A latent defect became reachable and was fixed in the same change: a topology
  update whose relayout cause was rejected as a duplicate dropped the
  replacement affected-output list, leaving a queued cause that could name an
  output which no longer existed. Issuing such a cause fails the session, and
  the re-arm makes a queued relayout far more common.
- Deliberately not added: a consecutive-stale ceiling escalating to a transport
  restart, which is what the legacy path did. The supervisor never feeds
  `ProcessHealthy`, so `restart_attempts` never resets and three lifetime
  restarts always exhaust the budget; escalating would convert a recoverable
  condition into session death, which is the failure being fixed. Recorded as a
  dependency for any future escalation.

<!-- END IMPORTED BODY -->
