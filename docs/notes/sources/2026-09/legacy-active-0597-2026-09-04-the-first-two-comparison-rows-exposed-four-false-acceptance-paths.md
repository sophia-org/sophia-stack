---
id: legacy-active-0597
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-09-04: the first two comparison rows exposed four false-acceptance paths

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18882–18936. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`cp14-schema4-tools` reached 2/36, but neither row is promotable. During the
first Sophia Kitty row, the operator's mouse generated physical motion and the
session routed it to the Engine, yet the cursor did not visibly move. The
atomic cursor owner stored a desired position while a primary flip was busy and
reported the logical target as visible, but nothing guaranteed a cursor-only
commit after that flip retired if no new primary frame arrived. A pending cell
was therefore mistaken for presentation evidence.

The second row kept XLibre and Kitty alive but lost XMonad. Its startup log
showed XMonad attempting to replace itself through `xmonad-x86_64-linux` in the
isolated cache, where no compiled alias existed. Capture had verified XMonad
once, then sampled only the Xorg supervisor and workload, so the dead policy
process did not invalidate the row. The same row recorded roughly twice
Sophia's DRM event population and near-zero intervals. The trace normalizer had
discarded the tracepoint's kernel sequence and counted duplicate deliveries as
independent frames. Finally, capture wrote `teardown=clean` and sealed the row
before the stack teardown or TTY handoff ran; the nested adapter also truncated
the outer gate log when it re-entered for an XLibre or niri child.

The cursor path now has one topology-wide transaction policy. Every active head
must expose accepted resources, a selected plane, and positioning properties
before atomic mode is selected. Motion replaces one desired cell per head and
returns `Queued`, not `Visible`; a primary commit may carry that state, while an
idle head services it immediately after retirement with a blocking cursor-only
commit. `WouldBlock` retains the newest state, a combined primary rejection
leaves it for cursor-only retry, and a hard cursor-only rejection moves the
whole topology to the proven legacy ioctl. Completion records expose queue,
coalescing, ride, cursor-only, combined-drop, fallback, pending, and maximum
queue-delay populations. High-volume per-frame composition, damage, queue, and
page-flip records moved to trace level so diagnostic logging no longer
dominates an ordinary comparison window.

Comparison acquisition now separates measurement from evidence admission.
`capture` stages the six raw inputs while the stack is live. The TTY adapter
waits for the bounded Sophia session or reference stack to exit and restore the
terminal; only then does `finalize` verify that the exact attested PID/start
identity is gone, add clean-teardown evidence, replay, and seal the row. During
measurement, supervisor, Hagia WM/shell, and XMonad identities remain pinned by
PID, start time, and executable on every sample. Resource records split stack,
workload, and aggregate populations without persisting PIDs. Kernel records
retain their sequence and delivery count, collapse repeated delivery of one
sequence, reject noncontiguous recurrence, and refuse a cross-card active CRTC
index that the tracepoint cannot disambiguate. The XMonad comparison launches
through its expected isolated compiled-cache alias, and internal adapters
append to rather than truncate the gate log.

Before the first Sophia measurement, a candidate-derived four-target X11 window
now requires actual pointer motion and a click in each changing target. This is
an operator-visible qualification of the cursor path, not a performance sample,
so it runs outside the measured window and is bound into the first attempt. The
old two rows remain diagnostic inputs; a fresh signed run is required to test
these corrections on hardware.

<!-- END IMPORTED BODY -->
