---
id: legacy-active-0112
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-08-06: Lifecycle repetition is a runner, not an operator ritual

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3697–3784. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first Milestone 12 instruction asked the operator to select the ordinary
greetd entry and press the logout chord ten times. That contradicted the
existing decision that installed cycles identify distinct launches rather than
repeat broader operator choreography. It also spent human attention replaying
the same shortcut already exercised by physical and QEMU input gates.

The installed artifact now carries one automated cycle entry. After one greetd
selection, its external runner creates a fresh bounded uinput keyboard and a
fresh installed Sophia launch for each cycle. It waits for an exact schema-2,
two-output startup-ready record from the new log inode, then sends
`Super+Shift+Q` through libinput, Engine input authority, and the blind WM. A
per-cycle deadline remains outside Engine. The runner requires a new immutable
attempt, verifies it immediately, stops on the first failure, and runs the
contiguous aggregate verifier before returning to greetd.

Nested lifecycles explicitly record `handoff=cycle_runner`; ordinary,
emergency, and watchdog sessions retain `handoff=display_manager`. This avoids
claiming ten PAM or display-manager round trips when the invariant is repeated
Sophia acquisition and cleanup on one authenticated local VT. The gate still
uses physical DRM, KMS, libseat, and VT ownership. Uinput removes repetitive
human key presses without adding an Engine test mode or replacing the retained
physical keyboard evidence.

The first installed cycle entry failed before graphics takeover. The runner
itself owned TTY7, but Bash redirected the stdin of its asynchronous child to
`/dev/null`; the installed lifecycle therefore reported `vt=other` and
correctly rejected preflight. The runner now opens its controlling VT once and
passes that exact descriptor to every asynchronous installed session. Its
self-test launches a child through the production helper and requires a value
read from the preserved descriptor. Failed runner work directories now move to
a durable, private diagnostic archive instead of disappearing during cleanup.

The first descriptor-preserving rerun exposed a narrower identity error. The
runner reopened its controlling terminal through `/dev/tty`, so the child saw
that generic alias instead of `/dev/tty7`; installed preflight correctly
requires a concrete local VT. The runner now duplicates its original stdin
descriptor. This preserves both the controlling terminal and its kernel device
identity across the asynchronous launch.

The next rerun reached installed preflight on the concrete VT and exposed an
ordering omission in the automation. The production input guard still required
its independent recovery-arm chord, while the runner's virtual keyboard could
emit only the later logout chord. Sophia therefore failed closed before
graphics takeover, and the runner correctly withheld logout injection. A cycle
now uses one bounded virtual keyboard for two ordered phases: after exact guard
readiness it injects Ctrl-Alt-Backspace and requires the new guard's armed
record; after exact two-output readiness it injects Super-Shift-Q. Fresh log
identity checks prevent either phase from accepting evidence from a preceding
cycle. This exercises the production interlock instead of bypassing it.

The armed rerun then reached graphics takeover but never presented its startup
surface. Host process and file-descriptor evidence found four orphaned legacy
WM bridges from earlier releases. Three retained xmonad children, and the
bridges retained Sophia-owned DRM and physical-input descriptors. The seat
broker had duplicated libseat descriptors without close-on-exec; separately,
the bridge socket server retained its runtime and returned to `accept` after
its sole Sophia client disconnected. Those orphans kept kernel ownership alive
after the Engine process disappeared and poisoned the next startup.

Seat-device duplicates are now close-on-exec, and the cycle launcher closes
its extra retained VT descriptor before executing the installed wrapper. The
legacy bridge server owns exactly one Sophia control-client lifetime, removes
its socket on exit, and bounds the preconnection wait. Client disconnect now
drops the bridge runtime and its xmonad child instead of returning to an
unowned listener. A Rust socket-lifecycle regression requires that teardown,
and every installed cycle now rejects preexisting helpers and requires the WM
process set to drain before accepting its immutable attempt.

The first clean-host rerun exposed a separate arm-boundary race. The recovery
guard published `status=armed` when the three keys were pressed, while the
uinput producer wrote its completion receipt only after releasing them. The
runner sampled that receipt once and could reject the cycle during the roughly
30-millisecond release interval, terminating an otherwise healthy startup.
`EmergencyChordAction::Armed` now means the complete first chord has been
released. The runner also waits independently for the producer receipt, so
neither guard observation nor producer completion stands in for the other.

Signed commit `958fb5e6` then passed the complete physical gate. Installed
attempts `0014` through `0023` each reached two-output readiness in 291--336 ms,
accepted the injected normal logout through the production input and WM path,
and exited with status zero. Every recovery record preserved KD mode 0 to 0
and restored termios; no Sophia, bridge, or xmonad process survived. The
installed aggregate verifier accepted all ten contiguous immutable attempts
through `0023`, closing the repetition gate without manual repair or emergency
recovery.

<!-- END IMPORTED BODY -->
