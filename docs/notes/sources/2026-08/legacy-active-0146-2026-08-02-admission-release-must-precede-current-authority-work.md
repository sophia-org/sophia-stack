---
id: legacy-active-0146
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security"]
---
# 2026-08-02: admission release must precede current authority work

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4679–4697. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first focused selection run reproduced the short black Firefox window.
Live evidence showed that Firefox's 1280-by-1040 fallback retired while an exact
1276-by-1422 standing-target Present was already quarantined. On admission
release, production batch assembly placed the current Firefox group before the
older retained groups. Engine therefore saw generation 50 before generations
3 through 49 and correctly rejected the entire chain as stale against visible
generation 1. The standing-target Present never reached native retirement, so
the temporary recovery extent remained active and the lower tile stayed black.

This is an owner-side authority ordering defect, not an X11 configure or client
geometry defect. Released admission groups now precede the current observed
batch, preserving FIFO generation order for the same surface. The regression
uses one surface with an older released DMA-BUF Present followed by a newer
current CPU update and requires both Engine commits plus final generation 2.
The geometry/admission gates remain fail-closed; a fresh focused physical run
is required.

<!-- END IMPORTED BODY -->
