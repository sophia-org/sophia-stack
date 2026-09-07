---
id: legacy-active-0588
date: 2026-09-02
recorded_date: 2026-09-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-09-02: comparison acquisition becomes terminal-free and visibility-bound

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18605–18646. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The replacement acquisition is implemented without changing the Sophia WM or
shell protocols. Terminal registration and terminal startup are now distinct
launcher operations. The comparison profile registers the approved terminal
action but starts Hagia with an empty application tree; the ordinary session
profiles retain their existing startup terminal.

The capture owner refuses to run when its own process descends from the
attested supervisor, and similarly refuses workload launch roots that entered
that tree. Before launching the workload it records an empty application
baseline. After launch it binds each root to PID plus `/proc` start ticks and
uses stack-native passive observation: X11 root-tree geometry, input focus, and
`_NET_WM_PID` for Sophia/XLibre, or niri's typed workspaces/windows IPC.
Correlation consumes identity only inside trusted conformance code. The
persisted `visibility.log` contains counts and DP-1/focus booleans, never
titles, classes, PIDs, or application metadata that would violate the blind-WM
boundary.

Settlement and every one-second sample require a workload-owned focused
toplevel visible on DP-1 and zero foreign application toplevels. Replay
independently checks the empty baseline, settled record, contiguous cadence,
sample count equality with the resource series, and the same visibility
invariants. Missing focus and foreign-window mutations fail closed. Raw
attempts now contain six checksummed inputs and reduce to schema-3 samples.
Schema-3 preparation records `acquisition=terminal_free_visible`; the
preserved `cp14` schema-2 run is therefore rejected with an explicit legacy
contract error rather than silently continued.
Preparation also binds SHA-256 identities for the Sophia, Hagia, Hagia shell,
XLibre, xmonad, and niri executables. Admission rechecks the live supervisor
and required policy/shell descendants against those identities.

`cargo xtask conformance desktop-comparison gate RUN` is the single-row TTY3
entry point, with `just desktop-comparison-row RUN` as its human-facing alias.
Rust still owns schedule choice, admission, capture, replay, and binding. The
narrow shell adapter owns local VT checks, the three stack launch mechanisms,
and failure teardown; it contains no SSH path. DP-1's active tracepoint CRTC
index is resolved through the Rust DRM API instead of `/tmp/crtc`. The new path
has passed offline compile, replay, mutation, launcher, and shell-syntax checks.
It has not yet passed a physical row, so CP-14.2 remains open for a clean signed
candidate, fresh preparation, all 39 rows, and final verification.

<!-- END IMPORTED BODY -->
