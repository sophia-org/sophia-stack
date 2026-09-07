---
id: legacy-archive-0002
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-07-10: Live Session Terminal Bootstrap Path

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 26–79. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`docs/live-session-bootstrap.md` now records the operator path for working on a
Sophia live session without losing the development control plane: keep Codex on
an outside TTY/SSH/tmux session, run Sophia experiments on a separate TTY, and
only move Codex inside Sophia after xterm rendering and keyboard routing are
proven.

The first terminal milestone is now a separate strict probe:
`x-authority-xterm-render-smoke`. The existing `x-authority-xterm-smoke` remains
a setup/lifecycle regression. The strict render probe launches a held xterm with
text and requires at least one committed `SurfaceTransaction`.

The strict render probe now passes as the first terminal transaction proof.
The compatibility work stayed xterm-driven: it preserves `ConfigureWindow`
geometry and emits `ConfigureNotify`, accepts bounded cursor recolor, keyboard
mapping, modifier mapping, passive button grab, and RGB color allocation setup,
then reduces `ImageText8` to conservative text damage. The probe disables
xterm ANSI/dynamic color setup with `-cm -dc` so it exercises terminal drawing
rather than spending the proof window on 256-color palette initialization.

The passing reduced evidence is `outcome=proof_window_killed`, `status=-1`,
`requests=232`, `opcode_count=28`,
`opcodes=1,2,3,12,14,16,18,20,43,45,46,47,53,54,55,60,65,72,76,84,91,94,96,98,101,119,133,134`,
`transactions=4`, `runtime_committed=4`, `runtime_surfaces=4`, and
`first_error=none`. The next live-session slice should wrap this into a
persistent `sophia-live-session` launcher and route keyboard input to the
focused terminal surface.

`sophia-live-session --terminal=xterm` now exists as the first bootstrap
launcher slice. It binds an auto high local X display for one xterm render
proof, drains the observed authority transactions through deterministic live
composition/scanout, and reports the remaining gaps as `keyboard=pending` and
`persistence=single_client_probe`. It intentionally does not claim to be a
persistent interactive session yet. Explicit display binding remains pending
because low display numbers such as `:77` can still stall in xterm palette/setup
before terminal text damage.

The passing reduced evidence on `:7926` was
`status=bootstrap_ready_keyboard_pending`, `authority_requests=232`,
`authority_transactions=4`, `authority_runtime_committed=4`,
`authority_runtime_surfaces=4`, `composition_status=Passed`,
`composition_batches=4`, `composition_committed=4`, and
`composition_surfaces=4`.

The X11 Authority socket layer now separates socket binding, one-client
serving, and a persistent sequential listener. Runtime, atom, and property
tables are shared by that listener rather than being rebuilt for each accepted
connection; a regression test creates a window through one connection and maps
it through the next. This is the backend entry point required by the live
launcher, but it deliberately does not claim concurrent multi-client dispatch
or client-specific X resource-ID allocation. Those are later protocol
milestones, not requirements for the first xterm control loop.

<!-- END IMPORTED BODY -->
