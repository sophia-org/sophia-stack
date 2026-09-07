---
id: legacy-active-0603
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-09-04: X Present's display clock must come from KMS, not request traffic

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19082–19126. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first interactive attempt prepared from signed candidate `8a3f3802`
reached Sophia and showed the randomized Kitty waterfall, but moving the mouse
still visibly changed its cadence. The attempt is retained under
`cp14-schema4-8a3f3802` as a partial diagnostic, not a performance row: the
operator changed to tty2 about ten seconds into measurement, so the remaining
capture window observed a quiesced Sophia session.

The workload producer was not reading input. It wrote and flushed one
deterministic random-number line before each 16 ms sleep, while Kitty presented
through DRI3 DMA-BUFs. The exact coupling was downstream. X request dispatch
allocates transaction IDs from one connection-wide sequence. Engine correctly
uses that identity as a causal presentation-cohort sequence, but backend-live
then exposed it as X Present's media stream counter. Kitty's mouse-driven X
requests could therefore change the counter gaps reported in CompleteNotify;
request traffic, rather than physical refresh, had become the client's display
clock. Logs showed displayed Present transactions advancing by irregular
increments while output cadence alternated between roughly 16.7 and 50 ms.

Backend-live now retains the real KMS `(ust, msc)` retirement evidence for each
output in a submitted DMA-BUF cohort. Each cohort binds one applicable output
before submission, so callback order cannot switch clock domains; that output's
paired physical clock reaches CompleteNotify after every output has retired.
Transaction identity is used only for ownership and settlement. Software
Present uses the same fixed-output physical evidence instead of repeating the
transaction-as-MSC conversion. A regression deliberately separates transaction
ID, UST, and MSC and requires the selected physical pair to survive a join whose
other output retires later. The protocol still owes the complete window/CRTC
selection model and full `target_msc/divisor/remainder` scheduling; neither gap
justifies synthesizing a clock from request IDs.

The same audit found a second real ordering hazard. Physical input immediately
entered a blocking cursor-only atomic ioctl, including before pending authority
work was ingested, and retirement helpers could service the cursor before the
visual runtime consumed and routed ready Present feedback. Input now only
replaces the topology-wide desired-cursor cells. The central native service
first retires and reports frames, admits all primary work, checks flip progress,
and only then services cursor-only work on heads that remain idle. A primary
commit still carries a pending cursor for free. Cursor settlement clears only
the exact position it committed, preserving any newer latest-wins value, and a
native CPU deadline repaint explicitly excludes cursor pixels while the KMS
plane owns them. Focused all-feature scheduler and cursor-owner tests pass; a
fresh signed physical run remains required to validate the visible result.

<!-- END IMPORTED BODY -->
