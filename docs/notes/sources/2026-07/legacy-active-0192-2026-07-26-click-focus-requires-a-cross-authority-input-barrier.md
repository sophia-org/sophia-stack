---
id: legacy-active-0192
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "security"]
---
# 2026-07-26: Click Focus Requires A Cross-Authority Input Barrier

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6491–6580. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Physical pointer events were already hit-tested against Engine scene truth and
routed to opaque surfaces, but a primary press never asked spatial policy to
change focus. Consequently clicking or click-dragging another xmonad tile
could move the cursor and deliver pointer input without changing Engine, WM,
or X11 focus.

WM API v4 adds `FocusRequested`, containing only surface, output, and
workspace. The reference WM returns `FocusSurface`; the metadata-blind legacy
bridge translates the target into a private synthetic primary-button gesture
so xmonad updates its own focus stack before returning the same opaque Sophia
surface. No XID, namespace, application metadata, or raw input payload crosses
the WM boundary.

Engine now owns a bounded pointer-focus handoff. The initial press and following
motion/release records remain ordered against the selected surface, including
drag coordinates outside its geometry, until the X frontend acknowledges that
same focus. A 256-record capacity and two-second timeout fail closed. Protocol,
reference-policy, bridge, route, ordering, and timeout regressions pass. The
real unmodified-xmonad smoke also focused a requested opaque surface. Xmonad's
focus refresh re-emitted unchanged configure requests, so the compatibility
runtime discards those only for a focus-only request; the resulting Sophia
transaction contains `FocusSurface` and no placement. Physical TTY confirmation
remains the promotion evidence.

The local QEMU xmonad gate then exposed a harness-only seat regression before
WM negotiation: its minimal initramfs provides neither logind nor a seatd
daemon, so automatic libseat discovery returned `Function not implemented`.
The guest now explicitly selects libseat's direct `noop` backend. Production
sessions keep ordinary backend discovery and remain libseat-owned.

After cursor recovery, the M7 harness exposed a stale restart assertion. Its
input sequence intentionally left workspace 2 active and empty, but the host
required a new focused-surface record after restarting the compatibility
bridge. Preserving that workspace correctly produces `hidden_focus_cleared`.
The harness now accepts exactly the two valid reduced recovery states: a new
focus reconciliation for a focused projection or a new clear-focus record for
an empty projection.

The next M7 pass reached post-restart launch/close and exposed a separate
supervision classification error. Proof mode treated every secondary child
exit as fatal, including an approved action-launched terminal carrying a
launch transaction. Fixed proof witnesses still must remain alive, but
transaction-correlated action children may now exit normally in both proof and
normal sessions.

Repeated two-window startup exposed a stateful WM queue ordering defect.
Sophia treated geometry changes and earlier committed workspace state as
reasons to discard a later xmonad response. Xmonad had already processed that
request, so the rejection desynchronized the two state machines and a
following action failed with `UnknownSurface`. Response lifetime checks now
track only the opaque surfaces in the request. Each ordered response is reduced
against the latest committed workspace state, with a `ManageSurface` target
added to that current planning projection. Engine transaction validation
continues to own geometry correctness.

The same QEMU gate found that startup readiness was reduced only when an
optional timeout was configured, although bounded completion always required
the readiness result. Readiness reduction is now unconditional; the option
only supplies a failure deadline. Its post-detail frame barrier requires a new
submission only on outputs intersecting the focused surface while retaining
the initial presentation baseline for every owned output. The final M7 run
completed two-head startup in 187 milliseconds, processed 14 WM requests with
13 commits and zero stale responses, recovered one compatibility-bridge
restart, launched and cleanly closed the action terminal, logged out with zero
protocol errors or pending work, and drained both outputs.

The M7 verifier previously required at least two CPU layers in the final frame.
That contradicted the scripted empty workspace at logout. Its three-surface
peak remains independently required by committed-layout evidence; bounded
completion now proves clean external-WM lifecycle without requiring closed or
hidden windows to remain in the shutdown frame.

The unattended M7 workflow now exercises the pointer-focus barrier itself
through the existing virtio mouse. After two xterm surfaces settle, a keyboard
focus action selects the non-master tile, relative motion clamps the pointer to
the unfocused master tile, and QMP emits primary press, drag motion, and
release. The run recorded `FocusRequested` for the other opaque surface,
committed and acknowledged that focus, released all three retained pointer
records only afterward, and routed a following ordinary key to the same
surface. The final ledger contained two routed buttons, four routed pointer
events, zero stale WM responses, zero protocol errors, and no pending input.

The shared physical verifier now requires that following keyboard record as
well as request, Engine commit, X frontend acknowledgment, and retained
press/motion/release order. QEMU proves the complete software and virtual-input
path reproducibly; the real libinput device, physical DRM, and operator
interaction remain the final promotion evidence.

<!-- END IMPORTED BODY -->
