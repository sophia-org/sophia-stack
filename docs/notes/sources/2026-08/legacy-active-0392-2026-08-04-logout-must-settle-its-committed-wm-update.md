---
id: legacy-active-0392
date: 2026-08-04
recorded_date: 2026-08-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "policy", "tooling"]
---
# 2026-08-04: Logout must settle its committed WM update

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11907–11930. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The pointer-motion rerun validated the legacy cursor repair even though a
  manual Logout preempted the bounded benchmark completion. After physical
  motion was observed and routed, output 1 accepted 1,116 page flips over
  18.600 seconds: 59.945 FPS with an 18.670 ms worst observed interval. The
  prior standalone atomic cursor path had fallen to 41.390 FPS with a 66.718
  ms p95 interval. Native suspension then drained the final scanout and TTY
  recovery restored termios and KD state normally.
- Super-Shift-Q arrived just before the workload's automatic 20-second exit.
  The same committed WM response carried the Logout session action and an
  ordinary `WmTransactionUpdate`. The owner executed Logout and its existing
  exit gate observed empty input-delivery, key-release, and X-control queues,
  but it did not inspect `pending_wm_update`. It therefore left the loop before
  the synthetic coordinator batch could deliver transaction 2 to Engine. The
  final clean-work assertion correctly rejected the remaining update.
- The session shutdown policy now models Running, Draining, and Complete from
  passive queue facts. A requested Logout remains Draining while input,
  key-release, X-control, or WM-update work exists. This gives the existing
  authority/runtime path one bounded owner cycle to consume the committed WM
  update; it neither discards the update nor weakens the final zero-debt
  assertion. Crate-boundary tests reproduce the exact lone-WM-update state and
  retain every pre-existing delivery barrier.

<!-- END IMPORTED BODY -->
