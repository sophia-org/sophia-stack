---
id: legacy-active-0116
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-08-06: The soak gate uses generic session evidence

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3853–3870. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The installed soak verifier required a Firefox M8 proof-completion record even
though the ordinary installed Sophia entry does not enable that proof mode. A
documented daily-driver run could therefore never satisfy its own archive gate.
The same verifier also accepted action launches without matching clean exits or
close actions and did not require workspace, resize, held-input, cursor, or
kernel page-flip-clock evidence.

The gate now consumes the generic redacted summaries already owned by the live
session. It requires clean Kitty and Firefox exits, complete close coverage,
repeated focus and workspace transitions, visually committed resizes,
bidirectional selection activity, distinct output identities, drained input
and key state, clean cursor and page-flip clocks, and zero allocator or bounded
ownership failures. This keeps the ordinary installed entry authoritative and
does not add application metadata or payload logging. Focused mutation fixtures
remove each evidence class independently.

<!-- END IMPORTED BODY -->
