---
id: legacy-active-0179
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-07-26: Emergency Shutdown Owns Routed-Key Drain

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6135–6158. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first commit-pinned emergency gate triggered both the live owner and the
independent guard, drained native presentation, restored the TTY, and returned
control, but the inner session failed its final key-ledger invariant. Ctrl and
Alt had already been routed to the focused client before Backspace completed
the emergency chord. The loop waited for existing authority deliveries but
did not synthesize releases for those two routed presses, leaving the client
ledger nonempty at completion.

Emergency shutdown now snapshots the complete bounded pressed-key ledger,
cancels active repeat, routes releases through the normal X authority path,
and adds those delivery IDs to the existing acknowledgement barrier. This is
session-wide input ownership, not Kitty, xmonad, or chord-specific client
policy. Surface-scoped focus and logout flushes share the same release reducer.

The run also exposed an outer-control-plane mismatch. The independent guard
intentionally exits its launcher with status 130 after recovery, while the
promotion driver previously rejected every nonzero launcher status before
running the emergency verifier. Gate policy now admits 130 only for emergency
evidence. The verifier remains authoritative: it requires the inner session
to exit zero with drained key, control, native, and Present state plus exact
TTY restoration.

<!-- END IMPORTED BODY -->
