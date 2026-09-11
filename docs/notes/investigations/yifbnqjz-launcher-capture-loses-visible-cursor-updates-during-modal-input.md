---
id: yifbnqjz
date: 2026-09-10
kind: investigation
status: awaiting-physical-acceptance
tags: [investigation, input, launcher]
---
# Launcher capture loses visible cursor updates during modal input

## Physical observation

During t002 acceptance on 2026-09-10, Super+Space opened the app picker and
Escape dismissed it. The operator reported that typing and arrow keys did not
respond. Tapping and releasing Super, Ctrl and Alt did not restore input.
Mouse-wheel navigation worked, but mouse movement and selection did not.

Installed Sophia is `171345049bf620a40b24c48d738329b1f63decaf`, Hagia is
`3459a85d5dd7a1943efcf526fa5d4ec297d246cd`. Session identity and executable
hashes are retained in the [concurrent reload investigation](v4geoq2j-policy-reload-compares-independent-configuration-generations.md).
Artifacts are in `.artifacts/t076-desktop-acceptance/`.

## Cursor finding

In `route_input_events_with_launcher`, the launcher capture advanced the stored
pointer position, then consumed motion before incrementing `pointer_events`.
The owner schedules visible cursor updates only when the report contains an
observed motion. The stored hit-test position could therefore move while the
visible cursor remained at its old location. The reference-sheet modal path
had the same accounting omission.

The repair counts observed motion, buttons and axes before modal capture and
retains output-boundary placement observations on captured motion. Delivery to
an underlying application remains suppressed. The external routing regression
passes a motion followed by a press/release over a presented target; it checks
position, activation, the counts used by the cursor scheduler and zero client
deliveries. Before the repair the count was zero instead of three; after it the
test passes. This confirms a cursor defect; it does not prove every failed
operator click had this cause.

## Keyboard finding and uncertainty

The working wheel path emits the same navigation operation as Down. Unlike
Down and text, it does not consult the launcher's command-modifier state.
Claude's independent source review identified that gate and the VT handoff as
a concrete divergence between keyboard views.

The VT switch path synthesized Ctrl/Alt releases for the shortcut router and
application keyboard mapper but omitted `LauncherKeyboard`. Physical releases
can happen on the destination VT, leaving the launcher's XKB state holding
modifiers. A local libxkbcommon probe also showed that a second down followed by
one up leaves a modifier active; a later press/release pair does not remove that
extra down. Thus the failed tap workaround does not rule out a missed release.

An external regression drives Ctrl+Alt+F2 through the production router without
actually switching VTs. The router requests VT 2, but before the repair the
launcher still reports a command modifier active. The fix delivers the same
synthetic modifier releases to the launcher keyboard before leaving. The test covers both an open and a closed picker and
then verifies no command modifier remains, a later Ctrl/Alt press/release stays
clear, and text composition returns `a`.

Both regressions pass after their repairs. The operator has been asked whether
this session included a VT switch; no answer is recorded yet. The source defect
is reproducible, but its responsibility for the observed typing/arrow failure
remains unconfirmed. No live keyboard state was inspected or reset.

## Validation and remaining gate

`launcher-regression-before.log` and `launcher-vt-before.log` retain the two
failing regressions; `launcher-focused-after.log` contains both passing tests.
An initial broad direct test invocation had unrelated socket-sandbox failures
and host-configuration contamination; it is not accepted as validation. The
standard full gate isolates configuration and grants the required local socket
access. `check-launcher.log` passed for the cursor fix; `check-final.log` is the
full-gate run after the VT repair. It reached an unrelated X11 shared-client
ordering failure: another client's pair arrived before the first client's pair.
The targeted rerun passed (`wire-order-rerun.log`); final gate retry is retained
in `check-final-retry.log`. The retry passed the complete gate: 3036 Rust
tests across 260 result groups, 29 intentional ignores, archive checks and
the host buffer-age/GLX/EGL checks. Independent review found no blocking defect in the
repairs. Multiple-keyboard modifier reconciliation remains outside this narrow
VT fix, as in the adjacent existing keyboard views.

No repaired owner is installed or running yet. Acceptance requires a new owner
and normal picker search, navigation, cursor movement, click/keyboard activation,
Escape and the terminal-entry cases. t002 remains tracked in
[todo.md](../../../todo.md) through the
[development-session plan](../plans/queue-02-cp-14-3-development-session-readiness-and-milestone-14-c.md#t002).


After applying the test-profile cleanup, the operator confirmed Super+Enter and
Super+B work while picker pointer and keyboard input still fail. The running
owner has not been replaced, so this is another observation of the original
candidate, not a failed acceptance test of the source repairs.
