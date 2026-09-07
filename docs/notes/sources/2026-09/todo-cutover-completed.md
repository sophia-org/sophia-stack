---
id: todo-cutover-completed
date: 2026-09-06
date_basis: cutover-snapshot
kind: source
status: historical
tags: [historical, milestone]
---
# Checked tasks at the todo.txt cutover

These 19 rows were already checked in the old roadmap. The date records this
snapshot, not their completion dates. They remain historical evidence; future
completions go to done.md with the date recorded by the todo.txt CLI.

<a href="../../../history/todo-cutover-2026-09-06.txt">Original roadmap snapshot</a>.

## legacy-done-001

Original section: CP-14.3 — Development-session readiness and Milestone 14 closure (`NOW`).

Implement the native application launcher through the generic revision-4
shell protocol: session-owned catalog and execution policy, Engine-owned
input/GPU presentation, and independent Narthex search. Hagia exposes only
session operation slot 7. See [application launcher](../../../application-launcher.md).

## legacy-done-002

Original section: CP-14.3 — Development-session readiness and Milestone 14 closure (`NOW`).

Accept Kitty startup and Super+Enter on a replacement release. On
`b25b29c1`, startup Kitty exited on RENDER CreateCursor before GLX, then the
eight-second startup guard ended the session while a shortcut launch waited.
Pictures now retain freed pixmap backing through their own lifetimes; Kitty
and the RENDER lifetime probe pass headlessly. Installed `f323323d` reached
the desktop on 2026-09-06 and admitted two Super+Enter terminals. The startup
guard and WM policy are unchanged.

## legacy-done-003

Original section: CP-14.3 — Development-session readiness and Milestone 14 closure (`NOW`).

Separate normal desktop lifetime from application startup proofs. Launch
admission no longer waits for an existing focused application; startup apps
need not produce frames before later apps launch, and normal completion does
not demand a proof timestamp. CPU drain accounting and native presentation
eligibility remain active independently of proofs.

## legacy-done-004

Original section: 1. Recover reliably.

Preserve session-wide native evidence across scanout replacement,
retaining earlier failures and each owner's drain obligations.

## legacy-done-005

Original section: 1. Recover reliably.

Honor runtime deadlines while the seat is suspended, without requiring
reacquisition to begin shutdown.

## legacy-done-006

Original section: 1. Recover reliably.

Add regressions for retirement before replacement, repeated resume,
immediate shutdown after resume, rejected VT switches, topology replacement,
suspension across the deadline, and failure retention. Lifecycle model and
negative controls pass; `cargo xtask check` passes (2,348 test executions).

## legacy-done-007

Original section: 1. Recover reliably.

Complete the physical suspended-deadline canary. The fixed candidate's
retry retained 3,425 native retirements, shut down while suspended at its
90-second deadline, drained in 539 ms with zero pending work, and restored
fallback input with exit status 0. The operator reports it seems to work.

## legacy-done-008

Original section: 1. Recover reliably.

Complete the [physical VT-return canary](../../../native-recovery-canary.md).
Two returns opened epochs 2 and 3 with 229 and 19 subsequent retirements;
session totals retained all 275 retirements across three settled owners.
Browser launch and logout were accepted after resume. Normal logout drained
in 46 ms with zero pending work, native failures, or emergency recovery;
fallback input was restored and exit status was 0. The operator reports done.
The Firefox deduplicated-GPU routing regression is fixed; the isolated full
gate passes 2,350 test executions. Both recovery canaries now pass.

## legacy-done-009

Original section: 2. Establish the live session.

Allow local session installation from signed local Hagia commits without
requiring a push or `origin/master` equality. Retain source signature,
committed-profile, manifest identity and artifact verification checks.

## legacy-done-010

Original section: 2. Establish the live session.

Add the [opt-in Quickshell X11 panel](../../../quickshell-x11-panel.md) and
`cargo xtask panel` launcher with explicit GPU/software selection and retained
binary identity. Isolated normal-session CPU content, popup update/withdrawal,
work-area release/reacquisition and clean normal-exit teardown pass.
Runtime profile activation and stale resize/work-area startup races repaired.
`cargo xtask check` passes (2,430 Rust test executions).

## legacy-done-011

Original section: 2. Establish the live session.

Honor the desktop startup list in the ordinary Hagia launcher, retaining
explicit CLI overrides and a terminal fallback. Register the opt-in panel in
the trusted core application registry; keep process launch out of Hagia.

## legacy-done-012

Original section: 2. Establish the live session.

Confirm the panel popout visibility repair in normal GPU use. On installed
Sophia `05ef0eb8` / Hagia `12f7493`, the operator confirms the incrementer
works; the live renderer samples the 240x112 popup on output 1. Evidence:
`/tmp/sophia-panel-camera-confirmed-05ef0eb8`. Both-output and lifecycle
acceptance remain in the broader panel row above.

## legacy-done-013

Original section: 2. Establish the live session.

Confirm that opening new windows in the scrolling layout moves the camera
to them. The operator confirms this on Sophia `05ef0eb8` / Hagia `12f7493`;
logs show three added terminals receiving focus with committed layout moves.
Second-output placement, vertical scrolling and close behavior retain their
separate acceptance scope.

## legacy-done-014

Original section: 2. Establish the live session.

Delegate WM policy-setting validation to Hagia. Sophia preserves bounded
ordered policy fragments; Hagia validates values before activation. The
default and personal Hagia profiles pass paired offline checks without edits.
TTY startup checks both owners before display-manager takeover. Cross-repository
regression coverage includes repeated workspace records and rejection before
acknowledgement; runtime/installed acceptance still follows below.

## legacy-done-015

Original section: Parallel Production Readiness.

Shell reference preparation: generic boundary documented, Quickshell fork
and `sophia` branch established, Void baseline built, and
[panel/popout requirements and results retained](../../../shell-reference-client-audit.md).
Eight of nine test suites passed; the existing popup-movement failure is
downstream baseline debt. This completes preparation, not content support.

## legacy-done-016

Original section: Parallel Production Readiness.

Document descriptor and content shell models and the proposed
[content-shell behavioral contract](../../../content-shell.md): explicit operator
admission, panel/popout lifecycle, input and visual trust boundaries, and
independent-client acceptance. No content wire, runtime capability, or
configuration is implemented by this documentation milestone.

## legacy-done-017

Original section: Parallel Production Readiness.

Implement `RENDER`. Both Quickshell and Brave ask for it and are refused:
`sophia_x11_authority_extension schema=1 status=absent name="RENDER"`. It is
the antialiased-text and image-compositing path every toolkit reaches for
first, so a client refused it falls back to core drawing and looks wrong or
feels slow rather than failing outright. The largest of these by a wide
margin, and the one worth doing properly. Done at version 0.4, which is
exactly what is implemented: the advertisement was withheld through three
commits until the requests behind it answered, because the base protocol has
no version gate. The Quickshell trace now reaches opcode 144. Version 0.5
adds ARGB cursors, which are stored but not yet displayed -- putting a
client's cursor on screen needs authority-to-engine plumbing that does not
exist. See the `RENDER compositing and glyphs` matrix row.

## legacy-done-018

Original section: Parallel Production Readiness.

Implement `XC-MISC`, before something needs it. It is how Xlib recycles
XIDs once a client exhausts its range, and the client that hits it is a
browser left open for days. Nothing has asked yet, which is exactly why this
is cheap now and an incident later: the failure mode is the client dying
rather than degrading. Done: grants draw from the same counter that connection
setup draws from, so a grant cannot collide with a range a client already
holds, and both refuse rather than overlap once the pool is spent. See the
`XC-MISC identifier ranges` matrix row.

## legacy-done-019

Original section: Native WM and shell product.

Implement the bounded session-owned [control v1](../../../sophia-control-v1.md)
endpoint and `sophia msg`: startup-only `session.control "host-admin"`, disabled
by default, socket-derived pidfd and user/mount/PID namespace admission,
bounded worker, exact catalog/action correlation, Engine-settled policy
actions, and asynchronous WM restart confirmed by replacement commit.
Generic WM/shell authority boundaries and their existing wires are preserved.
