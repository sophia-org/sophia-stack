#!/usr/bin/env python3
"""Exercise real arboard teardown on a private software-only Sophia X socket."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import selectors
import shutil
import subprocess
import sys
import time
import runner

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def evaluate(events, returncode, timed_out, invalid=False):
    sequences = {e["seq"] for e in events}
    loss = invalid or any(e["dropped"] for e in events) or (
        bool(events) and (min(sequences) != 0 or len(sequences) != len(events)
                          or max(sequences) + 1 != len(events)))
    stages = {e["stage"] for e in events}
    windows = {e["value"] for e in events if e["stage"] == "clipboard_window"}
    notified = {e["value"] for e in events if e["stage"] == "clipboard_destroy_notify"}
    passed = (not loss and not timed_out and returncode == 0 and len(windows) == 1
              and windows == notified and
              {"clipboard_join_return", "clipboard_drop_complete", "process_exit"} <= stages
              and all(e["value"] == 1 for e in events if e["stage"] == "clipboard_join_return"))
    if passed:
        required = ("clipboard_window", "clipboard_destroy_notify", "clipboard_join_return",
                    "clipboard_drop_complete", "process_exit")
        positions = {stage: [e["seq"] for e in events if e["stage"] == stage] for stage in required}
        passed = all(len(seq) == 1 for seq in positions.values())
        if passed:
            passed = all(positions[a][0] < positions[b][0] for a, b in zip(required, required[1:]))
    return dict(status="PASS" if passed else "FAIL", trace_loss=bool(loss),
                timeout=timed_out, client_exit=returncode, window_ids=sorted(windows),
                notified_window_ids=sorted(notified), open_spans=runner.open_spans(events))

def stop(child):
    if child is not None and child.poll() is None:
        child.terminate()
        try:
            child.wait(timeout=1)
        except subprocess.TimeoutExpired:
            child.kill()
            child.wait(timeout=1)

def inside():
    work = Path("/tmp/work")
    Path("/tmp/.X11-unix").mkdir(mode=0o700)
    server = client = None
    reader = writer = None
    events, invalid, pending = [], False, b""
    with (work / "host.log").open("wb") as log, selectors.DefaultSelector() as selector:
        try:
            server = subprocess.Popen(["/tmp/work/host", "/tmp/.X11-unix/X99"],
                                      stdout=log, stderr=log)
            deadline = time.monotonic() + 3
            while not Path("/tmp/.X11-unix/X99").exists():
                if server.poll() is not None or time.monotonic() >= deadline:
                    raise RuntimeError("private host did not bind")
                time.sleep(.01)
            reader, writer = os.pipe2(os.O_NONBLOCK | os.O_CLOEXEC)
            env = dict(os.environ, T082_DUMMY_PROBE="1", T082_TRACE_FD=str(writer))
            client = subprocess.Popen(["/tmp/work/client"], env=env, pass_fds=(writer,),
                                      stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            os.close(writer)
            writer = None
            selector.register(reader, selectors.EVENT_READ)
            started = time.monotonic()
            deadline = started + 5
            total_bytes = 0
            while time.monotonic() < deadline:
                for key, _ in selector.select(.02):
                    data = os.read(key.fd, 4096)
                    if not data:
                        selector.unregister(key.fd)
                        continue
                    total_bytes += len(data)
                    if total_bytes > 1_048_576:
                        raise RuntimeError("diagnostic byte budget exceeded")
                    pending += data
                    while b"\n" in pending:
                        line, pending = pending.split(b"\n", 1)
                        try:
                            if len(line) > 1024:
                                raise ValueError("record too long")
                            events.append(runner.parse_trace(line, client.pid))
                        except (ValueError, UnicodeError):
                            invalid = True
                    if len(pending) > 1024:
                        raise RuntimeError("unterminated diagnostic record")
                if client.poll() is not None and not selector.get_map():
                    break
            timed_out = client.poll() is None
            host_alive = server.poll() is None
            stop(client)
            result = evaluate(events, client.returncode, timed_out, invalid or bool(pending))
            result.update(elapsed_seconds=time.monotonic() - started,
                          host_alive_before_cleanup=host_alive,
                          harness_termination="deadline" if timed_out else None)
            if not host_alive:
                result["status"] = "FAIL"
            (work / "stages.jsonl").write_text("".join(json.dumps(e) + "\n" for e in events))
            (work / "report.json").write_text(json.dumps(result, indent=2) + "\n")
            return 0 if result["status"] == "PASS" else 1
        finally:
            stop(client)
            stop(server)
            for fd in (reader, writer):
                if fd is not None:
                    os.close(fd)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--host", type=Path)
    parser.add_argument("--bundle", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--inside", action="store_true", help=argparse.SUPPRESS)
    args = parser.parse_args()
    if args.inside:
        return inside()
    if not all((args.host, args.bundle, args.output)):
        parser.error("--host, --bundle and a new --output directory are required")
    host = args.host.resolve(strict=True)
    bundle = args.bundle.resolve(strict=True)
    client = bundle / "clipboard-probe"
    identity = json.loads((bundle / "identity.json").read_text())
    if digest(client) != identity["binaries"]["clipboard-probe"]:
        raise RuntimeError("clipboard probe identity mismatch")
    bwrap = shutil.which("bwrap")
    if not bwrap:
        raise RuntimeError("bubblewrap is required; never use an operator display")
    output = args.output.resolve()
    output.mkdir(parents=True, mode=0o700, exist_ok=False)
    here = Path(__file__).resolve().parent
    for source, name in ((host, "host"), (client, "client"),
                         (here / "clipboard_check.py", "clipboard_check.py"),
                         (here / "runner.py", "runner.py")):
        shutil.copy2(source, output / name)
    env = {k: v for k, v in os.environ.items()
           if not k.startswith(("SOPHIA_", "HAGIA_", "T082_", "LD_"))
           and k not in ("DISPLAY", "WAYLAND_DISPLAY", "WAYLAND_SOCKET", "XAUTHORITY",
                         "DBUS_SESSION_BUS_ADDRESS", "PYTHONPATH", "PYTHONHOME", "PYTHONOPTIMIZE")}
    command = [bwrap, "--unshare-all", "--die-with-parent", "--new-session",
               "--ro-bind", "/", "/", "--tmpfs", "/tmp", "--proc", "/proc", "--dev", "/dev",
               "--bind", str(output), "/tmp/work", "--chdir", "/tmp/work",
               "--setenv", "DISPLAY", ":99", "--setenv", "HOME", "/tmp",
               "--setenv", "XAUTHORITY", "/tmp/absent",
               sys.executable, "-B", "/tmp/work/clipboard_check.py", "--inside"]
    with (output / "adapter.log").open("wb") as log:
        try:
            status = subprocess.run(command, env=env, stdout=log, stderr=log,
                                    timeout=15).returncode
        except subprocess.TimeoutExpired:
            status = 124
    report_path = output / "report.json"
    report = json.loads(report_path.read_text()) if report_path.exists() else {
        "status": "FAIL", "error": "No completed fixture report; inspect adapter.log"}
    if status != 0:
        report["status"] = "FAIL"
    report.update(adapter_exit=status, host_sha256=digest(host), client_sha256=digest(client),
                  fixture_sha256=digest(here / "clipboard_check.py"),
                  parser_sha256=digest(here / "runner.py"),
                  bundle_identity_sha256=digest(bundle / "identity.json"))
    report_path.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    return 0 if report["status"] == "PASS" else 1

if __name__ == "__main__":
    raise SystemExit(main())
