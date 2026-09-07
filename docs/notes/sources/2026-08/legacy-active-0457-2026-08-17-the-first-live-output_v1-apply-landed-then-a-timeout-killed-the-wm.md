---
id: legacy-active-0457
date: 2026-08-17
recorded_date: 2026-08-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy"]
---
# 2026-08-17: the first live output_v1 apply landed, then a timeout killed the WM

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13750–13783. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- With the unrasterizable-demand fix in place, the mixed-output gate reached
  and passed the milestone it exists for: `sophia_output_v1_reference schema=1
  status=settled kind=Committed topology_epoch=2 heads=3 groups=2`. A proposal
  crossed the wire, formed a mirror group plus an extended output across three
  physical heads, and committed. This is the first live `sophia_output_v1`
  apply on hardware; every prior topology had come from a startup profile.
- The run still failed, downstream of that. After the topology committed, the
  proof surface had to reshape 1920x1080 to 1280x1440 for its new head. The
  client did not adopt in time, so the session timed the transaction out and
  handled it exactly as designed — `layout_timeout ... preserved_layout=true`
  with a clean rollback. The reference policy client then classified `TimedOut`
  as fatal and exited. The supervisor restarted it through epochs 2, 3, and 4;
  the fourth exit exhausted the restart budget, `apply` returned `Ok(None)`,
  and the session died with "public WM supervisor did not restart Hagia".
- Cause: `stateless_reference_projection_decision` grouped `TimedOut` with
  `RejectedInvalid` and `Disconnected`. A timeout names a slow *client*, not a
  faulty proposal or a broken connection. The session preserves the committed
  layout and rolls the proposal back, so re-proposing from a fresh snapshot is
  the entire recovery — the same handling `RejectedStale` already had. The
  conformance corpus already requires this client to recover from a timeout
  and does exercise it, but against the reducer rather than the live loop, so
  the live classification drifted without a test noticing.
- Fix: `TimedOut` joins `RejectedStale` as `RetryFreshSnapshot`. Fatal is now
  reserved for faults retrying cannot repair. The regression was rewritten to
  assert both recoverable outcomes and both fatal ones by name.
- Consequence worth watching on the next run: if the reshape was a transient
  during native-scanout drain, the retry settles and the gate proceeds to its
  visual phase. If it times out indefinitely, the gate will now run to its
  bounded end without a settled `sophia_wm_v1_reference` record instead of
  crash-looping, which distinguishes a slow adoption from a reshape that never
  completes.

<!-- END IMPORTED BODY -->
