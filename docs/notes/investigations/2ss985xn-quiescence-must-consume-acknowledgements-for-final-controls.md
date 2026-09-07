---
id: 2ss985xn
date: 2026-09-06
kind: investigation
status: closed
tags: [investigation, session, validation]
---
# Quiescence must consume acknowledgements for final controls

## Evidence and cause

[t010](../plans/queue-04-2-establish-the-live-session.md#t010) retained
`/tmp/sophia-panel-probe-v5`: an isolated software Quickshell panel and xterm,
with Hagia and a ten-second explicit runtime limit. Frontend drain completed,
then the final work-area change committed a layout and dispatched another
control. Quiescence declared completion after 541 ms, but terminal accounting
reported twelve dispatched controls, eleven delivered, and one pending. The
session returned status 1. Its zero protocol-error tally rules out the separate
ordinary-logout policy defect as the cause of this observation.

On signed baseline `ef1ba0e7c5b8360dbb3ac7b94c3da03dbe2e5807`, the quiescence
snapshot checked authority, coordinator, CPU, and native work but omitted
controls. Control service consumes available acknowledgements before dispatch;
an acknowledgement generated during dispatch is consumed on another owner
turn. Layout progress can also enqueue a control after servicing returns.
Neither path was represented in the completion predicate.

A deterministic regression uses the real frontend router with a departed
client. It generates a `ClientGone` acknowledgement during dispatch, leaving
one accepted control pending until the next service call. Before the repair,
quiescence incorrectly returned Complete. Delayed-acknowledgement and
input-barrier cases independently reproduced the same missing condition.
These tests establish the ordering defect without guessing which opcode or
acknowledgement timing produced the historical final row.

## Repair

The small quiescence model now belongs to the public Rust `session_shutdown`
domain. Its snapshot includes pending controls, sampled after control and
layout servicing; zero is required for successful quiescence. Existing stale
client acknowledgements still retire through their exact accounting path.
Rejection, timeout, and unexpected-acknowledgement checks remain fatal, and no
queue or shutdown deadline was extended. Settlement at the deadline retains
precedence over cancellation.

Quiescence schema 3 reports `pending_control_count` on completion and timeout.
Diagnostic reduction retains that count and the record-scoped frontend-drained
and timed-out statuses. Two existing pure-state tests moved out of private
production test contexts into lifecycle integration tests. No X11 extension,
WM policy, or native wire protocol changed. The
[architecture contract](../../architecture.md) and
[operational diagnostic guide](../../operations.md) describe the drain rule.

## Validation

The three new ordering cases failed against the old predicate; all 34 focused
checks pass after correction: ten shutdown, twelve control, and twelve
diagnostic integration tests. They cover each accepted-work domain, immediate
stale acknowledgement, delayed delivery, follow-up layout work, input-release
barriers, missing acknowledgements, non-stale rejection, unexpected replies,
and settlement exactly at the quiescence deadline.

The real ten-second software probe passed in
`/tmp/sophia-t010-panel-deadline-v3`: quiescence completed after 539 ms, all
eleven enqueued controls were dispatched and delivered, and pending, rejected,
timed-out, unexpected, and stale-retired counts were zero. Final cleanup was
clean and process exit was zero. The probe uses private configuration, runtime,
checkpoint, and cache directories, an independent X display, no physical
input, and no native scanout. Identity and exact invocation are in
`identity.json`; the executable and all configuration inputs are hashed.

The first two attempts were invalid fixtures: current Hagia publishes an
application-launcher operation even without a shortcut binding, while the old
fixture supplied no application catalog. A private terminal-only catalog made
that operation available for the third attempt. Production capability checks
were not weakened. Initial full-check failures also exposed temporary-worktree
setup assumptions: group-writable KDL fixtures inherited from the host umask,
and archive tools expecting Hagia in a sibling directory. Removing write
permission for group/other on those copied fixtures and using the supported
`SOPHIA_HAGIA_ROOT` override corrected the test environment.

No physical-session acceptance is claimed or required for this isolated
shutdown regression. Ordinary sessions remain unbounded. The
[unclassified runtime crash](fltuldiq-runtime-session-crash-retains-no-specific-cause.md)
remains separate under t019; this repair does not establish its cause.

## Completion and retained evidence

The implementation is integrated into the main worktree on `ec213d5c`, retaining
its newer X11 commits. `cargo xtask check` passed there, including workspace
tests, Clippy, profile/layout checks, all retained archive checks, verifier
fixtures, and the host buffer-age equivalence proof. Rebuilding xtask after
transfer was necessary because the shared build cache retained its embedded
temporary-worktree path. No verifier or security check was skipped.

The private evidence bundle is
`/home/niltempus/.local/state/sophia/development-evidence/t010-ef1ba0e7-43427737ec94`.
All thirteen entries in `SHA256SUMS` verified. It retains the passing probe's
configuration and identity, failed-fixture logs, the reproduction script,
red/green regression output, full-gate output, and implementation patch.
The probe executable SHA-256 is
`43427737ec940e7c9ca54daf5a217fc452cda50a48c503833696a1db5df2ba69`;
the retained implementation patch SHA-256 is
`1f3a2ac4acb4325105fb536bf75b7bbba4a3abb56512ca9b5557b5844109d32f`.
The patch identifies the tested changes relative to the signed isolated
baseline; subsequent edits only document completion and update task tracking.
These results satisfy t010's diagnosis and isolated shutdown-regression exit.
