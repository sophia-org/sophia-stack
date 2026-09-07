---
id: queue-14
date: 2026-09-06
kind: plan
tags: [plan, milestone]
---
# Native WM and shell product

This plan retains the scope, constraints, and task details from the roadmap
cutover. Task status and order live only in [todo.md](../../../todo.md)
and the [monthly completion history](../../../done.md). Follow the
[work-tracking contract](../../work-tracking.md).
Historical candidate identities in the details require revalidation before use.

[Parent scope](queue-12-candidate-queue.md).



## t035

Before admitting a content prototype, characterize Quickshell's retained
popup-movement failure with an isolated display backend. After the CP-15
coherence gates, separately admit one panel/popout workflow and require an
independent C content client against the same public shell contract. The
[audit](../../shell-reference-client-audit.md) owns feasibility evidence and the
[proposal](../../content-shell.md) collects the behavioral requirements. Transport,
pixel semantics, numeric bounds, wire design, modeling, and a conformance
corpus remain prerequisites; documenting them admits no runtime implementation.


Previously completed evidence: [Implement the bounded session-owned control v1 endpoint and sophia msg: startup-only session.control "host-admin", disabled by default, socket-derived pidfd and user/mount/PID namespace admission,…](../sources/2026-09/todo-cutover-completed.md#legacy-done-019).


## t036

Optional short installed control smoke: discover, invoke a safe registered
action, restart WM, and observe continued input/rendering. Automated endpoint
and supervised-owner evidence is separate; do not restart the 36-row gate.


## t037

Repair transactional profile reload before advertising `reload-profile`.
Keep shell commands, delegated grants, parameters, queries, and subscriptions
behind separately specified contracts. Linux admission does not attest
arbitrary third-party sandboxes sharing the host namespaces.


## t038

Harden the implemented issuer-scoped action checks and reservation/work-area
coordination only against a named remaining lifecycle gap; preserve existing
offline conformance and distinguish it from signed physical acceptance.


## t039

Stabilize the minimum `sophia_shell_v1` lifecycle only after CP-15.1 and
CP-15.2; require signed installed Narthex evidence and preserve metadata
separation from the blind WM.


## t040

Add bounded target-resolved move, resize, drag, and scrolling interactions.


## t041

Extend launch-placement or output-scoped workspace policy only for a named
unmet workflow; opaque launch provenance and active-output selection already
exist. Native tab implementation is complete; acceptance belongs to CP-14.3.


## t042

Expose per-head scanout framebuffer handles from `sophia-backend-live` so a
requested output topology can be applied inside a live session.
`compose_from_current_framebuffer` in `desktop_output_heads.rs` reads the
CRTC's current framebuffer, which is correct for a standalone command that
composes nothing and empty inside a session, whose atomic commits leave the
legacy field unset. A head's own submissions are `pub(crate)`, so this is new
public surface. It is followed by an unanswered question: whether an atomic
modeset is safe while the session owns and is flipping those planes.
`apply_requested_native_output_topology` already runs after startup presents
and declines cleanly, so the gap reads in every session log as
`sophia_live_native_topology_apply status=declined reason=heads`.
What it costs on this seat: DP-1 is a DELL S3222DGM whose EDID offers
2560x1440 at 120Hz, the profile requests exactly that, and the session runs
it at 60. DP-2 is a DELL P2319H that tops out at 1920x1080 at 60, so it is
already at its best and only DP-1 is waiting on this.


## t043

Define a bounded redacted workspace/layout/focus status feed. The native
launcher action is implemented above; add lock, screenshot, wallpaper and
audio through their owning shell/session capabilities. The wire is not what blocks these:
`SnapshotSessionOperation` (record kind 4, max 256) already advertises
every operation with its slot each snapshot, and a policy client resolves
slot to operation and sends `SessionOperationRequest`, so no revision is
needed. What is closed is this repository's own vocabulary — the
variants of `DesktopSessionShortcut` in `crates/sophia-config/src/
shortcut_candidate.rs` and `WmActionBehavior` in `.../types.rs` — plus each
behavior's implementation, lock's being a security transition rather than a
launch. A policy client's side is one profile-whitelist string and one
appended action per capability, so each operation defined here unblocks
Hagia in a few lines.
