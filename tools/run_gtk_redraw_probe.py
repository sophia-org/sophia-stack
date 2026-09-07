#!/usr/bin/env python3
"""Run a private, headless GTK3 redraw/remap probe against a candidate Sophia."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shlex
import signal
import subprocess
import tempfile


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    repo = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sophia", type=Path, default=repo / "target/debug/sophia")
    parser.add_argument("--wm", type=Path, required=True, help="sophia_wm_v1 WM executable")
    parser.add_argument("--output-parent", type=Path, default=Path(tempfile.gettempdir()))
    args = parser.parse_args()
    sophia, wm = args.sophia.resolve(), args.wm.resolve()
    for binary in (sophia, wm):
        if not binary.is_file() or not os.access(binary, os.X_OK):
            parser.error(f"not an executable: {binary}")
    os.umask(0o077)
    root = Path(tempfile.mkdtemp(prefix="sophia-gtk-redraw-", dir=args.output_parent)).resolve()
    print(f"evidence={root}", flush=True)
    source = repo / "tools/probes/gtk_redraw.c"
    flags = shlex.split(subprocess.check_output(
        ["pkg-config", "--cflags", "--libs", "gtk+-3.0"], text=True))
    subprocess.run(["cc", "-Wall", "-Wextra", "-Werror", str(source),
                    "-o", str(root / "probe"), *flags], check=True)
    for name in ("config", "state", "runtime", "cache"):
        (root / name).mkdir(mode=0o700)
    (root / "core.kdl").write_text(
        'schema 2\ndiagnostics verbose=#true\n'
        'session { application-catalog "probe" launch-policy="trusted-host" '
        '{ application "terminal"; }; }\n')
    (root / "desktop.kdl").write_text(
        'schema 1\npolicy { layout "scroller"; }\nshell { enabled #false; }\n'
        'shortcut { profile "gtk-redraw-probe"; }\n'
        'session { application-catalog "probe"; }\n')
    env = {key: value for key, value in os.environ.items()
           if not key.startswith("SOPHIA_") and key not in
           {"DISPLAY", "XAUTHORITY", "WAYLAND_DISPLAY", "LD_PRELOAD"}}
    env.update({f"XDG_{name.upper()}_HOME": str(root / name)
                for name in ("config", "state", "cache")})
    env.update(XDG_RUNTIME_DIR=str(root / "runtime"),
               DBUS_SESSION_BUS_ADDRESS="unix:path=/dev/null",
               LIBGL_ALWAYS_SOFTWARE="1", WINIT_UNIX_BACKEND="x11",
               GDK_BACKEND="x11")
    command = [str(sophia), "session", "run", "--no-input", "--session-mode=normal",
               "--session-start=terminal", f"--session-app=terminal={root / 'probe'}",
               "--session-action-app=terminal=terminal", "--session-app=browser=/usr/bin/true",
               "--session-action-app=browser=browser", "--max-runtime-ms=16000",
               "--wm-interface=sophia_wm_v1", f"--config={root / 'core.kdl'}",
               f"--desktop-profile={root / 'desktop.kdl'}", f"--wm-process={wm}",
               f"--display=:{41000 + os.getpid() % 10000}"]
    identity = {"sophia": str(sophia), "sophia_sha256": sha256(sophia),
                "wm": str(wm), "wm_sha256": sha256(wm),
                "probe_source_sha256": sha256(source), "command": command}
    (root / "identity.json").write_text(json.dumps(identity, indent=2) + "\n")
    with (root / "session.log").open("w") as log:
        process = subprocess.Popen(command, env=env, stdout=log,
                                   stderr=subprocess.STDOUT, start_new_session=True)
        try:
            status = process.wait(timeout=25)
        except (subprocess.TimeoutExpired, KeyboardInterrupt):
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
            status = 124
    records = (root / "session.log").read_text()
    captures = [line for line in records.splitlines() if line.startswith("gtk_redraw capture=")]
    expected = {"main", "dialog", "menu", "dialog-remap", "dialog-redraw"}
    captured = {line.split("capture=", 1)[1].split()[0] for line in captures}
    passed = (status == 0 and captured == expected and len(captures) == 5
              and all("content=pass saved=1" in line for line in captures)
              and "status=exited id=terminal source=startup exit_status=exit status: 0" in records
              and "sophia_live_session_health schema=1 status=clean protocol_errors=0" in records
              and "sophia_live_session_cleanup schema=1 status=clean" in records)
    result = {"passed": passed, "session_exit": status, "captures": captures}
    (root / "result.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
