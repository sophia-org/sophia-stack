---
id: legacy-active-0626
date: 2026-09-05
recorded_date: 2026-09-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "architecture"]
---
# 2026-09-05 — The mixed-Present ownership crash family

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20137–20198. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Four session-ending crashes in a row, all in the same square inch of code, all
found by using Firefox on the installed desktop. Each fix was correct and
uncovered the next layer, which is what an untested path looks like when it
first meets real load: nobody had run a composited-and-presented frame under
scene supersession before, because nobody had run Firefox on it before.

The path is `MixedPresent` — a frame the compositor draws that also carries a
client's Present, going to the plane. Three ownership modules existed for the
neighbouring cases (software, composited copy, direct flip) and none for this
one. The retirement code even cited `PresentFlipOwnership`, whose own header
excludes it.

The common error under all four: ownership was judged by what the scheduler
names *now* rather than what the session actually did. The scheduler holds one
present in flight, and the kernel retires what it scanned out, so the two
disagree constantly under load.

The layers, in the order they surfaced:

1. **Retirement judged by currency** (~7 min in). A frame superseded by a later
   present retired after the scheduler stopped naming it, and the retirement
   was called unowned. Fix: a superseded frame this session submitted settles
   instead of ending the session.
2. **Reserved but never recorded** (65 s, clicking Bitwarden). A submit pass
   that finds a frame already in flight records nothing, so `submitted_frame`
   answered `None` for a frame in the kernel. The reservation was the only
   remaining claim. Fix: ownership became one question with three answers —
   named by the cohort, submitted and since displaced, or reserved and never
   recorded.
3. **Zombie cohort** (7 s, Bitwarden). Settling the frame left its cohort still
   waiting for it; the next scene submission found a pending present at the
   submission gate and died. Fix: a superseded retirement skips the present its
   frame can no longer complete, settling the client as Skipped.
4. **Exporter poisoned by the skip** (7 s, launching Firefox). The skip reused
   the topology-quiescence path, which rolls back the skipped frame's renderer
   image — safe when nothing else composes, fatal mid frame-service where the
   exporter is already building the successor. InvalidTarget 55 microseconds
   later. Fix: the supersession skip takes the cohort and settles the client
   and touches nothing else; the quiescence path keeps its rollback because it
   has the quiet this path does not.

What made four crashes cost minutes rather than weeks was the first fix, which
was not a fix at all but a diagnostic: it made the fatal name the frame, both
transactions, and what the scheduler held, and classify which disagreement it
was. Every crash after it was read from a single log line. The one that named
nothing — the submission catch-all — was the one that needed a source trace and
a log-mining agent; it now names its submit status too.

`PresentMixedOwnership` in `validation/tla` now models the path, with three
negative controls: judge-by-currency strands a submitted frame, leave-cohort-
pending violates a liveness property, and each fails on exactly its own rule.
This class is checked now, not merely patched. The lesson recorded for the next
one: a fix drawn from a single trace is a hypothesis, and the honest way to
hold it is to make the next failure legible rather than to assume the first was
the whole shape.

Verified live: the exact sequence that killed each session — `retired_after_
successor` then `present_skipped_after_supersession` — now appears in the log
as handled WARN lines while the session stays up.

<!-- END IMPORTED BODY -->
