---
id: legacy-active-0127
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy"]
---
# 2026-08-06: Retained focus damage keeps the native visual owner

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4135–4168. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first long idle-efficiency attempt could stop after an arbitrary successful
focus transition. X11 focus and the external WM transaction both committed,
but no retained native submission followed. Reduced diagnostic runs reproduced
the stop earlier and showed that the production runtime had no native owner for
the cycle that observed the focus change.

The CLI had treated a coalesced authority batch as a reason to withhold native
scanout from the entire Engine cycle. That decision was valid only for building
a redundant CPU frame. Retained DMA-BUF projection and Engine-owned chrome use
the same native owner without requiring a new CPU frame. Because the runtime
had already committed the new focused surface, the next batch observed no
change and could not recover the omitted repaint.

CPU-frame deferral is now advisory only for CPU composition. Every
native-enabled production cycle retains access to the native visual owner, and
the backend initializes native frame state only when that cycle actually
contains a new CPU frame set. A focused regression fixes the ownership policy;
diagnostic markers preserve the queue boundary for future renderer work.

The final diskless virgl proof freezes one real `glxgears` DMA-BUF surface next
to a static CPU/SHM Xterm. Two consecutive production runs each committed 256
Super-J actions and delivered 256 partial, page-flip-retired `RetainedMixed`
submissions without another client Present or CPU submission. Both then spent
two seconds idle with zero repaint, page flip, or client Present. The latest
run recorded 73 imports, 257 cache hits, 73 evictions, 334 balanced worker
requests/completions, a 34 ms maximum worker request, one active output, one
baseline-only output, and clean teardown. The two runs legitimately differed
between two and three startup uploads because the static initial frame may
coalesce; the causal retained window rejects uploads regardless of that startup
choice. Mutations cover lost transitions, full damage, CPU submission, idle
work, weak cache reuse, worker debt, output leakage, and cleanup debt.

<!-- END IMPORTED BODY -->
