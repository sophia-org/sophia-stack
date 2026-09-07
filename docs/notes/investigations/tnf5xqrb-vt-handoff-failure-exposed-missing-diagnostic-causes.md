---
id: tnf5xqrb
date: 2026-09-06
kind: investigation
status: investigating
tags: [investigation, session, tooling]
---
# VT handoff failure exposed missing diagnostic causes

## Question

Why did switching from the installed Hagia session on tty7 to tty3 return the
user to greetd, and why was `sophia` unavailable from the TTY?

## Evidence

The installed Sophia release was `0.1.0-c01663aaea09`, with Hagia source
`0e5e86f26fa32a8d926d8e8de8d3dc0152018686`. The failed run was
`00000001788743375093-ef5c2c4c-345b-4afc-a4f0-d816f34de8d6`.
Its record remains under `$XDG_STATE_HOME/sophia/sessions/`; an independent
checksummed copy was preserved under `sophia/session-investigations/` with
suffix `13bc6571-e1ab-438a-861a-c3eb1b20d791`.

Events 3933–3935 record VT preparation and input epoch 3. Event 3936 records
an owner-loop fatal error 225 ms later. There is no intervening successful
renderer-image capture or VT request record. Final cleanup drained native
scanout, cleared images, and returned status 1. The emergency guard did not
trigger; TTY recovery restored termios, keyboard mode, and keyd.

The recorder wrote 3,950 events with no discarded records or storage errors.
However, its field filter removed the VT statuses, phase/source values, and
original fatal error. Zero queue loss does not mean sufficient diagnostic
content. The failure is narrowed to release preparation, but the original
cause cannot be established from these records.

## Findings and changes

The installed command surface omitted the CLI, although every release already
contained `target/release/sophia`. Activation and rollback now expose
`/usr/local/bin/sophia` through `current/target/release/sophia`. This also works
with older releases, without modifying their immutable contents. Creating that
host symlink immediately requires the user's sudo password; `sudo -n` could
not perform it.

Daily records now retain the finite VT lifecycle vocabulary. Renderer handoff
failures emit their phase, retained-image count, and a reduced failure code
before returning the original error. Owner-loop fatal records also retain a
reduced code. Codes come from typed renderer variants or exact internal
invariant messages; unknown errors become `unclassified`. Raw application
paths, text, and arbitrary error messages remain excluded.

No renderer ownership or VT recovery behavior has been changed. Missing
promoted images, worker/export failure, and failures before image export remain
candidates, not established causes. In particular, this is not evidence that
extending a timeout would fix the incident.

## Validation and remaining work

The install/activation/rollback fixture passes, including CLI resolution across
both releases and rollback. All nine diagnostics integration tests pass,
including retained lifecycle fields, typed failure codes, and payload rejection.
The full `cargo xtask check` passed; its transcript is
`/tmp/sophia-vt-diagnostics-xtask-check.log`. Formatting, Cargo metadata, changed
note links, task-ID uniqueness, and `git diff --check` also passed. Concurrent
X11 extension edits belong to the other agent and are outside this repair.

The next installed run must identify the precise failure boundary and cause if
the VT problem recurs. Only then can a renderer correction be tied to this
incident. The independent-TTY marker and later-login acceptance criteria for
[t015 and t016](../plans/queue-05-3-make-failures-diagnosable.md) remain open;
this failed attempt does not satisfy them.

## Connections

The [daily diagnostics implementation](e84g9ivq-durable-daily-session-diagnostics-and-incident-markers.md)
introduced the recorder and owns its original validation. The
[operator contract](../../operations.md#mark-and-investigate-a-problem)
describes the installed commands.

Earlier investigations explain why
[detached renderer work must settle before export](../sources/2026-08/legacy-active-0094-2026-08-07-vt-handoff-must-settle-detached-renderer-work.md)
and why [retained images must survive renderer replacement](../sources/2026-08/legacy-active-0119-2026-08-06-vt-resume-transfers-renderer-owned-snapshots.md).
Those historical fixes remain in the code; their old symptoms alone do not
establish this incident's cause.
