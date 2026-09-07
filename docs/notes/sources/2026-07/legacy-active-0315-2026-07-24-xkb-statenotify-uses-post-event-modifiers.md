---
id: legacy-active-0315
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-24: XKB StateNotify Uses Post-Event Modifiers

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9736–9896. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The installed Kitty baseline exposed two keyboard gaps. First, Helix did not
recognize `:`. A strengthened real-Kitty smoke reproduced it: Kitty's keyboard
trace saw Shift press with no effective modifier, semicolon press as `;`, then
Shift only on semicolon release. Core key events correctly carry pre-event
modifier state, but Sophia incorrectly reused that state in XKB `StateNotify`,
whose effective modifiers describe the post-event state.

Routed key records now carry both values explicitly. Core and XI events retain
pre-event state while XKB notifications use the post-event state produced by
the per-seat xkbcommon machine. The real Kitty gate now types exact `:ll` and
requires shell receipt plus later Presents. A complete pc105 US symbol-table
regression covers printable base/shift pairs and F1 through F12.

Second, kernel VT shortcuts cannot operate while the graphical owner has set
the console keyboard to `K_OFF`; leaving translated console input enabled would
reintroduce typed bytes on the hidden TTY. The protocol-neutral session input
owner now recognizes Ctrl-Alt-F1 through Ctrl-Alt-F12, consumes the function-key
edges, and asks the controlling VT to activate the selected terminal. This is a
session-control action, not an application or X11 shortcut.

The first physical switch attempt exposed a launcher boundary: the graphical
owner is deliberately started with `setsid`, so `/dev/tty` is unavailable
inside it. The helper failed with `ENXIO`; treating that failed control action
as fatal correctly returned to greetd. The wrapper now passes the exact
originating `/dev/ttyN` as `SOPHIA_SESSION_TTY`, and the detached owner opens
that explicit device for VT activation. A launcher regression requires the
device handoff to precede `setsid`.

The next physical run refined that diagnosis: the path was correct, but
reopening `/dev/tty7` after detachment failed with `EACCES`. Device paths are
not durable capabilities across display-manager ownership transitions. The
launcher now duplicates its already-authorized controlling-TTY descriptor
before takeover and passes the descriptor number as
`SOPHIA_SESSION_TTY_FD`. Session-control helpers issue VT ioctls directly on
that inherited descriptor; the path remains only a compatibility fallback.
This keeps VT control in the session-control boundary without adding
terminal- or application-specific behavior to Engine.

The next physical run disproved descriptor inheritance as the final solution.
`VT_ACTIVATE` returned `EPERM`: Linux authorizes that ioctl by controlling-TTY
ownership, which the deliberate `setsid` boundary removes, rather than by the
mere possession of an open descriptor. Sophia then exited with status 1, so
the greetd screen observed after Ctrl-Alt-F3 was greetd reclaiming tty7; the
existing tty3 login had never become active.

VT switching now belongs to a libseat controller. Switch rejection is
nonfatal. A successful switch produces an explicit release boundary that
stops input, drains and releases native scanout, and acknowledges suspension;
acquisition rebuilds both hardware domains and repaints the retained scene.
Kitty, X11 clients, focus, and Engine state remain above that hardware
lifecycle, and Engine gains no application-specific branch.

The first physical return from tty3 exposed an incomplete authority boundary.
libseat delivered disable and enable correctly, but Sophia reopened
`/dev/dri/card*` with ordinary filesystem access after enable. AMDGPU rejected
the initialization with `EACCES`, and the session exited cleanly to greetd.
Login-session ACLs are not the device authority once libseat owns the session.

The live backend now runs the non-`Send` libseat handle on a dedicated broker
thread. KMS card and udev-libinput opens request libseat device leases; backend
objects receive duplicated descriptors while the broker retains and closes
the lease token. Suspension drops input and KMS resources before acknowledging
disable, and acquisition obtains fresh leases before rebuilding them. Direct
device opens remain available only to standalone validation paths that do not
participate in the managed live-session lifecycle.

Physical validation of the installed Kitty baseline then completed the missing
proof. Ctrl-Alt-F3 released the graphical seat and exposed the already-active
text login; the Sophia session remained alive. Repeated Ctrl-Alt-F7 returns
reacquired KMS and input, repainted Kitty, and preserved interactive keyboard
and pointer operation. Switching away again continued to work. This promotes
the libseat-backed Kitty session to the known-good installed baseline while
leaving the full F1-through-F12 matrix and xmonad workflow as separate open
proofs.

The first xmonad VT attempt exposed an ordering race hidden by the quieter
Kitty workload. The xmonad profile had a primary-plane frame in flight when
Sophia called `libseat_switch_session`. Seat authority moved before the owner
drained that frame, so its page-flip callback could no longer arrive. Waiting
500 milliseconds after revocation then failed with `persistent native scanout
remained in flight during teardown` and incorrectly ended the whole session.

Operator-requested VT changes now use a prepare-before-release boundary shared
by every profile. The owner stops input, prevents further native submission,
retires and releases KMS work while the seat is still active, drops the old
leases, and only then requests the switch. A request rejection or missing
disable event rebuilds hardware and repaints instead of ending the session.
An unsolicited disable cannot be drained after authority is gone; that path
immediately detaches native state, reports any abandoned scanout, completes an
already-submitted Present as `Skip`, and preserves queued work for acquisition.

Physical xmonad switching then proved KMS survival but exposed a separate input
state boundary. Ctrl and Alt presses had already reached the focused X client
before the function-key press identified the sequence as a VT chord. Their
physical releases occurred on the text VT, outside Sophia's libinput ownership,
so XKB and the WM shortcut router retained both modifiers after acquisition.
The reopened libinput poller was healthy; application input was interpreted
with stale Ctrl-Alt state.

VT activation now emits synthetic releases for every pressed chord modifier,
clears the WM seat state, and waits up to 500 milliseconds for X Authority to
acknowledge those deliveries before KMS quiesce and `switch_session`. Failure
to flush rejects the switch without ending the graphical session. Suspension
still clears local keyboard state as a second boundary, but no longer relies
on that local reset to repair client-visible XKB state.

The next physical xmonad capture isolated a distinct multi-output retirement
failure. Output 2 submitted its third startup frame but never produced another
page-flip callback; output 1 continued submitting and retiring normally. Both
VT attempts therefore reached the prepare boundary with one permanently
in-flight output and timed out before `switch_session`. Emergency shutdown hit
the same strict drain and returned status 1.

Native suspension now has one data-oriented result for both authority states:
whether all callbacks drained, how many scanouts were abandoned, and which
submitted Present was settled as a Skip. While authority remains active the
owner still attempts exact retirement first. A bounded timeout transitions to
the same detached runtime representation used after unsolicited revocation,
then drops the native leases before requesting the VT switch. Final teardown
uses this operation as well. This keeps the missing kernel callback observable
without allowing it to wedge VT control or emergency recovery, and avoids
duplicating detach/Present-settlement policy across lifecycle callers.

The following physical run proved that boundary: an owner-requested VT switch
timed out with one abandoned scanout, detached before release, resumed both
hardware domains, preserved the action-launched Kitty, and completed final
logout with a fully drained scanout. The remaining status-1 exit was unrelated:
two RANDR `GetOutputProperty` requests used atom `None` and correctly received
`BadAtom`, but the session counted those optional client probes as unexpected.

Protocol reduction now recognizes only the complete probe tuple (`BadAtom`,
RANDR `GetOutputProperty`, atom `None`) as expected. The client-visible reply
does not change, and an unknown nonzero atom remains unexpected. The physical
verifier also now distinguishes Sophia's structured failures from harmless
Kitty/GLFW stderr and uses xmonad's actual action identity `768` for
Super-Enter; action `1` remains focus-next. Mutation fixtures preserve each
distinction so the acceptance gate cannot silently regress to broad error
whitelisting or the former action-ID alias.

The next physical start exposed a false-positive readiness gate rather than an
application-launch failure. Kitty mapped, xmonad committed layout and focus,
and output 1 repeatedly retired mixed Present transactions. Output 2 submitted
its startup frame but never delivered a callback. Despite that partial KMS
state, the owner reported `status=ready`: the CPU fallback examined only the
first output, while the DMA-BUF path treated transaction retirement as visual
proof without inspecting the composed pixels. Completion later reported zero
CPU detail, no output-1 nonzero export, and no output-2 retirement.

Startup readiness now reduces flat evidence for every owned output and requires
at least one callback from each. Mixed composition captures a bounded one-time
GPU readback, carries its nonzero RGB count with the exact submitted
transaction, and becomes stable only after that content retires. A missing
output callback after 750 milliseconds, or retired mixed frames without visible
pixels after 1500 milliseconds, triggers one shared native detach/reopen and
repaint through the existing libseat authority. The eight-second deadline
remains authoritative; a second failure exits through guarded cleanup instead
of accepting a cursor-only desktop. Engine remains protocol- and
application-neutral.

<!-- END IMPORTED BODY -->
