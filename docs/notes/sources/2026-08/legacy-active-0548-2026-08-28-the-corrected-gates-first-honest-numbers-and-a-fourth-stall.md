---
id: legacy-active-0548
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-28: the corrected gate's first honest numbers, and a fourth stall

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16971–17008. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The run on the corrected correlation produced thirteen good samples before a
page-flip stall ended it, and those thirteen are the first physical
input-to-photon figures Sophia has ever recorded that measure what they
claim. Full chain 52-77 ms; dwell-to-submit 36-62 ms; submit-to-page-flip
13-15 ms. The previous run's 32-61 ms was the same machine measuring the
wrong flip: correcting it moved every number up by roughly twenty
milliseconds, which is the stale render the old predicate was quietly
accepting. Queue dwell and submit-to-page-flip are unchanged and still
inside budget; the whole overage remains in dwell-to-submit.

The stall that ended it was the fourth of the evening and the first on the
primary. Head 1 submitted its second flip -- the empty startup desktop, frame
3 -- and the kernel never delivered a completion event: `submissions=2
retirements=0 callbacks=0 ever_retired=false`, with DP-2 holding nothing
outstanding. The session never armed, so nothing was measured, and the
harness nonetheless failed the whole run: the retry classifier recognized
only stalls after proof completion.

That was too narrow. There is exactly one window where a stall must be fatal
-- armed but not finished, where the stall may be the very thing the sample
was measuring. Before arming nothing has been measured; after completion the
measurement is already taken. The classifier now keys on the stall record and
the terminal error rather than one wrapper message, because the same fault
surfaces through the completion drain after a proof and through bounded
cleanup before one, and it refuses only the armed-and-unfinished window. All
three real logs classify correctly: the pre-arming stall that ended this run
and the post-proof stall from the previous one are retryable, a clean session
is not.

Four missed vblanks in one evening across both connectors, one at startup
with zero prior retirements, is a host-level pattern rather than a
coincidence. Sophia's detector and forced detach behave correctly each time.
Worth investigating separately at the driver level -- amdgpu panel self
refresh and DP link training are the usual suspects for a completion event
that never arrives on an otherwise healthy commit.

<!-- END IMPORTED BODY -->
