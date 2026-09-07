---
id: legacy-active-0140
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling", "architecture"]
---
# 2026-08-05: the semantic preflight restores source ownership boundaries

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4472–4494. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The commit-pinned semantic gate stopped before QEMU because seven Rust files
exceeded the reviewed 1,000-line source boundary. Six had direct ownership
seams: WM admission tests, layout commit reduction, native one-shot rendering,
X input-event writing, routed X delivery, and X routing tests. Each domain now
lives behind its existing facade without changing its public API or runtime
order.

The remaining file is the single ordered X11 connection lifecycle, from setup
through request dispatch and disconnect cleanup. Splitting its control flow
would scatter one tightly coupled algorithm, so it has one explicit temporary
cohesion exception. The source-layout audit, formatting checks, offline
metadata, and complete all-features suite pass after the extraction. The
commit-pinned semantic gate remains the next acceptance step.

The next preflight exposed an independent fixture-ownership error. The generic
TTY verifier still consumed the integrated Firefox fixture even though the
shortened Firefox workflow had intentionally removed the last-window exit and
desktop relaunch sequence. A dedicated generic TTY fixture now owns that
sequence, two-output retirement, and clipboard evidence. Firefox fixture
changes can no longer invalidate the unrelated TTY verifier.

<!-- END IMPORTED BODY -->
