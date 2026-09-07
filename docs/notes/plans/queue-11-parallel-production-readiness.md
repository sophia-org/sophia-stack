---
id: queue-11
date: 2026-09-06
kind: plan
tags: [plan, milestone]
---
# Parallel Production Readiness

This plan retains the scope, constraints, and task details from the roadmap
cutover. Task status and order live only in [todo.md](../../../todo.md)
and the [monthly completion history](../../../done.md). Follow the
[work-tracking contract](../../work-tracking.md).
Historical candidate identities in the details require revalidation before use.


These rows do not reorder the critical path.


Previously completed evidence: [Shell reference preparation: generic boundary documented, Quickshell fork and sophia branch established, Void baseline built, and panel/popout requirements and results retained.](../sources/2026-09/todo-cutover-completed.md#legacy-done-015).


Previously completed evidence: [Document descriptor and content shell models and the proposed content-shell behavioral contract: explicit operator admission, panel/popout lifecycle, input and visual trust boundaries, and…](../sources/2026-09/todo-cutover-completed.md#legacy-done-016).


## t024

Repair the evidence readers still pinned below their emitter. Ten accept
`sophia_live_session status=bounded_complete` at schema 15 or lower against an
emitter that writes 16, and nine accept `sophia_live_wm status=ready` at
schema 1 against an emitter that writes 4. These are retired-policy physical and
QEMU gates; they fail loudly rather than silently, so each needs a per-gate
decision about whether it still earns its keep. Add each repaired record to
`tools/check_live_record_schema_readers.sh` once its emitters are confirmed to
agree.


## t025

Decide whether `run_frame_fed_output_gate_tty4.sh` and
`run_current_critical_path_tty4.sh` keep requiring HEAD to equal the locally
known origin/master. The direct-scanout and Hagia native runners no longer do;
`package_live_session.sh` keeps it deliberately, because packaging is the
publishing question the rule was wrong about being.


## t026

Move remaining session-private test modules out of production `src` as
visibility boundaries permit, and split the oversized cohesive units named in
`docs/source-layout-debt.txt`. Do not weaken privacy or add test-only
production APIs.


## t027

Reduce `tools/start_sophia_tty3.sh` to the minimum TTY/display-manager
adapter around `sophia session run`. Typed parsing, verification, archive
handling, and gate orchestration stay in Rust.


## t028

Repair the load-sensitive `sophia-x-authority` `x11_wire` flake. Rewrite
the affected tests together with `read_x_reply`: it currently treats Present
event type 35 as a reply and interprets bytes 4..8 as a body length. Raising
the ten-second timeout is not a fix. Preserve the 178-test baseline while
making record-kind parsing explicit.



Previously completed evidence: [Implement RENDER.](../sources/2026-09/todo-cutover-completed.md#legacy-done-017).


## t029

Implement `SHAPE`. Quickshell asks for it in the same trace. Small: a
handful of requests for non-rectangular window regions, and Sophia already
carries region machinery for XFIXES.


Previously completed evidence: [Implement XC-MISC, before something needs it.](../sources/2026-09/todo-cutover-completed.md#legacy-done-018).


## t030

Decide, rather than implement, `Composite`, `DAMAGE`, `XTEST` and `DPMS`.
Each is a domain Sophia owns -- compositing, input, power -- and a client
reaching through one of them is asking to step around that authority. They
belong in the matrix as deliberate exclusions or as admitted surface, not as
gaps that stayed open because the list looked incomplete.


Every row above came from measurement rather than a survey of what a server
usually has. `QueryExtension` now records what it refuses, so the next live
session extends this list by observation; the four decisions above should be
revisited against a week of real logs rather than against this paragraph.

Completed infrastructure baseline: `sophia-session` owns production lifecycle,
`sophia-conformance` owns development-only evidence logic, `cargo xtask` is the
canonical developer/CI surface, `just` is optional human shorthand, canonical
installed commands live under `sophia session`, and source-layout debt is an
exact identity ledger.
