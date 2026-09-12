---
id: iux6ctsy
date: 2026-09-12
kind: investigation
status: investigating
tags: [investigation, input, session, x11, pinentry]
---
# Pinentry submission stalls before GUI exit and input recovery remains blocked

## Question and scope

Why does the configured egui signing prompt remain open after submission, and
why did ordinary keyboard input and attempted VT recovery become unusable?
The initial request was investigation and documentation for a later repair; that
investigation changed no implementation, GPG configuration, agent state or
installed release. The subsequent approved t077 recovery implementation is
recorded below. The original GUI stall remains a separate investigation.

**Root-cause status:** submission reaches pinentry's completion handler; the
exact subsequent GUI/native stall remains unlocated. A separate, concrete
control-recovery liveness gap is confirmed in source and an isolated executable
probe. The incident confirms no-focus keyboard suppression and outstanding input
deliveries blocking VT handoff, but does not identify the missing receipt or prove
that this liveness gap caused the original GUI stall. Do not collapse these
findings into one proven egui, Mesa or Sophia defect.

## Reproduction and operator observations

1. The operator ran `echo "cache" | gpg --clearsign > /dev/null`.
2. Ctrl+V visibly populated the pinentry field. The prompt did not complete.
   Terminal `--pinentry-mode loopback` subsequently unlocked the key.
3. A direct Assuan probe, independent of GPG and its agent, reproduced the issue:

   ```sh
   printf 'SETDESC Diagnostic only: enter test\nGETPIN\nBYE\n' | ~/.cargo/bin/pinentry-egui >/dev/null
   ```

4. The operator typed the literal dummy string `test` and pressed Enter.
   The field cleared, but the window remained open.
5. The operator subsequently reported inability to click OK or Cancel and loss
   of keyboard usability, then ended the desktop session.

Visible insertion rules out total clipboard-delivery failure in this occurrence.
The dummy reproduction rules out GPG key unlocking as a necessary trigger. These
observations do not prove that every pointer event was delivered or that all
physical key handling stopped.

## Exact incident and preserved evidence

Session: `00000001789185326396-eb54265e-ff0f-4282-baff-54920c587370`.
Installed Sophia: `a23ac9ae37bd491afecd1f3c55117f8b229ef6c9`;
binary SHA-256 `61eefac3e35960474f9ca82234b4007d2f5aefd38d91c508ac16c05e8529cd26`.
Desktop profile mode was `user`, generation 1. Hagia began with digest
`e5c9f3ada0a6dd6936bf0de4f51deedf9efcd5acbccb8f77aa6cad352139aa92`;
at UTC millisecond 1789209306352 it was replaced at WM epoch 2 by digest
`6cb15ad180f7c9f227408510fe2951386787ad5c19a248c7180b3c28b9cf19b8`.
The latter was active during the incident; do not attribute its policy behavior
to the initial binary without checking that identity.

The installed `pinentry-egui` is Cargo version 0.1.1, binary SHA-256
`5c8f3c898b0401036f198de1419d6b73243f3ed152f5598dadf1fe8d90f5484e`.
Embedded source paths identify eframe/egui 0.33.3 and winit 0.30.12, agreeing with
the archived crate lockfile for the paths reviewed here.

Private evidence: `.artifacts/pinentry-egui-20260912/`:

- `session/`: all four stopped recorder segments, identity, manifest, health,
  outcome, lifecycle and recovery records. Health ends at sequence 409520 with
  zero discarded records, rotated bytes and storage errors.
- `sources/`: versioned upstream crate archives and the Sophia source used for
  the control/timeout analysis. `review-identity.json` records URLs and identities.
- `observations.txt`: operator reports and the pre-exit process wait sample.
- `analyze.py`, `summary.json`, `focus-transition.tsv`: reproducible extraction
  of the following counts and transition records, without input payloads.
- `held-control-probe.rs`, executable and `.log`: the deterministic liveness
  probe. `SHA256SUMS` covers the retained artifacts.

No clipboard contents, real passphrase, key material or process-memory dump was
collected. The dummy prompt was PID 10977, parent 12328, on `DISPLAY=:77` with
`XDG_SESSION_TYPE=x11`. Its main thread waited in `futex_do_wait`; ancillary
threads waited in futex, poll and epoll. A futex wait is **not** a userspace stack
trace and cannot by itself identify GL swap, a mutex or an event-loop bug.
Kernel stack/syscall reads were denied. Automatic approval review refused a
privileged debugger attachment because pinentry is credential-handling software;
no debugger attached. No replacement inspection of its memory was attempted.

The attempted live X-property probe did not yield a mapping. In particular,
**no recorded XID has been positively tied to PID 10977**. Recent surface
numbers and timing are not application identities.

## What the session actually did

| Record | UTC milliseconds | Evidence |
| --- | --- | --- |
| 401246 | 1789216077820 | Last positive `key_routed_count`, at 12:27:57.820 UTC. |
| 401260 | 1789216077957 | A WM action is processed. Its identity is not retained. |
| 401270–401278 | 1789216077959–7961 | Layout transaction 285 commits; a key-release record names surface 144703491, and controls name that surface and 4194318. Control kinds/reasons are not retained. |
| 401283 | 1789216077968 | Chrome reports five eligible frames, all unfocused, and zero focus rings. |
| 401304 | 1789216078127 | Subsequent keyboard input reports `key_no_focus_count=1`. |
| 407320–407322 | 1789216141880–2381 | VT request waits 501 ms, then is rejected with `modifier_release_timeout`. |
| 407329–407331 | 1789216142418–2920 | A second VT request fails the same way after 502 ms. |
| 409502 | 1789216169125 | Native suspension drains. |
| 409511 | 1789216169169 | Completion reports `phase=layout_validation`, `failure_code=unclassified`. |

After record 401246, the routing counters total 332 observed keys, **zero routed
keys**, 269 no-focus suppressions, and 29 WM actions. Pointer routing still counts
277 events and no lease waits/rejections in those aggregate input records. The
same interval contains **2,736 present retirements and 2,736 scanout records**.
Thus application typing failed while control-plane actions, some pointer routing
and rendering continued. Routing counts do not prove client consumption or a
visible response from the prompt.

The final outcome is exit 130 with `emergency=true`; the input guard records
`triggered`, and recovery reports restored termios/keyd and graceful shutdown.
The `layout_validation` failure occurs in completion checks after native drain.
It does not prove a pre-existing owner-loop crash, and is distinct from the
older `phase=control` signing incidents. Its exact failed predicate was not
retained.

## Confirmed submission boundary and separate floating defect

In pinentry 0.1.1 `src/main.rs`, `pin_dialog_ui` sets `submitted=Some(true)`
when the single-line field loses focus on Enter, or when OK is clicked.
`PinDialog::update` then sends `DialogResult::Pin`, **clears the field**, and
requests `ViewportCommand::Close`. `show_dialog` waits in `eframe::run_native`
before consuming that queued result. Only afterward does the Assuan loop send
`D ...` and `OK` to its caller.

The operator's cleared dummy field therefore locates execution after accepting
Enter. It does not establish that the GUI close command completed, that the
queued result was consumed, or that GPG ever received the password.

The matching eframe glow implementation runs application update and rendering,
then `swap_buffers`, then processes viewport output. egui-winit turns Close into
a viewport close event; eframe recognizes it on a subsequent update and exits.
That path gives specific boundaries to instrument. A render/presentation wait
before processing Close, or failure to service the following update, can retain
the prompt after submission. Neither is yet proved by a userspace backtrace.

Separately, `show_dialog` supplies only a title, 400x200 size and
`resizable=false`. It does not declare dialog type or a transient parent, and
ignores Assuan OPTION values. Sophia's X authority floats declared dialog or
transient windows (`property.rs`, `window.rs`). The missing application hints
explain why automatic dialog treatment cannot be assumed. A nonresizable normal
window is not equivalent to a declared dialog. This does not explain the hang.

A further independent source defect converts each non-ASCII UTF-8 password byte
into a Unicode scalar before re-encoding. Public dummy text demonstrates byte
corruption. It is **not** the explanation for the ASCII `test` reproduction or
for an open, unresponsive window, and no assumption is made about the user's
real password.

## Confirmed recovery liveness gap

The following reviewed source files are unchanged between installed `a23ac9ae`
and review HEAD `677e401002ba7b3e3323862a6399ae720bd8a06d`.

1. `live_session/owner_loop/session_control.rs`: key flushes add delivery IDs to
   both `input_delivery.pending` and `client_key_release_barrier`. Control service
   passes `dispatch_ready = client_key_release_barrier.is_empty()` to the shared
   queue. The ordering prevents focus/close from overtaking releases.
2. `session_control.rs`: `dispatch_eligible_at` starts only when that gate opens.
   Undispatched entries have no queue timeout until that timestamp exists.
   Consequently a permanently held release barrier can withhold focus/close
   indefinitely even though queue and acknowledgement limits are nominally 500 ms.
3. `input_delivery.rs` and `live_session/owner_loop_state.rs`: pending IDs carry
   no individual age. The delivery timeout runs only with `wait_started_at=Some`.
   Ordinary key flushes do not arm it; assignments are in synthetic/physical
   proof and emergency paths. It is not a normal-session per-receipt watchdog.
4. `live_session/owner_loop/lifecycle.rs`: VT handoff waits on **all** pending
   input deliveries. At 500 ms it rejects the handoff and resets chord/modifier
   tracking, but does not terminally settle those pending receipts. Despite its
   name, `modifier_release_timeout` is not proof of a physically stuck Ctrl key.

The two retained VT timeout records prove that pending input delivery was
nonempty at those two deadlines. They do not prove which client/delivery it
named, that it remained the same entry throughout, or that a release-barrier
member rather than another delivery held the VT gate.

The artifact probe imports the real `session_control.rs`, linked to existing
protocol/frontend libraries; it does not substitute a model of that queue. With
dispatch held false, both FocusSurface and CloseSurface remain pending at logical
0, 1, 10 and **120 seconds**, with zero dispatches, outcomes or timeouts. Opening
the gate immediately dispatches the command; leaving its ACK missing then
produces the ordinary timeout after 501 ms. No wall-clock sleep or real client
was used. This establishes the unbounded pre-dispatch case, not that it caused
this particular missing receipt. The existing barrier test covers a one-ms hold
followed by release; it does not exercise permanent loss.

There is a concrete way a client-side stall can become a wider recovery failure,
but its activation in this incident remains a hypothesis. Similarly, no-focus
suppression is directly demonstrated, while whether the policy intentionally
selected an empty output or lost required focus cannot be recovered from the
filtered records. `wm/commit.rs` intentionally clears seat focus when the active
output projection has no focus. Do not classify that rule alone as the defect.

## Later repair and acceptance work

- Add bounded, payload-free receipts: delivery/client/surface identity, queued
  age, terminal disposition, release-barrier membership, and held control identity.
  Record the exact focus transition and safe reason, plus typed completion
  validation failures. Do not collect keystrokes, clipboard text or passphrases.
- Give ordinary delivery and release-barrier waits a bounded lifecycle. On
  failure, contain the offending client/route and settle its exact obligations;
  preserve key-release ordering. Do not fix this by blindly dispatching controls
  before releases or by treating an unacknowledged event as delivered.
- Establish an authorized recovery path when a client cannot acknowledge input;
  repeated VT rejection must not be the sole result. Exercise both pending key
  release and unrelated pending pointer-delivery cases.
- Use only the direct dummy prompt for the next physical reproduction. Mark
  submit accepted, render/swap entered and returned, Close processed, next update,
  GUI return and Assuan response, without any value/length output. Correlate its
  actual PID/XID with input receipts and Present feedback. A debugger is not a
  prerequisite: an explicitly instrumented dummy probe can supply these stages.
- Separately fix/verify dialog hints and UTF-8 encoding. Do not call either the
  submission or keyboard-lockout fix without a distinguishing reproduction.
- Acceptance: dummy Enter/OK closes promptly and completes the protocol; Cancel
  works; ordinary typing and click focus in another application continue; a
  failed/stalled dummy client cannot strand focus/close/VT recovery; and session
  shutdown finishes with no unexplained pending obligations. Queue tests alone
  are insufficient physical acceptance.

No physical retest was requested after the operator closed the session. Xvfb and
xdotool were unavailable, so no independent stock-X-server comparison was run.
The isolated queue probe passed; no full build gate was run for this documentation
and no repair is claimed.

## Connections

- [Existing signing-dialog task t077](../../../todo.md) remains open. This
  recurrence adds input-recovery evidence; it does not establish that the older
  control-phase exit has the same cause.
- [Earlier installed control-phase failure](3qlzcrya-installed-owner-exits-with-an-unclassified-control-phase-failure.md)
  lacked the specific typed failure and remains a separate incident.
- [Earlier signer-paste investigation](fltuldiq-runtime-session-crash-retains-no-specific-cause.md)
  records an inconclusive synthetic-input probe; visible paste is now confirmed
  for this occurrence only.
- [Hidden-window input-standing repair](ohkzr8kg-unmapped-dialogs-retain-input-ownership-after-leaving-the-scene.md)
  explains why retained pixels must not grant input authority; neither current
  unmap nor an actual input grab was observed here.

## t077 recovery implementation

The approved scope is bounded desktop recovery and diagnosis. An input delivery
now has a bounded ticket registered before ingress, including its exact surface
and generation, seat, control epoch and original admission time. The frontend
binds the actual receiving X connection after resolving grabs. The six-second
absolute deadline preserves the existing five-second keyboard subscription
grace; later input never refreshes it. Accepted ticket credit remains charged
until terminal receipt observation (and consumption of any cancelled unresolved
queue entry).

A bound timeout revokes the exact X connection and shuts down a separate socket
clone before settling outstanding deliveries as failures. It does not lock the
output writer, kill the process or touch gpg-agent. Current-event and writer-exit
guards cover disconnect, write error and dropped queued work. An unresolved
expired route is cancelled before dispatch and retains a tombstone until intake
consumes it. Only authenticated client+delivery receipts release owner barriers;
late or duplicate receipts cannot restore focus or report a false flush.

Focus/close controls preserve their release barrier and existing 500 ms queue
and acknowledgement budgets. A client-specific timeout disconnects that client,
retires its controls and clears its pressed/repeat/focus/lease standing; ordinary
frontend surface removal drives policy repair. Healthy connections remain live.
A requested VT gets 500 ms for receipts, then cancels or disconnects remaining
old-seat deliveries before handoff. Typed shared failures and strict proof
requirements remain errors rather than being counted as successful recovery.

The recorder retains bounded identities, ages, release-barrier membership,
terminal outcomes and typed completion causes. A committed empty active output
has its own explicit focus-clear reason. No keycodes, text, passwords, titles or
arbitrary client messages are added.

Deterministic evidence includes a real blocked Unix-socket writer whose mutex is
held, with an independently progressing healthy socket; grab-owner routing;
pre-routing cancellation; exact receipt correlation; timeout/completion ordering;
current/queued writer settlement; an owner regression for both focus and close;
VT key+pointer revocation; stale controls; and recorder redaction. Disabling the
actual owner watchdog causes the new barrier regression to fail. The positive
TLA model explored 349,360 distinct states; no-deadline and early-barrier negative
controls fail the expected liveness and safety properties respectively. The
model does not constitute emitted-runtime-trace or physical acceptance.

Full gate/release identity and operator acceptance are tracked below. t077 stays
open until an installed dummy-input reproducer demonstrates recovery, healthy
focus/close, VT round trip, continuing presentation and clean shutdown.

## Pending pinentry work

The egui completion/native event-loop stall is **not fixed by t077**. A future
repair must instrument a fresh dummy-password instance (never attach to a real
credential prompt) at submission, result-channel delivery, viewport close,
`run_native` exit and Assuan response. Confirm the first absent transition before
changing rendering or event-loop code. Separately test dialog/transient hints
and intended floating placement, plus byte-correct UTF-8 Assuan encoding. The
observed ASCII `test` failure is not explained by the UTF-8 encoding defect.

## t082 diagnostic delivery

The first t082 delivery prepares a controlled dummy reproduction; it does not
change Sophia, installed pinentry, GPG configuration, dialog placement or UTF-8
encoding. The completion/native-loop cause remains unproved. Instructions and
source are in [the probe README](../../../tools/probes/t082_pinentry/README.md).

The private baseline and instrumented pinentry-egui 0.1.1 builds use the same
archived registry dependency versions, including eframe/egui/egui-winit 0.33.3,
winit 0.30.12 and glutin 0.32.3. The builder verifies archive hashes and fails on
dependency-version drift. Instrumentation covers submission, channel send,
Close enqueue/processing/observation, paint/swap, event-loop exit and return,
`run_native` return and the Assuan response. The actual X11 handle comes from the
created window; surface ordering is not identity evidence.

A separate nonblocking pipe carries fixed stage names and numeric metadata.
Sequence/drop accounting and an independent heartbeat distinguish an observed
unfinished call from missing diagnostics. Direct `SETDESC/GETPIN/BYE` requests
use only public dummy input. The harness validates `test` in memory, discards
arbitrary child stderr and never saves the entered value. It bounds each child
to 60 seconds, with a further 15-second deadline after instrumented submission,
and records exact-child timeout termination separately from normal GUI closure.
The uninstrumented baseline has only the total lifetime deadline.

Eleven hardware-free harness tests pass, including real trace-pipe writes,
concurrent writers, overflow, timeout isolation from a healthy process, response
redaction, full marker whitelist coverage, analysis boundaries and wrapper
preparation. All six upstream in-memory UI tests pass against the instrumented
source. These do not prove native closure, X routing, client progress or input
recovery on the installed compositor.

Prepared host artifacts are `.artifacts/t082-probe-v4/identity.json` (archive,
generator, generated-source and binary identities) and
`.artifacts/t082-validation/` (test logs and wrapper preflight). The fixed command
is `/tmp/sophia-pinentry-trace/capture.sh`. Its launcher and three generated shell
files pass `bash -n`; preparation was checked against the packaged t077 release.
It refuses to run without installed commit
`2f38ac757f8ea70f001bba664e8c8acc4984ef4b`. Installation and the attended VT run
remain operator steps. No physical reproduction was performed for this delivery.

The matrix starts with baseline Enter, then instrumented Enter, OK, Cancel,
Escape and WM close in fresh processes. A failed instrumented specimen stops
automatic progression. Keep the same VT during each specimen, then separately
verify healthy terminal input, focus, VT round trip and clean logout. Correlate
the explicit PID/XID with session records and identify the first absent native
transition before proposing a repair. A non-reproduction or lost trace leaves
t082 open rather than establishing success.

### Installed-release retargeting

The operator subsequently installed `0.1.0-18f70f862300` (commit
`18f70f862300ff1cf2cd82704217bb1882cdd396`). Its manifest and full release
checksums were verified, and git ancestry confirms it contains `2f38ac75`.
The launcher now pins that exact installed release; the earlier installation
prerequisite is satisfied. Both the materialized capture and all three wrappers
generated from this installed release pass `bash -n`. This is preparation only:
the attended dummy reproduction and physical acceptance remain outstanding.

### t077 recovery gap exposed by the first t082 capture

Capture `/tmp/sophia-pinentry-trace/run.TRaWWi` ran the deliberately pinned
`18f70f862300` release. The first uninstrumented baseline started as PID 6664 at
15:09:50.018 UTC on 2026-09-12. The operator reports clicking OK; the entered
public word is uncertain. Only `case_start` survives: there is no protocol
terminal, summary, harness-termination marker or instrumented specimen.

At 15:10:18.424940 the session begins retiring outstanding deliveries for client
2 as `ClientDisconnected`. Its final error at session.raw.log line 16021 includes
`persistent X authority server failed: failed to write XI2 generic event: Broken pipe (os error 32)`.
This is about 28 seconds into the baseline's 60-second budget. The evidence does
not establish a harness timeout, successful pinentry exit, pinentry crash, or the
original native-loop stall. Nor does it establish client 2's PID by itself.

The session-fatal write classification is a t077 recovery gap, with t082 blocked
on it. In `connection/writers/input.rs`, the XI2 generic-event write wraps its
I/O error with `X11SetupSocketError::new`; both client classification flags are
false. The writer absorbs only `client_disconnect`, returning other errors into
the connection's writer join and ultimately the authority failure. Existing
`is_x11_client_disconnect` already recognizes BrokenPipe, ConnectionReset and
UnexpectedEof; the core-record writes use it but neighboring paths do not.

Direct inspection finds eight unclassified I/O sites in input.rs: core leave,
core enter, XI2 leave/focus-out, XI2 enter/focus-in, XKB state, XI2 generic,
emulated XI2 wheel-button, and final flush. Two other write sites classify peer
disconnects correctly. Counting constructors inside both branches would double
count those classified sites. The reported additional four sites in
`writers/records.rs` are not confirmed on master `3a2139bf`: that file contains
no stream writes or flushes. Audit adjacent protocol/control writers as part of
the repair rather than treating the approximate twelve-site total as verified.

Repair scope: apply consistent typed peer-I/O classification across the writer
subsystem without downgrading lock, encoding or shared-authority errors. Exercise
real closed sockets through affected writer paths, verify exact delivery
settlement without false Flushed outcomes, and demonstrate that a healthy peer
and the authority stay alive. Keep a non-peer-error control that remains fatal.
A helper-only test is insufficient because a missed call site caused this gap.
Then repeat the attended dummy capture on the checked repair release. No further
VT reproduction or runtime repair is performed by this filing.

A peer disappearing at an affected write can terminate the session; the defect
is not specific to pinentry. The current installed symlink has since moved to
`fd2b86c72f54`, but the retained capture pin still names the actual `18f70f86`
specimen. Neither installation change constitutes physical acceptance of t077.

### Peer-write recovery repair

The approved repair classifies the eight audited input I/O errors with the
existing BrokenPipe/ConnectionReset/UnexpectedEof definition. Only I/O closures
change: poisoned locks, codecs and shared-authority errors remain fatal. The
bounded adjacent audit also found bare protocol/control flush errors and a
control write that swallowed peer disconnect as `Ok(())`. The latter could reach
`Delivered` acknowledgement despite writing no record. It now returns a typed
connection-local error through the control caller, preventing that success ACK;
the frontend supervisor already handles that class without stopping authority.

Regression tests use actual Unix sockets through the production input writer for
XI2 generic, core enter/leave, XI2 enter/leave and emulated-wheel paths. Leave
cases first complete a successful delivery before closing the peer and changing
the pointer destination. Each failed delivery has one `ClientDisconnected`
receipt, never `Flushed`. Its actual writer result passes through the production
frontend reaper; a healthy production writer still emits a complete XI2 record
after each reap. The fixture supplies the worker join wrapper, rather than
performing a whole session or client handshake, so this is worker-boundary
coverage, not physical acceptance.

Additional tests exercise a closed-socket protocol writer, the actual control
record writer's error return, the disconnect-kind classifier, and a poisoned
output lock which must retain fatal classification. The control test proves the
record writer fails; the following `?` before `send_ack` is the static evidence
that no successful ACK follows it. It does not observe a full control-queue
round trip. XKB's later write and UnixStream's no-op flush are audited sites,
not individually fault-injected native failures.

A negative control restoring the original XI2 generic-event constructor fails
the regression with `WriteFailed` instead of `ClientDisconnected`; restored
positive tests pass. Evidence is under `.artifacts/t082-validation/peer-write-*`.
The first full-gate attempt inherited a real atomic-scanout opt-in and failed its
atomic submission. That environmental failure is retained separately; offline
validation clears inherited SOPHIA_/HAGIA_ proof flags. It is not acceptance of
any physical path. No repeated attended pinentry run has occurred.

The clean-environment `cargo xtask check` completed successfully: 264 passing
test-result groups, clippy, the unchanged source-layout ledger, promoted archive
verification and installed native verifier fixtures. Claude reviewed the repair
read-only and approved it. The existing TLA recovery model is unchanged; no new
model-checking claim is made for this error-classification patch. Installation
of the repair, capture retargeting and attended acceptance remain outstanding.

### Follow-up capture and shutdown instrumentation

The attended capture `/tmp/sophia-pinentry-trace/run.QUTcIk` used the installed
peer-write repair `1a59ab8c1406`. Its instrumented PID 23234 reported XID
8388611 (0x800003), explicitly mapped to authority client 4. Enter sent the
result and queued Close; close acceptance, paint, swap and viewport processing
all returned. The last application marker was viewport_output_return. The
heartbeat remained fresh with no reported trace loss, but native return and the
Assuan result were absent before the 15-second submission watchdog terminated
the child. The session continued. This is not evidence of a blocked swap or of
the earlier session-fatal peer-write defect recurring.

Client 4 has no recorded major-4 DestroyWindow dispatch in that specimen.
Its final recorded requests include GLX context/window destruction, FreePixmap
and QueryExtension sequence 872. A nearby DestroyWindow belongs to client 5.
Consequently the independently discovered missing DestroyNotify cannot explain
this specimen without first establishing that the relevant destroy request was
issued and reached the server. A missing dispatch does not prove the client
never called destroy_window: teardown may stall earlier, or a request may remain
buffered. The eframe CloseRequested path destroys running state without itself
requesting event-loop exit; a subsequent window event with no running state can
produce Exit. That is a branch to test, not an established cause.

The approved diagnostic extension is materialized as immutable
`.artifacts/t082-probe-v6`, selected by the fixed capture script. It patches
private, checksum-verified winit 0.30.12 and x11rb 0.13.2 copies in addition to
the original three crates. Baseline and traced builds retain the original
dependency versions. The trace now brackets autosave, minimized-state querying,
saving, on_exit, painter destruction, running-state drop, the Window::drop body,
DestroyWindow request construction and existing X11 waits/flushes. Window-event
categories and running-state/Exit markers distinguish later-event handling.
No flush, sync, synthetic event or forced exit is added.

Request-wait correlation includes thread, opaque connection ordinal and sequence.
All open spans remain available; ordinal overflow, dropped or invalid records
make the result inconclusive. Window::drop's body-return marker precedes automatic
field destruction; it is not an assertion that the entire value was dropped.
A returning request/flush call is not proof of server processing or client event
consumption.

The fixed launcher now runs only one instrumented Enter specimen: type the dummy
word test and press Enter once. The full six-case matrix remains an explicit
runner selection. Lifetime and post-submission deadlines remain 60 and 15 seconds,
with cleanup confined to the spawned child. No GPG configuration, installed
binary, desktop profile or live session is changed by preparation.

Offline validation: both release binaries built; 15 harness tests and all six
upstream in-memory UI tests passed. Original lockfile archive checksums, bundle
hashes, the retained installed release manifest/checksums, and bash syntax for
the fixed launcher and all generated wrappers were verified. The whole-repository
source-layout audit is not green: it reports oversized X11 socket tests outside
this probe's changes; the concurrent runtime owner was notified. No new physical
run was performed, and t082 remains open pending the instrumented specimen.
