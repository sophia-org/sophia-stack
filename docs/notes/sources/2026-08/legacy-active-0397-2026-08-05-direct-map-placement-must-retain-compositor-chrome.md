---
id: legacy-active-0397
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "shell"]
---
# 2026-08-05: direct-map placement must retain compositor chrome

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12044–12082. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first installed no-WM fallback after direct MapWindow ownership was
  restored mapped Kitty and retired 16 animated Presents. Its 2556-by-1422
  content remained at output origin, however, while the focused two-pixel
  compositor ring extended outside that content. The output clipped the
  negative left and top bands; the right and bottom bands remained visible.
- Border generation was symmetric and X Authority's client geometry was
  coherent. The ownership error was in live layout: centering the first
  policy-managed surface depended on a startup-input proof flag that normal
  sessions always disable. No external WM existed to supply another
  placement.
- Direct mapping now also declares Engine ownership of initial placement. The
  first toplevel is centered inside the first output without changing its
  source extent or its authority transaction. Deferred mapping continues to
  leave final placement and chrome clearance to the external WM.
- A live-reducer regression reproduces the installed 2560-by-1440 output and
  2556-by-1422 Kitty content, requires the compositor target at +2+9, verifies
  that a two-pixel outer ring remains within output bounds, and proves the X
  transaction geometry is unchanged.
- Installed commit `a752ca27` confirmed that target for all 14 retired Kitty
  Presents and composed the focused ring with two-pixel clearance. The session
  ended with clean health, zero native submission or retirement failures, and
  no pending native cleanup; the operator confirmed the complete border.
- The automatic fallback archive was marked failed for an independent gate
  condition. Output 2 completed its synchronous startup modeset and recorded
  one nonzero export, but received no later damage and therefore no
  asynchronous page flip. The then-current verifier required an asynchronous
  retirement on both outputs. That promotion-policy question remains separate
  from the accepted direct-map placement fix.
- The fallback verifier now uses the same contract already retained by the
  integrated Firefox gate: exactly two unique synchronous startup-output
  records prove per-output liveness, and at least one asynchronous retirement
  proves active scene progress. Per-output nonzero export summaries and clean
  native drain remain mandatory. The pass fixture now models an idle second
  output; mutations reject a missing or duplicate startup identity and a
  session with no asynchronous retirement. Installed archive `0004` passes
  this corrected session-level verifier without changing its evidence.

<!-- END IMPORTED BODY -->
