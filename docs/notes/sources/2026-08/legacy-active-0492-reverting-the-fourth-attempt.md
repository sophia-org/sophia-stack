---
id: legacy-active-0492
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: 5fea22a7b9a474c04e726edefd7ff4bdf119b8df
committed_at: 2026-08-21T20:45:47-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# Reverting the fourth attempt

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15069–15099. The heading has no date. Its first recorded addition is commit
`5fea22a7b9a474c04e726edefd7ff4bdf119b8df` (2026-08-21T20:45:47-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The chrome diagnosis is right and the change is out of the tree. Layout timeouts
went from one per run to seven the moment the conversion landed, and the session
ended with a layout transaction still pending. A follow-up that sent a
content-size request alongside the converted extent changed nothing, which is the
useful part: my explanation for the regression -- that the client was never told
to resize -- predicted an improvement and produced none, so it was wrong too.

Four attempts, and the count is worth being exact about. Two fixed real defects
that were not the reported one. One shipped a regression. One repaired half of
that regression and did not repair it. The reported symptom is unchanged from
where it started, and the operator has spent six physical runs on it.

What went wrong is not any single mistaken diagnosis; those are ordinary. It is
that I kept treating a plausible mechanism as a finished one. The check that
would have caught three of the four costs nothing: take the proposed repair, work
the arithmetic forward to the geometry the evidence reports, and see whether it
lands there. Relaying out could not have moved a rectangle on a path that never
applies clearance. A size request could not have been the missing piece when
adding it changed no number in the log.

The revert is the right end to it rather than a fifth attempt, because the next
step is not a guess about geometry, it is evidence that does not exist yet: what
the peer proposed, what was requested of the client, and what the client
acknowledged, per cycle. Three diagnoses stalled earlier in this same area for
the same reason, and adding `sophia_live_head_border` turned the fourth into
reading a log rather than inferring one. The lesson did not generalise on its own
and is written down here instead: when a second attempt at one symptom fails,
stop fixing and start instrumenting.

<!-- END IMPORTED BODY -->
