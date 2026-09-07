---
id: legacy-active-0545
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering"]
---
# 2026-08-28: one withheld vblank ended a thirty-five-session run

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16867–16894. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Sample 7 of the first full physical latency run failed after its measurement
was already complete. The proof matched all fourteen events with a pixel
change; nine milliseconds later the session submitted one more blank frame to
DP-2, and that flip's kernel event never came. The completion drain polled the
card every five milliseconds for the full window -- DP-1's event arrived
normally on the same descriptor mid-wait -- and at 501 ms the hard-stall
detector did exactly what it is for: named the head, the ages, the peers, and
terminated with a forced detach rather than trusting an unretirable scanout.
The six samples around it flipped DP-2 in sixteen milliseconds or less. The
missing event is below Sophia; the stall record's `callback_serial=none` is
not evidence of a routing fault, because that field is only assigned on the
mirror path.

What was Sophia's to fix is the blast radius. The harness treated any sample
failure as run-fatal, so one transient display hiccup during teardown -- after
the sample's data existed and after six good sessions -- cost the whole run
and another visit to the console. The runner now redoes a sample when the
failure matches this exact shape: proof completed, then `hard_stall` during
the completion drain. A stall before proof completion stays fatal, because a
session that stalls while measuring may be measuring the defect. Two redos
per run are the budget; the stalled sample's evidence is renamed and retained
rather than overwritten, and each redo is a named record in the run. The
classifier's self-test covers the retryable shape, the mid-proof stall, and
the clean completion, and the real sample-7 log classifies as retryable while
a clean neighbour does not.

<!-- END IMPORTED BODY -->
