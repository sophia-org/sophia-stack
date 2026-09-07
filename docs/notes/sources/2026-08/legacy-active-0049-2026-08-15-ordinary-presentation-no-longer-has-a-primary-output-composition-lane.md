---
id: legacy-active-0049
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-15: ordinary presentation no longer has a primary-output composition lane

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1540–1575. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The live output runtime now retains the exact root-space logical viewport from
the output authority across both candidate installation and rollback. Ordinary
CPU, DMA-BUF, retained renderer-image, compositor-chrome, software Present, and
hardware-cursor work consumes that placement instead of reconstructing a
horizontal layout or treating every output as origin zero.

Engine owns two new pure facts at the cross-output boundary. The applicable
retirement set is the union of logical outputs intersecting the old and new
root-space surface geometry, so a move cannot update its destination while
leaving stale source pixels. `TransactionPresentationCohort` then joins those
independently submitted and retired logical outputs at the latest output UST;
the transaction generation is its logical sequence, rather than a fabricated
combination of CRTC-local MSCs. Output-local mirror cohorts continue to join
their own physical heads below this reducer.

Production DMA-BUF Present now builds all applicable output snapshots from one
prepared Engine candidate, resolves CPU/DMA-BUF/retained sources once, lowers
every head at native size, and retains one frame identity per logical output.
The client source becomes submitted only after every applicable output submits,
and feedback, source release, committed input geometry, and displayed-image
promotion wait for the final output retirement. Software Present uses the same
retained source-set lowerer and output join; the former synthetic secondary
marker frame is no longer a presentation path. Focus, order, chrome, outline,
resume, and cursor updates likewise cover the complete runtime output set.

Deterministic evidence includes the Engine cross-output move/retirement reducer,
asymmetric output submission and retirement in the Present scheduler, exact
root-space runtime replacement, opaque-head DMA-BUF duplication, retained-image
native geometry, and a cursor projected through negative/raised viewports. The
full backend, 187-test CLI binary, Engine, and renderer-live suites pass. The
remaining acceptance boundary is signed physical evidence for a mixed
mirror-plus-extended topology and native-density authority variants for client
content that can actually provide them.

<!-- END IMPORTED BODY -->
