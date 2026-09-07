---
id: legacy-active-0175
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy", "architecture"]
---
# 2026-07-26: X Hierarchy Defines the WM Admission Boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5999–6041. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical pre-pixel-admission candidate still exited with
`stage=not_focused`: native scanout and page flips remained healthy, but the WM
never received a manage request. A retained real-xterm authority trace exposed
the exact ordering. xterm issues `MapSubwindows` for descendants of its
toplevel, then issues `MapWindow` for the toplevel itself. Sophia had discarded
the requested parent at `CreateWindow` and implemented `MapSubwindows` as
"map every window in this namespace." That prematurely moved the toplevel out
of its deferred state, so the later real map request could not emit the
presentation intent required by Engine and the blind WM.

The authority window table now owns parent links as passive X protocol state.
`QueryTree` projects that state, reparenting validates cycles, and
`MapSubwindows` affects only direct children. A non-override-redirect root
child is a policy-managed toplevel; descendants and override-redirect windows
are client-positioned. Deferred map policy therefore applies only at the X
root boundary. The retained wire regression reproduces xterm's real opcode
sequence and proves that mapping a child does not admit its parent, while the
following toplevel `MapWindow` emits exactly one managed presentation request.

Core software drawing follows the same tree reduction. Descendant buffers stay
X-authority resources, while their translated damage is accumulated into one
immutable toplevel presentation buffer and one Sophia surface generation.
The concrete presentation extent grows only with observed descendant coverage
and is capped by the toplevel geometry; a configure alone therefore cannot
manufacture a full-size buffer that would satisfy admission. This gives xterm's
shell/content window split one visual identity without leaking X children into
the WM.

The audit also closed a visual-boundary gap. Drawing to an unmapped managed
window is valid X11 traffic, but it is not permission to enter Sophia's
committed scene. The live layout now keeps at most one latest pre-admission
transaction per surface in a bounded data table. It records the safe extent
for planning and recovery, excludes the transaction from renderer intake, and
releases it exactly once—rebased to the first Engine visual generation and the
accepted WM geometry—after frontend admission is acknowledged and matching
pixels are ready. Withdrawal, removal, and terminal timeout erase the retained
record. Early Present submissions follow the same boundary in a fixed
256-record queue; overflow fails the session closed instead of leaking a GPU
submission or growing memory without bound. No client identity or X resource
enters Engine or WM policy.

<!-- END IMPORTED BODY -->
