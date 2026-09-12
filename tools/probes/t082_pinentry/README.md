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
  --output .artifacts/t082-probe-v4
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
Do not repeat the attended capture until the subsequent peer-write recovery
repair is installed and this launcher's exact pin is retargeted and verified.
The historical `18f70f86` pin below contains t077's initial recovery, but does
not contain that follow-up repair. See the linked incident note.

The capture pins installed release `0.1.0-18f70f862300`, whose commit is a
verified descendant of the t077 recovery commit `2f38ac75`. Installation on this
host is complete; no further installation or sudo is needed. An exact pin keeps
the specimen reproducible. If that release is removed, verify and deliberately
retarget the launcher instead of accepting an arbitrary newer build.

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

A normal interactive terminal and dummy prompt start automatically. Follow the
prompt's instruction, using only the public word `test`. Cases are baseline Enter,
then instrumented Enter, OK, Cancel, Escape and normal WM window-close. Each case
uses a fresh process and a direct `SETDESC/GETPIN/BYE` exchange. The harness
removes inherited Wayland endpoints so these are X11 specimens. After a failed
baseline it runs the instrumented Enter specimen; after an instrumented failure
it stops instead of automatically multiplying the failure.

Stay on the same VT during a specimen. Each process has a 60-second total limit;
instrumented submission also starts a 15-second limit. The baseline has no submit
telemetry, so only its total limit can be enforced. Later frames/heartbeats never
refresh deadlines. Timeout sends TERM, then KILL after one second if necessary,
only to the exact child created by the harness. Timeout cleanup is explicitly
recorded and must not be mistaken for normal window closure.

Verify ordinary typing and focus in the terminal. After the cases, perform the
VT round trip as a separately marked acceptance step, then use Ctrl+Alt+Delete
for normal logout. Ctrl+Alt+Backspace remains the existing independent emergency
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
