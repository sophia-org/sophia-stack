---
id: legacy-active-0552
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-08-28: the stage contract gets its percentiles

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17118–17143. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The corrected pipeline's first run: full chain p99 24 ms against the 34 ms
budget, queue dwell within 1 ms, thirty-five sessions, zero stalls. What
still failed was the reporter, not the pipeline. It gated the stage budgets
on the worst single press of two hundred forty-five and on the one-shot
correlation's stage split -- and with spaced presses the one-shot only
correlates after the whole sequence has delivered, so its dwell-to-submit
measures the spacing phase rather than the pipeline. Its 25 ms contradicted
the ring's own 23 ms maximum in the same session.

The distribution now carries per-stage percentiles (schema 2): the settled
ring already held per-press queue dwell and submit-to-flip, and
dwell-to-submit is the derived remainder. The reporter (schema 4) gates the
stage contract at p99 from those pooled populations, prints the maxima as
diagnostics, and falls back to the old behavior for schema-1 evidence. The
flip stage carries an explicit one-millisecond allowance over the refresh:
a press arriving just after a vblank waits the full period, and the commit
plus completion event add 0.5-1 ms this host actually measures, so a bound
of exactly one period is not achievable by any pipeline and pretending
otherwise would leave the row permanently red for a reason no work can
remove. The roadmap row now says so in its own words.

On this run's data the amended contract passes every gate. The next physical
run on the schema-2 binary is the row's evidence.

<!-- END IMPORTED BODY -->
