---
id: legacy-active-0113
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-06: The daily-driver churn gate waits for settled admissions

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3785–3805. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Signed commit `5fbfc849` passed the strict unattended `xmonad-m8-soak` gate
after the launch barrier began waiting for session-owned admission rather than
an action's incidental layout or focus record. The two-output QEMU session ran
for 1,901,036 ms and completed 25 terminal, Firefox, and launcher cycles, 75
close actions, and 11 scheduled WM-bridge recoveries. Every recovery preserved
layout. The session routed 663 expected input events, recorded 50 expected and
zero unexpected protocol errors, retired 338,220 native page flips without a
rejected callback, and drained with no pending WM, action, input, frontend,
namespace, Xauthority, or native-cleanup ownership.

This is the bounded unattended precursor to Milestone 12, not a substitute for
its physical gates. The strict verifier requires at least 30 minutes, 20 cycles,
60 close actions, two layout-preserving bridge recoveries, the complete Firefox
workflow, clean health and cleanup summaries, and normal guest completion. The
exact revision is packaged as immutable release `0.1.0-5fbfc849fb63` and is now
the `/opt/sophia/current` target. Installation remains separate from the
evidence run, so the next acceptance boundary is physical use of that unchanged
release.

<!-- END IMPORTED BODY -->
