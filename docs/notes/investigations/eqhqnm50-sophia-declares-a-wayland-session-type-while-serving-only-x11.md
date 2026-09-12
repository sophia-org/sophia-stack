---
id: eqhqnm50
date: 2026-09-11
kind: investigation
status: awaiting-physical-acceptance
tags: [investigation, session, compatibility]
---
# Sophia declares a Wayland session type while serving only X11

## Question

Chromium refused to start inside the installed Hagia session on 2026-09-11,
reporting that it was trying to open a Wayland session. Every client in the
session is told `XDG_SESSION_TYPE=wayland`, and Sophia serves clients over X11.
What sets that variable, and which component is wrong?

## Evidence

Observed while producing `chrome://gpu` and `chrome://version` dumps for the
upstream Chromium report owned by `t068`. Session `310d483b8c9c`, installed
release `0.1.0-310d483b8c9c`.

The session leader's environment carries `XDG_SESSION_TYPE=wayland`. So does
every client launched from it; a running Kitty shows `DISPLAY=:77`,
`XAUTHORITY=/run/user/1000/.sophia-Xauthority-13254-77-...` and
`XDG_SESSION_TYPE=wayland` together.

Sophia installs its session entries under `/usr/share/wayland-sessions/`:
`sophia-hagia.desktop`, `sophia-hagia-promotion.desktop`,
`sophia-firefox-proof.desktop`, `sophia-kitty.desktop`,
`sophia-layout-test.desktop`, `sophia-native-chrome-proof.desktop` and
`sophia-recovery-proof.desktop`. Nothing is installed under
`/usr/share/xsessions/`.

What the installed server actually offers clients:

| Observation | Result |
| --- | --- |
| Wayland protocol symbols in the `sophia` binary (`wl_compositor`, `wl_surface`, `xdg_wm_base`, `wayland-0`) | 0 |
| X11 symbols (`DRI3`, `Present`, `MIT-SHM`) | 232 |
| Wayland socket under `$XDG_RUNTIME_DIR` | none |
| X authority file under `$XDG_RUNTIME_DIR` | present |

Clients connect on `DISPLAY=:77` through Sophia's own X authority. This is a
static and environment observation, not a protocol capture.

## Finding and resolution

greetd is behaving correctly. The directory split is the signal a display
manager acts on: an entry under `xsessions/` yields `XDG_SESSION_TYPE=x11`, and
one under `wayland-sessions/` yields `wayland`. Sophia installs only into
`wayland-sessions/`, so greetd declares the session wayland, and that
declaration is false for every client.

Ozone-based clients read it during platform selection. Chromium selects the
Wayland backend, finds no socket, and refuses to start. Launching it with
`--ozone-platform=x11` works, which confirms the variable rather than the
display as the cause.

The likely reason for the placement is launch semantics rather than protocol
identity: an `xsessions/` entry is conventionally started by the display
manager after it starts an X server, which is wrong for Sophia because Sophia
starts its own. A `wayland-sessions/` entry is launched as a self-contained
server, which matches how Sophia runs. The session type appears to have been
inherited as a side effect of choosing that launch behaviour.

The boundary is Sophia's session installation and launcher, not greetd and not
any client. A client is entitled to believe `XDG_SESSION_TYPE`.

## Repair

`tools/installed/sophia-session` now exports `XDG_SESSION_TYPE=x11` and unsets
`WAYLAND_DISPLAY` before anything else runs. All seven session entries exec
through that launcher, so one change covers every profile. The
`wayland-sessions/` placement is kept, because its launch behaviour is the
behaviour Sophia needs; only the identity it implied is corrected.

Unsetting `WAYLAND_DISPLAY` is defensive. No display manager on this host was
observed setting it, but leaving a promise of a socket that does not exist
would reproduce the same failure by a second route.

Nothing in the stack reads `XDG_SESSION_TYPE` expecting `wayland`. The only
other writers are `tools/desktop_comparison_tty3.sh`, which sets it per
compared desktop, and `tools/benchmark_xserver_graphics.sh`, which refuses to
run under Wayland.

## Validation and remaining work

Deterministic: `tools/check_installed_session_type.sh` runs the real installed
launcher against a fixture release with a stubbed Engine and asserts the
environment handed to the session owner, which is what every client inherits.
It is registered in the `cargo xtask check` tool list. Confirmed to fail with
the fix reverted, reporting the observed `XDG_SESSION_TYPE=wayland`, and to
pass with it applied.

Physical, still open: start the installed session and launch an Ozone client
with no platform flag. It must start without `--ozone-platform=x11`. This needs
a package and install; the running session predates the repair.

This defect blocked collecting the `chrome://gpu` evidence for `t068` until
the flag was added by hand. It is unrelated to that report's VA-to-GBM device
split and must stay out of it: an environment fault would give the upstream
triager a reason to dismiss a measured client-internal failure.

## Connections

The [Brave GPU restart investigation](uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md)
owns the VA-to-GBM device split under `t068`. This note exists only because
that report needed browser diagnostics; the two failures share a client but
nothing else.

`t013` in [todo.md](../../../todo.md) owns the installed session path and its
documented start, logout and recovery behaviour, which is where the session
entry and its exported environment belong.
