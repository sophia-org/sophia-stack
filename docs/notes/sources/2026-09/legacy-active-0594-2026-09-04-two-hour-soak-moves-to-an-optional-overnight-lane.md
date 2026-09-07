---
id: legacy-active-0594
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-09-04: two-hour soak moves to an optional overnight lane

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18788–18807. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The comparison's three two-hour rows made a diagnostic matrix consume six
hours of attended critical-path time after its 36 short rows. They also made
reference-compositor durability a prerequisite for closing Sophia's milestone,
even though reference performance is explicitly not a Sophia correctness gate.

The schema-4 comparison contract now separates lanes. Ordinary `prepare`
creates the complete 36-row interactive matrix: four workloads, three
repetitions, and three rotated stacks. `prepare-soak` creates an independent
one-row Sophia two-hour durability run. Each run keeps its own signed candidate,
host identity, immutable schedule, raw attempts, checksums, verification, and
report. Completing or omitting the optional run cannot change the required
matrix's result.

The soak verifier and 7,200-second capture path remain intact for overnight
use. Milestone 14 now closes on the required matrix and its existing absolute
correctness, resource, teardown, and refresh-relative latency evidence; the
two-hour run adds durability confidence but does not block productive work.

<!-- END IMPORTED BODY -->
