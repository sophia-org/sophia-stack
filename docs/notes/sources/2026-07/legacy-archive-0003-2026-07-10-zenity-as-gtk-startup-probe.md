---
id: legacy-archive-0003
date: 2026-07-10
recorded_date: 2026-07-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11"]
---
# 2026-07-10: zenity as GTK Startup Probe

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 80–103. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`x-authority-zenity-smoke` now launches `zenity` through the CLI external-probe
harness and reaches GTK X11 startup requests with no X protocol error. The
compatibility work stayed probe-driven: zenity added bounded
`GetSelectionOwner`, `GrabServer`, `UngrabServer`, `CreateColormap`,
namespace-local `MIT-SHM` attach/detach and put-image admission, additional
minimal `RANDR` replies, minimal `XKEYBOARD` advertisement plus `UseExtension`,
and minimal `BIG-REQUESTS` advertisement plus `Enable`.

The external-probe harness no longer hard-codes host binary paths. Probe
binaries resolve through `PATH`, with `SOPHIA_XAUTHORITY_<LABEL>` overrides for
non-standard installs. GTK probes use `DISPLAY` and `GDK_BACKEND=x11` instead
of X Toolkit-style `-display` arguments.

Under `dbus-run-session`, the current TTY host reaches GTK startup with no
client-visible X protocol error. Zenity still exits before a rendered dialog
because the host session lacks working portal display state and Sophia does not
yet advertise XInput2. The reduced evidence is therefore a protocol-startup
regression, not a rendered GTK proof: `outcome=client_exited_failure`,
`requests=103`, `opcode_count=14`,
`opcodes=2,16,20,23,36,37,43,55,78,98,131,132,133,134`, `transactions=0`,
`runtime_committed=0`, `runtime_surfaces=0`, and `first_error=none`.

<!-- END IMPORTED BODY -->
