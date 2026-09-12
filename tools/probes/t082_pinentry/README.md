# t082 dummy pinentry probe

This is a diagnostic, not a pinentry repair or a replacement GPG pinentry.
The first delivery isolates the native completion boundary. Floating/dialog
hints and UTF-8 encoding remain separate work. No Sophia runtime code changes
are required for this probe.

## Build and check

`build.py` verifies the archived crate hashes, extracts private copies and
builds baseline and instrumented binaries with the same registry dependency
versions. It refuses an existing output directory. The generated projects have
their own Cargo workspaces; they never patch the Cargo registry cache or this
repository's workspace. Only the instrumented copy receives tracing hooks.

From the main Sophia checkout, the reproducible build is:

```sh
python3 -B tools/probes/t082_pinentry/build.py \
  --sources .artifacts/pinentry-egui-20260912/sources \
  --output .artifacts/t082-probe-v6
python3 -B tools/probes/t082_pinentry/test_probe.py
```

The prepared bundle already exists at that path on this host. For a new build,
choose an unused output directory and deliberately retarget `capture.sh`.
`--target DIR` can reuse a Cargo build cache. Builds default to offline operation;
any missing locked dependencies must be fetched explicitly before retrying.
`identity.json` records archive hashes, generator hashes, generated Rust/manifest
hashes, source base commit and both executable hashes. Generator hashes, rather
than the base commit alone, identify an uncommitted diagnostic build.

The six upstream in-memory UI tests can be run with `cargo test --locked --release
--bin pinentry-egui` from the generated instrumented project. They test egui
controls, not native closure, Present feedback or GPG integration. Harness tests
compile the real trace helper against a stand-in process; they do not open a GUI.

## Operator capture

The first physical run exposed a session-fatal peer-write classification gap.
The follow-up repair is now installed. The capture pins verified release
`0.1.0-1a59ab8c1406`, containing that repair and the initial t077 recovery.
No installation or sudo is needed for this run. The manifest, release checksums
and generated wrapper syntax have been checked against this installed build.
An exact pin keeps the specimen reproducible; if that release is removed,
verify and deliberately retarget rather than accepting an arbitrary newer build.
Installation is not physical acceptance: the attended dummy run remains needed.

End the old desktop normally. From a local VT, as the ordinary user, run:

```sh
/tmp/sophia-pinentry-trace/capture.sh
```

The capture command takes no arguments and needs no sudo. It verifies the exact
release manifest and checksums, copies the session wrapper into its private
capture directory and checks all shell syntax before supervision. It never edits
installed release files, the user's profile, installed pinentry or GPG config.
The exact packaged Hagia/Narthex binaries are used with the existing desktop
profile. Source checks in `prepare.py` refuse an unexpected wrapper.

A normal interactive terminal and one instrumented dummy prompt start automatically.
Type only the public word `test`, then press Enter once. The fixed launcher selects
`--case instrumented-enter`; `runner.py --case full` retains the six-case baseline
and instrumented matrix for separately chosen runs. Each case uses a fresh process
and a direct `SETDESC/GETPIN/BYE` exchange. Inherited Wayland endpoints are removed.
No real GPG operation is involved.

Stay on the same VT during a specimen. Each process has a 60-second total limit;
instrumented submission also starts a 15-second limit. The baseline has no submit
telemetry, so only its total limit can be enforced. Later frames/heartbeats never
refresh deadlines. Timeout sends TERM, then KILL after one second if necessary,
only to the exact child created by the harness. Timeout cleanup is explicitly
recorded and must not be mistaken for normal window closure.

Verify ordinary typing and focus in the terminal. After this single specimen finishes or its watchdog cleans it up, use
Ctrl+Alt+Delete for normal logout. Do not switch VTs during the specimen. Ctrl+Alt+Backspace remains the existing independent emergency
input guard. A controlled capture does not prove that emergency recovery works
unless the operator actually exercises and records it.

## Evidence and interpretation

The trace uses a dedicated inherited nonblocking pipe, not stdout or stderr.
Each write is below PIPE_BUF; simultaneous writer records cannot interleave.
At most 65,536 ordinary records are emitted, plus heartbeat/exit diagnostics.
The independent 100-ms heartbeat reports accumulated drops even when the GUI
thread is stuck. Missing sequence IDs, invalid records, overflow, or absent
heartbeat/exit evidence make the result inconclusive. A fresh heartbeat confirms
a recent prefix of the trace, not events after that heartbeat or after cleanup.

Markers name channel send, close enqueue, paint, swap, viewport handling, close
observation/acceptance, event-loop exit/return, native return and protocol writes.
They contain timestamps, sequence IDs, PID/thread ID, numeric status and the
actual X11 window handle, never entered data or password length. The harness
validates the dummy response in memory and retains only its outcome. Arbitrary
child stderr is discarded, with only a presence flag retained. Session logs use
existing protocol metadata and redacted input diagnostics; there is no global
trace-level logging or process-memory inspection.

```sh
python3 -B /tmp/sophia-pinentry-trace/analyze.py /tmp/sophia-pinentry-trace/run.XXXXXX
```

`response_after_submit_ms` measures harness receipt times, not the exact client
execution interval. Per-case `summary.json` and `stages.jsonl` distinguish observed incomplete calls,
missing later transitions, protocol failure and harness cleanup. The analysis is
not a root-cause verdict. Correlate the probe's explicit PID/XID with the session
trace; do not infer application identity from surface ordering. Require an
explicit surface/client mapping before attributing input or Present records.
`session_present` denotes retirement, not receipt of an incoming Present.
Feedback routing is not proof of client consumption. If the baseline or traced
specimen does not reproduce, record that fact and keep t082 open.

After a physical run, retain its release identity and ordered timeline in the
linked investigation, identify the first missing transition and propose the
repair against that evidence. No live capture or GUI repair is claimed by the
offline tests in this directory.

### Shutdown trace (v6)

The private patches additionally cover locked winit 0.30.12 and x11rb 0.13.2;
their archives are copied into the source archive directory from Cargo's cache
and verified against the checksums in the original pinentry lockfile. No cache
source is modified. The earlier v5 build remains immutable and is not selected.

Markers bracket autosave, the minimized-state query, save_and_destroy, saving,
app on_exit, painter destruction and explicit drop of the taken running state.
The explicit drop occurs where that local previously left scope. Winit markers
bracket the existing Window::drop body and destroy_window call. The body-return
marker is **before automatic field destruction**, not proof that the entire
Window value has finished dropping. The outer running-drop marker covers that
larger interval.

Existing XPending, XCB flush, reply wait, checked-request wait and event wait
calls are bracketed. No flush, sync, event injection or exit is added. A request
call returning does not prove bytes reached the server; a flush returning does
not prove the server processed DestroyWindow. Correlate the actual XID with the
authority's dispatch trace. A wait marker describes an observed call interval,
not proof that its underlying call blocked for the entire interval.

Window-event kinds are fixed numeric categories: 0 other, 1 Destroyed,
2 CloseRequested, 3 RedrawRequested. A separate boolean records whether running
state still exists; the Exit-producing branch has its own marker. No event
payload is logged.

Connection IDs are bounded per-thread opaque ordinals (1..16), not addresses or
globally stable X client identities. Pointer reuse can reuse an ordinal; correlate
within an observed connection lifetime, not across reconnects. Zero means overflow
and makes the capture inconclusive. Reply and checked-request markers carry the
request sequence; analysis matches thread, connection and sequence independently.
All open spans remain in the summary rather than hiding outer teardown behind
a nested reply wait. Unmatched spans and absence of later events remain
observations, not automatic diagnoses.
