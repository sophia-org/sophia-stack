---
id: legacy-active-0577
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-30: the nested retry guard needs a longer bounded arm window

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18118–18138. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The repeat on signed commit `5617d780` reached the same host tail by a
different path: head 2 retired 246 flips before its next callback disappeared,
with the same empty, routed, clean poller attribution. The attempt detached
cleanly and was retained.

The retry handoff correctly recorded `awaiting_operator`, `operator_ready`, and
`retrying`. The operator then left TTY3 immediately after acknowledgement,
while the child session was still reaching its fresh safety prompt. Its
30-second arm window expired, so the launcher again refused graphics takeover
and restored greetd. This archive is not CP-14.1 evidence.

The shared session launcher now accepts a validated, positive input-guard arm
timeout no greater than 300 seconds while preserving its 30-second default.
The terminal gate leaves the initial attempt unchanged and sets only retried
sessions to 120 seconds after `operator_ready`. That window remains bounded but
allows enough time to return to TTY3 after inspecting a retained attempt.
`source.env` records the chosen retry timeout. This changes operator scheduling,
not Engine failure classification or graphics-takeover safety.

<!-- END IMPORTED BODY -->
