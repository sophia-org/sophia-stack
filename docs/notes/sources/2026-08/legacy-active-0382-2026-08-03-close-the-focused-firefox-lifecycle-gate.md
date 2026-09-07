---
id: legacy-active-0382
date: 2026-08-03
recorded_date: 2026-08-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "policy", "validation"]
---
# 2026-08-03: close the focused Firefox lifecycle gate

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11621–11634. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The one-purpose physical slice launched two independent Kitty processes and
  two Firefox processes. Six ordered Kitty checkpoints bracketed the first
  Firefox's normal `Ctrl+Q` exit, the second launch, and its WM-forced close.
- Both Firefox processes reported managed status-zero exits. The forced path
  included the committed `CloseFocused` action before process retirement, so
  it did not pass through a page or harness shortcut.
- The strict verifier passed with zero protocol errors, pending WM/actions/input,
  recovery extents, or constraint relayout. Application groups and frontend
  workers drained, and the namespace and Xauthority were revoked. All focused
  Firefox gates are now closed; only the integrated promotion run remains for
  Milestone 10.

<!-- END IMPORTED BODY -->
