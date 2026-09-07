---
id: legacy-active-0549
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-28: attributing the missed vblanks before chasing them

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17009–17041. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The evening's run produced twenty more honest samples (44-77 ms full chain)
and five more page-flip stalls -- nine today, accelerating, all one
signature: an early blank-frame flip whose completion event never arrives,
mostly DP-2's second or third submission, once DP-1's before any retirement.

The event path was audited end to end for a Sophia-side eater: one reader on
the card descriptor (`receive_events` in `reader.rs`), a poller whose pending
queue drains or self-heals under backpressure, per-head routing that errors
loudly on an unknown head, and a per-output queue whose rejects are counted.
The silent-loss sites that exist -- an event for an unrouted CRTC, a decode
without a route -- are counted but not logged mid-session, and a session that
stalls never reaches the completion line that would print them. So the
evidence could not say which side of the descriptor the fault was on, and
that, not the stall itself, was the defect worth fixing tonight.

The stall record is now schema 2 and carries the poller's state at the
moment of declaration: pending depth, route count, and the last read loop's
status and decode/reject counts. An event stuck or dropped inside Sophia
leaves pending depth or rejects behind; a poller that is empty, routed, and
last read clean means the completion event never crossed the descriptor and
the fault is below this process. The harness classifier accepts both schemas
so the day's retained evidence still classifies, and the retry budget is
eight for the diagnostic phase -- tonight's rate would exhaust four -- to be
dropped back once the cause is settled.

The kernel side remains one command away: the socklog kernel log around any
stall timestamp. Repeated full modesets -- this box has done hundreds today,
two per sample -- with a completion that goes missing on the flip immediately
after one, at a rate that climbs through the evening, reads like link
retraining or a display that is slower to lock than the commit is to flip.

<!-- END IMPORTED BODY -->
