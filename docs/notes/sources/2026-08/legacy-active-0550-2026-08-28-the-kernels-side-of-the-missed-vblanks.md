---
id: legacy-active-0550
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-28: the kernel's side of the missed vblanks

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17042–17070. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The collector's first capture settles the attribution. The kernel log carries
`amdgpu 0000:03:00.0: [drm] REG_WAIT timeout 1us * 100 tries -
dcn32_program_compbuf_size` in bursts that align with the latency runs almost
one line per session: about thirty-five during the thirty-five-session run,
twenty during the run that stalled five times, six during the seven-sample
run. Every modeset on this host trips a display-controller register timeout
while programming the compressed buffer, and a fraction of those modesets
escalate to a page flip whose completion event never arrives. Scattered
single occurrences across the log's whole history say the condition is
chronic on this box -- RDNA3, DCN3.2, kernel 6.18.46 -- and today's runs
merely raised the modeset rate high enough to see its tail: each sample
performs two modesets, so a thirty-five-session run is seventy.

This closes the question the schema-2 stall record was built to answer, from
the other side: the fault is below Sophia. Sophia's part -- detector, forced
detach, named evidence, bounded retry -- behaves correctly at every
occurrence. The stall retry budget stands at eight so gate runs complete
despite the host; the one open mitigation is kernel-side, and a kernel
update is the first thing to try when one arrives.

One session-side observation worth keeping: the startup native recovery path
re-modesets a head whose first callback never arrives, but only after 750 ms,
while the hard-stall detector terminates at 500 ms -- so on this host the
detector always wins and recovery never runs. Whether a mid-session
missing-vblank recovery (re-commit rather than terminate) is wanted is a
product decision for its own plan, not a patch.

<!-- END IMPORTED BODY -->
