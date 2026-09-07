---
id: 64o6l37g
date: 2026-09-06
kind: investigation
status: awaiting-physical
tags: [investigation, session, x11]
---
# Normal logout reports failure after X protocol errors

## Question

Why does an operator-requested logout return status 1 after apparently orderly
quiescence and TTY recovery?

## Evidence

The marked installed `4b4f2841` session
`00000001788745936827-954d3556-800f-4929-b3c7-bdb25c873b25` completed its VT
round trip. At logout, events 5409–5427 show quiescence completing in 88 ms;
event 5428 records drained native scanout. No owner-loop fatal event appears.
Event 5430 reports five X protocol errors. The wrapper returned status 1,
with emergency=false and termios, keyboard mode, and keyd restored. A new login
started normally afterward.

The final evidence is preserved with suffix
`c37d8c2d-184e-4554-bd45-7119c6113790`, as described in the
[diagnostics acceptance record](../milestones/v4ycp9ba-daily-session-diagnostics-accepted-across-logout-and-login.md).

## Parallel investigation findings

The logout-policy and protocol-tally investigations independently verified the
installed `4b4f2841` source. `policy.rs` defines fatal protocol observations as
`count != 0 && (normal_session || application_proof)`, and `completion.rs`
returns an error for that condition. An existing test explicitly demands this
behavior. Commit `994033d93a321aa887a42b16396bc2b8648939d9`, “Advance milestone
8 Firefox session support,” introduced the rule as a deliberate compatibility
gate. It was carried into ordinary daily sessions; it is not a Boolean typo.

The count does not mean an Engine failure. `transport.rs` reduces handled
`XClientOutput::Error` replies, excluding only exact BadWindow probes with
resource zero for GetWindowAttributes/GetGeometry and a RandR
GetOutputProperty BadAtom probe with atom None. `authority.rs` tallies every
other reduced reply. Unsupported operations, stale resources, invalid
requests, and access refusals can therefore enter the same bucket as
compatibility defects. Relabeling every reply “expected” would conceal useful
frontend evidence without correcting the session outcome policy.

The wrapper did not invent status 1. The native process emitted
`sophia_session_result status=failed` before the wrapper returned; the CLI
emits that record when its command returns `Err`, and the wrapper and diagnostic
supervisor preserve the child's status.

## What the retained evidence cannot establish

The tally is emitted after the inner session loop returns, even on failure.
Its presence after native drain does not prove that the protocol-count check
was the first failed completion obligation. Renderer cleanup, presentation
shutdown, feedback draining, and other completion checks can fail before it.
The archive retains no owner-loop fatal event, but the original completion
error and renderer-cleanup status were not preserved.

The one retained tally row proves five observations in one retained tuple;
it does not prove five total observations. The tally holds up to 64 distinct
`(major, minor, error-code)` tuples. On overflow it clears the table and
accumulates a separate discarded count. The recorder removed that count,
major/minor opcodes, error code, and distinct count. Recorder health showed
zero queue loss; the missing information was removed by field reduction.
Neither the tuple nor the first failing completion check can be reconstructed
from this archive.

Earlier Ghostty/Thunar errors are not a defensible identification: the installed
release already includes the RENDER 0.6 correction and successful startup
probes. A hypothesis about the current requests must not borrow the identity
of those older failures.

## Recommended repair and regression scope

Keep X error replies as frontend compatibility observations. An ordinary
session's exit status should describe its lifecycle and cleanup result;
strict application proofs may continue to require zero unexpected replies.
Keep genuine authority, Engine, renderer, and cleanup failures fatal. This
change belongs in Session completion policy, not the WM or the X access checks.

Record an approved completion stage and cause before contextual string
wrapping. Preserve bounded numeric major/minor/error-code/count fields for
the tally's exact schema, with a reconciled total and discarded count. This
requires no XIDs, request sequences, application identity, or payloads.

A meaningful regression should send an invalid X request, observe its error,
then send a valid request and shut down normally. Ordinary-session exit should
remain successful with the refusal retained; strict proof exit should fail.
Independent cleanup/runtime failures must still return failure and retain
their actual cause. These checks separate compatibility assessment from
session liveness rather than expanding the expected-probe allowlist.

## Implemented repair and validation

The working candidate on `a8beb61c` removes the ordinary-session count-only
failure. Explicit application proofs retain their nonzero-protocol-error gate.
Authority access checks and the expected-probe allowlist are unchanged.
Schema 3 tally records retain safe opcode/error classifications, discarded
observations, and the reconciled total. Saturating accumulation also applies
when a full table resets.

Session failure records retain an approved phase and cause before tally
context wraps the error. Runtime failures keep their phase across cleanup;
publication-generation overflow now joins collected cleanup errors instead of
escaping cleanup early. No arbitrary error text or application identity enters
the default recorder. See [the operational contract](../../operations.md).

The headless CLI regression sends opcode 255, receives BadRequest, then receives
a valid GetInputFocus reply on the same connection. It reproduced status 1
before the repair and passes afterward with the nonzero tally and clean
shutdown. The diagnostic suite covers record scope, integer bounds, private
payload rejection, and safe phase/cause retention. Existing normal-session
lifecycle coverage also verifies that an explicitly requested startup proof
still fails and emits a failure record. The application-proof count gate is
retained by inspection; the new wire regression exercises ordinary sessions.

These deterministic checks do not establish physical acceptance. The installed
session still runs `4b4f2841`. Reinstall the candidate and observe one ordinary
logout; t019 remains open. The subsequent [runtime crash](fltuldiq-runtime-session-crash-retains-no-specific-cause.md)
is a separate unresolved failure and is not claimed fixed by this policy change.

Validation on 2026-09-06: all 12 diagnostic integration tests and both headless
CLI regressions pass. `cargo xtask check` passes, including workspace tests,
Clippy, profile/layout checks, and archive/verifier fixtures. Logs were observed
at `/tmp/sophia-logout-diagnostics.log`, `/tmp/sophia-logout-cli.log`, and
`/tmp/sophia-logout-xtask-check.log`. Two launcher safety-test references were
updated for the concurrently committed session-bus wrapper; its production
script was not changed. Modified Markdown links and open task-ID uniqueness
were checked, followed by `zk index` and `git diff --check`.
