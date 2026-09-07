---
id: legacy-archive-0023
date: 2026-07-07
recorded_date: 2026-07-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "shell"]
---
# 2026-07-07: Blind WM And Compositor Chrome

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 392–402. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Sophia WM manages opaque layout nodes, not X11 windows. The WM protocol must not
carry XIDs, namespace IDs, raw titles, app classes, PIDs, or icon pixels.

Sophia Engine is the broker for compositor chrome. It may receive metadata from
Sophia X Bridge, but user-facing titles, icons, trust badges, and attention
state are rendered by the compositor or compositor shell from sanitized chrome
descriptors. This keeps complex layout policy useful without granting it X11
god-mode or namespace visibility.

<!-- END IMPORTED BODY -->
