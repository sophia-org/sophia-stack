---
id: legacy-active-0245
date: 2026-07-15
recorded_date: 2026-07-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "tooling"]
---
# 2026-07-15: Milestone 4 Mixed-Presentation Implementation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8171–8203. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The narrow handoff is now implemented without moving protocol or native object
ownership into Engine. The X frontend assigns typed buffer/fence handles and
routes feedback by exact `TransactionId`. `LivePresentationResourceSession`
immediately duplicates frontend registrations into renderer-private ownership,
polls xshmfences, builds mixed CPU/DMA-BUF frames, and retains reusable DRI3
sources separately from individual Present lifetimes. The native EGL path
supports one-to-four-plane EGLImages, clipped placement, alpha blending, and a
single persistent output composition pass.

Engine now exposes a prepared surface commit for asynchronous presentation.
Preparation does not mutate committed state. Page-flip application revalidates
only surfaces touched by the prepared transaction, which prevents stale GPU
callbacks from overwriting a newer version of the same surface while allowing
unrelated CPU surfaces to continue committing. Rejection and disconnect drop
the candidate. Successful native feedback applies the candidate, routes Present
Complete with Flip mode, retires the renderer presentation and idle fence, then
routes Idle. Teardown converts remaining queued work to Skip/Idle and asserts
that no source, fence, presentation, or cleanup debt remains.

The offline all-feature workspace suite passes, including prepared-commit
merge/stale regressions, real xshmfence wait/trigger tests, repeated-pixmap and
deferred-release tests, mixed-frame backend ownership, multi-plane renderer
validation, and exact transaction routing. The schema-14 session evidence adds
mixed-export, acquire-wait, completion, idle-fence, and live-resource counters.
`tools/live_session_milestone4_hardware_proof.sh` pairs the established software
resize proof with a `vkcube`/CPU mixed session, controlled first acquire delay,
one rejected Present, required later Flip recovery, and strict teardown checks.
Its verifier passes positive and missing-mixed-export fixtures. The exclusive
TTY X13 run is deliberately still unclaimed and is the remaining Milestone 4
exit action.

<!-- END IMPORTED BODY -->
