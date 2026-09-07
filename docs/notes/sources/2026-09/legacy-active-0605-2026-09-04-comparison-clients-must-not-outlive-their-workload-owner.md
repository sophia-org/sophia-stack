---
id: legacy-active-0605
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-09-04: comparison clients must not outlive their workload owner

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19174–19209. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Owner-only run `cp14-schema4-401d2b68`, bound to signed candidate `401d2b68`,
passed the excluded cursor qualification with all four targets and 1,151 motion
events. Its first Sophia row then retained an empty application baseline, one
focused and visible DP-1 workload at settlement and all 60 samples, 60 resource
samples, a complete workload record, and 3,599 contiguous single-delivery
kernel frames over 60.021 seconds. The measurement was staged but correctly
remained partial: the 95-second session deadline entered quiescence with no
authority, CPU, or native work pending, yet the frontend did not drain. Its
two-second bound expired, forced two client cleanup envelopes to cancel, and
returned status 1, so post-teardown finalization did not mislabel the row as
clean.

The capture owner had created only ordinary child processes and stopped only
each launcher PID. A toolkit launcher may exit before helpers that inherited X
connections, leaving those descendants outside any teardown owner. This is an
invalid conformance lifecycle even apart from the two surviving connections in
this run. Every Kitty and Firefox workload now starts as a private process-group
leader. Cleanup sends TERM to the whole group, waits at most two seconds for the
leader, then sends KILL to the group even when the leader has already exited;
all workloads are attempted if one cleanup reports an error. The passive X11
visibility probe is also dropped immediately after its final durable sample.
Normal session shutdown remains strict `StopAccepting` plus drain: the gate does
not convert a proof deadline into forced client disconnection.

The same session reported five `PolyText8` BadLength errors during qualification.
The conformance client had passed a plain ASCII string to x11rb's `poly_text8`,
whose payload is a stream of length-prefixed text and font-shift items. Its
leading `M` was therefore decoded as an impossible item length. Qualification
now uses `ImageText8`, the fixed-text request intended for a plain byte string,
and synchronously checks the background, target, and instruction draw cookies.
Protocol refusal can no longer be mistaken for a successful visual prompt. The
partial remains immutable diagnostic evidence; both lifecycle corrections need
one fresh signed physical row before any comparison result is promotable.

<!-- END IMPORTED BODY -->
