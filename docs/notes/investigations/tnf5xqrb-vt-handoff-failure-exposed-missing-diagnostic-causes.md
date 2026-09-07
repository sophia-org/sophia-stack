---
id: tnf5xqrb
date: 2026-09-06
kind: investigation
status: awaiting-physical-acceptance
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

The diagnostics repair was committed as `1f8b6a43`. The installed rerun on
`5c929b84610c0cf1edc8d6b7f7c951dd1e7371cd` supplied the missing cause:
`handoff_missing_image`, during `export_images`, with three retained images.
The marked run was `00000001788745011061-e84982e5-6623-4be9-9713-05e465ab5be2`;
marker `fd9dbff9-e6fb-431a-b954-98df4fd6b310` was written after the session
ended. Its independent preserved copy has suffix
`f914012b-6b45-40d3-860c-4b5fe470e1f9`. Events 1629–1633 show preparation,
input revocation, and the failed export before any switch request. The recorder
retained 1,647 events with zero dropped records or storage errors.

The export took the first logical output's heads but required each of their
stores to contain the entire session's retained image set. That requirement
fails when an image has only been rendered on another output. Restore had the
same first-output assumption. The session has two outputs; the captured error
and a deterministic sparse-store fixture identify this coverage defect.
The log does not identify the individual missing image or its owning head.

Handoff now spans every enabled head and collects each head's available
promoted images. Their union must cover all retained identities. The backend
retains output, card, and connector membership with each collection, resolves
all replacement heads before import, and rejects missing or repeated owners.
Restore preserves sparse coverage and imports each image once per store,
including the opt-in shared-worker case. Genuine missing images and exporter
failures still abort the handoff, releasing partial snapshot leases.

VT release, startup recovery, and topology replacement use the same corrected
capture path. Handoff code now lives in its own backend module. No timeout,
WM policy, application authority, or disclosure boundary changes were needed.

## Validation and remaining work

The install/activation/rollback fixture passes, including CLI resolution across
both releases and rollback. All nine diagnostics integration tests pass,
including retained lifecycle fields, typed failure codes, and payload rejection.
The full `cargo xtask check` passed; its transcript is
`/tmp/sophia-vt-diagnostics-xtask-check.log`. Formatting, Cargo metadata, changed
note links, task-ID uniqueness, and `git diff --check` also passed. Concurrent
X11 extension edits belong to the other agent and are outside this repair.

The corrected native session builds. Six handoff integration tests pass:
exact identity admission and resume ordering, sparse two-output collection,
mirrored/private and shared-store restore, invalid or genuinely missing images,
and cleanup with preservation of a typed exporter failure. Evidence:
`/tmp/sophia-vt-handoff-build.log` and `/tmp/sophia-vt-handoff-tests.log`.
`cargo xtask check` passed the handoff tests but stopped in the concurrent X11
extension work: `render_refusals_split_between_not_offered_and_not_that_version`
sends a four-byte RENDER request that the new parser requires to be at least
24 bytes. The failure is at `extensions_dispatch.rs:1844`; its new request
length belongs to the other agent's uncommitted extension changes. The full
run is therefore not a pass. Transcript:
`/tmp/sophia-vt-handoff-xtask-check.log`. The canonical
`cargo xtask check layout`, formatting, Cargo metadata, note links, stable task
IDs, and `git diff --check` pass.

The installed correction `4b4f28418829d03191d53e533d7903d07d433633` passed the
reported tty3/tty7 round trip in session
`00000001788745936827-954d3556-800f-4929-b3c7-bdb25c873b25`. Events 923–928
record image capture, a drained native owner, the switch request, and seat
suspension. Marker `2c40685a-345f-4d33-bbac-2a0a0cd52aca` was written at boot
millisecond 227868106, between suspension at 227863714 and reactivation at
227870017. Events 931–934 record the replacement native owner, scene and image
restoration, and active seat. Presentation continued afterward; the recorder
remained running with zero dropped records or storage errors. The user reported
completion of the requested physical round trip.

A checksummed snapshot of this live evidence is preserved under
`sophia/session-investigations/` with suffix
`b9114a77-47d4-418b-b926-069d120b59f1`. This snapshot records a still-running
session, not a final logout outcome. The VT regression and independent-TTY
marking checks have now passed. Inspection of this marked run after a later
normal logout/login remains the physical acceptance step for
[t015 and t016](../plans/queue-05-3-make-failures-diagnosable.md).

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
