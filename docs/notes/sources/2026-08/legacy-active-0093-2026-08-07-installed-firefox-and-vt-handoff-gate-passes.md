---
id: legacy-active-0093
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation", "tooling"]
---
# 2026-08-07: Installed Firefox and VT-handoff gate passes

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3082–3110. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed release `0.1.0-4c3121421f12` completed automatic Firefox attempt
`firefox-runs/0002` with a schema-4 `record_kind=firefox` pass and exact runtime
identity for both connected outputs. The physical VT cycle captured one
retained renderer image, drained scanout without abandonment, restored that
image after seat reacquisition, and retired a new primary-output frame. The
former `WorkerPending` handoff failure did not recur.

The same run retained two independently interactive Kitty processes across
exactly two Firefox launches. Firefox completed the loaded, keyboard, scroll,
layout, refocus, and dialog stages once each. Its first process exited normally
through Ctrl+Q; its second exited through the WM close action. One initial
Firefox admission timed out, performed the permitted single WM restart,
reseeded committed layout before replaying pending admission, and converged
without standing-target or geometry debt. Normal logout then completed with
`protocol_errors=0`, `unexpected=0`, no pending input, actions, or WM work,
clean renderer and frontend ownership, one-percent runtime-tmpfs use, and no
retained proof profile. The installed aggregate verifier passed the immutable
archive and release identity.

The physical Firefox verifier now makes the VT evidence normative for this
gate. It requires ordered queue, prepare, renderer capture, drained quiescence,
seat release/acquire, equal-count renderer restore, active resume, and a
post-resume primary retirement; it rejects forced detach. Fixture mutations
remove capture and falsify the restore count, complementing the worker-level
race regression. A future browser pass therefore cannot hide a broken VT
renderer handoff.

<!-- END IMPORTED BODY -->
