---
id: legacy-active-0330
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-07-25: Core Selection Ownership Must Be Queryable

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10300–10369. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical cycle covered the normal interaction set in a different
order. Mouse selection worked, but Ctrl-Shift-C and Ctrl-Shift-V did not.
Kitty's GLFW layer repeatedly reported that it failed to become owner of the
clipboard selection. This distinguishes the defect from physical key routing:
Kitty received the copy chord and attempted the X11 ownership transition.

The frontend accepted `SetSelectionOwner` into its namespace-aware selection
table, but core `GetSelectionOwner` unconditionally replied with `None`.
GLFW performs the standard set-then-query ownership check and therefore
correctly treated every copy as failed. Core owner queries now return only the
owner visible in the caller's admitted namespace. Classic shared-X clients see
the shared owner, while a confined client cannot discover an owner in another
namespace. The wire regression covers the visible and confined cases.

Normal-session evidence now reports only owner-change and conversion counts;
clipboard content remains redacted. The strict physical gate requires both
operations, rejects GLFW ownership failures, and retains operator confirmation
that the selected text was pasted unchanged.

The first retest acquired clipboard ownership twice with no GLFW ownership
failure, and the operator confirmed that paste appeared to work. It produced
no `ConvertSelection`, however, because the copy and paste occurred inside one
Kitty process; Kitty can reuse its locally held selection without a protocol
round trip. The promotion sequence now pastes into an independently launched
Kitty before the owner exits, so the conversion witness measures the intended
same-namespace X11 path.

That retest also exposed an independent pointer-boundary defect. After paste,
the hardware cursor disappeared. Input remained healthy and the final record
showed 160 successful cursor-plane updates with zero failures, but only 1,202
of 8,158 observed pointer events routed to a surface. Session pointer placement
had applied the libinput accumulator plus its startup offset without confining
the result to any Engine output. The KMS cursor owner deliberately detaches the
cursor plane when given a point outside every output, so that reachable path
matches the reported disappearance without producing a hardware failure.

Engine now provides one output-union confinement system. The live session
projects its existing ordered output topology into that system before physical
input starts. Confinement chooses the nearest valid point across all output
rectangles, including unequal output heights, and corrects the raw-to-logical
offset at the edge so discarded overshoot does not create a sticky boundary.
Integration coverage drives positions past every side and into the dead area
beside a shorter output. Completion evidence now counts intentional hidden
updates separately from successful updates and failures; the strict gate
requires zero. Physical edge/reversal confirmation remains pending.

The first confinement repair still left its accumulated raw position, startup
offset, corrected edge offset, and current logical position in the CLI owner.
That was the right behavior in the wrong authority: the CLI could effectively
choose Engine cursor state, and the confinement helper alone could not prove
that a real input stream reversed immediately after overshoot.

`OutputUnionPointerState` now owns that complete state machine inside Engine.
The live owner supplies only immutable output rectangles, the optional initial
surface geometry, and each raw backend point. Engine returns a logical
placement plus reduced boundary contact/reversal facts; no backend handle,
device identity, client metadata, or pointer coordinate is logged. Deterministic
coverage proves all four edge directions and unequal-output projection.

The rebuilt two-output `xmonad-m7` guest then drove the real virtio-mouse path
hard against the right edge and sent one 96-unit reverse delta. Engine emitted
an output-edge contact followed by an immediate-reversal observation, after
which the complete click-drag focus, keyboard, workspace, bridge-restart,
launch/close/logout, and native-drain workflow passed with zero protocol,
cursor-plane, stale-WM-response, or cleanup failure. This is unattended
evidence for the state machine; the physical gate still requires every edge of
the actual output union and visible hardware-cursor confirmation.

<!-- END IMPORTED BODY -->
