---
id: legacy-active-0380
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-02: separate routed selection events from flushed delivery

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11580–11599. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first one-Firefox run after preserving the SendEvent bit reached both
  reverse conversion requests. Kitty wrote the Firefox requestor properties
  and Sophia accepted two successful SelectionNotify events into the correct
  routed channel, but Firefox again made no GetProperty request. This proves
  that `notify_routed` was not a sufficient delivery oracle and that the
  standards fix alone did not close the real-client seam.
- Diagnostic protocol writers now emit a redacted record only after the
  recipient Unix socket write and flush succeed. Selection request, notify,
  and clear records include the recipient client, recipient sequence,
  timestamp, resource IDs, atoms, property presence, and synthetic flag, but
  no property payload. The focused verifier requires a flushed synthetic
  property-bearing notify between each conversion and its consumer checkpoint.
- The same run also produced zero title checkpoints even though the dedicated
  page was visible. PRIMARY diagnostic mode now records each observed
  `_NET_WM_NAME` byte length before applying the monotonic checkpoint reducer.
  The next run can therefore distinguish incorrect canary lengths from missing
  metadata without exposing title content or adding another operator step.

<!-- END IMPORTED BODY -->
