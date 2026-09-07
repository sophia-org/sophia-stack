---
id: legacy-active-0575
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-30: the terminal gate meets the host's lost-vblank tail twice

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18066–18092. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first two physical schema-4 attempts on signed commit `32b555b6` did not
reach the visual question. The first ran head 1 through 307 successful
retirements before submission 309 received no callback; head 2 was still
retiring with an 11 ms outstanding flip. The immediate retry moved the fault:
head 2 stopped after 41 retirements while head 1's peer age was 0 ms.

Both stall records carried the same decisive schema-2 attribution:
`poller_pending=0 poller_routes=2 poller_last_read=WouldBlock
poller_last_decoded=0 poller_last_rejected=0`. No callback was queued, decoded,
or rejected inside Sophia, and the sibling CRTC continued receiving events.
The kernel delta contained no new diagnostic, but the event-path signature is
the already documented below-process missed-vblank condition on this host.
Both sessions forced a bounded detach, restored greetd, and retained complete
failed archives. Neither is CP-14.1 evidence.

Manual repetition is the wrong interface for a known transient. The terminal
wrapper now applies the input-latency gate's bounded-redo principle with a
stricter classifier: it retries only a current schema-2 stall whose poller is
empty, routed, and clean. A pending callback or rejection is a Sophia-side
signal and is never retried. Each attempt owns an immutable `attempt-NNN`
subdirectory; the final attempt alone is promoted to the archive root for
schema-4 reduction and visual confirmation. Eight retries match the established
diagnostic budget, an operator may lower it, and exhaustion fails with an
explicit `page_flip_stall_retry_budget` result.

<!-- END IMPORTED BODY -->
