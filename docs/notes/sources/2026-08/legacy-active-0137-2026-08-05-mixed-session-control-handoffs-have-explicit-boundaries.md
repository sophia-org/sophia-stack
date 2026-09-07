---
id: legacy-active-0137
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-08-05: mixed-session control handoffs have explicit boundaries

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4398–4432. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The complete M8 workload exposed four races after the earlier presentation and
focus repairs. An X request could observe no pending control, lose the runtime
mutex, and then overtake the newly queued control. A control's acknowledgement
deadline began while it was still behind a prerequisite or key release. The
session's 500 ms WM transport deadline contradicted the xmonad bridge's bounded
three-second reply collection. Finally, fixed host sleeps and per-frame serial
tracing let fault injection and input run against stale guest state.

Request dispatch now rechecks control priority while holding the runtime lock.
Control admission records when an item first becomes dispatchable and gives a
dispatched item an independent acknowledgement deadline. The outer WM response
budget is four seconds, still below the ten-second transaction and twelve-second
admission budgets. The QEMU proof anchors its fault after startup clients exist,
uses action, projection, layout, focus, and clipboard-owner barriers, and pins
the final modal click to a deterministic DOM anchor. Reduced M8 logging keeps
the serial channel causal without changing verifier evidence.

Modern GTK also required a real session bus before it would connect to X. The
guest image now packages the D-Bus runner, daemon, and session configuration,
and Sophia plus every child run inside one session-scoped bus. The minimal GTK
scenario explicitly disables its out-of-scope accessibility-bus lookup. The M8
Zenity launcher now opens, is admitted, closes normally, and no longer strands
the postlude.

Deterministic regressions cover request/control lock arbitration, dispatch
eligibility, independent queue and acknowledgement deadlines, deadline
ordering, and every new verifier barrier. The rebuilt full M8 gate passed all
eight Firefox stages, launcher admission and exit, one expected bridge restart,
zero timed-out or unexpected controls, and clean health and teardown. The
older isolated GTK `--entry` scenario still times out before X connection with
the host's GTK 4 Zenity, while the M8 `--info` launcher passes; that separate
harness compatibility issue is not evidence for the mixed-session milestone.

<!-- END IMPORTED BODY -->
