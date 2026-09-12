#!/usr/bin/env python3
"""Direct dummy Assuan harness. No GPG, credential dumps, or raw child output."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import selectors
import signal
import subprocess
import time

STAGES = frozenset("process_start process_exit heartbeat app_update submit_enter submit_ok submit_cancel submit_escape result_send_enter result_sent close_request_enter close_enqueued run_native_enter run_native_return run_native_error window_created getpin_enter result_received assuan_data_enter assuan_data_return assuan_terminal_return paint_enter paint_return swap_enter swap_return viewport_output_enter viewport_output_return native_close_requested close_processed close_observed close_accepted event_loop_exit_requested event_loop_return".split())
SUBMIT = frozenset("submit_enter submit_ok submit_cancel submit_escape native_close_requested".split())
INSTRUCTIONS = {
    "enter": "Type the dummy word test, then press Enter.",
    "ok": "Type the dummy word test, then click OK.",
    "cancel": "Click Cancel.",
    "escape": "Press Escape.",
    "close": "Request window close using the normal window-manager close action.",
}

def parse_trace(line, pid):
    parts = line.decode("ascii").split()
    if len(parts) != 9 or parts[0] != "t082" or not re.fullmatch(r"ThreadId\(\d+\)", parts[3]):
        raise ValueError("invalid trace envelope")
    seq, named_pid, mono, wall, value, dropped = (int(parts[i]) for i in (1, 2, 4, 5, 7, 8))
    if named_pid != pid or parts[6] not in STAGES or min(seq, mono, wall, value, dropped) < 0:
        raise ValueError("invalid trace identity or stage")
    if max(seq, mono, value, dropped) > 2**64 - 1 or wall > 2**127 - 1:
        raise ValueError("trace number out of bounds")
    if parts[6] != "window_created" and value not in (0, 1):
        raise ValueError("invalid stage metadata")
    if parts[6] == "window_created" and value > 2**32 - 1:
        raise ValueError("invalid XID")
    return dict(event="trace", seq=seq, pid=pid, thread=parts[3], monotonic_ns=mono,
                unix_ns=wall, stage=parts[6], value=value, dropped=dropped)

class Protocol:
    def __init__(self):
        self.phase = "greeting"
        self.data = False
        self.dummy_matches = None
        self.terminal = None
        self.invalid = False

    def feed(self, line):
        # Values live only in this parser invocation, never in logs/errors.
        if self.phase in ("greeting", "description") and line.startswith(b"OK"):
            self.phase = "description" if self.phase == "greeting" else "response"
        elif self.phase == "response" and line.startswith(b"D ") and not self.data:
            self.data = True
            self.dummy_matches = line == b"D test"
        elif self.phase == "response" and line == b"OK" and self.data:
            self.terminal = "pin"
            self.phase = "bye"
        elif self.phase == "response" and line.startswith(b"ERR 83886179 ") and not self.data:
            self.terminal = "cancelled"
            self.phase = "bye"
        elif self.phase == "bye" and line.startswith(b"OK"):
            self.phase = "done"
        else:
            self.invalid = True

def last_open_span(events):
    opened = []
    pairs = {"paint_return": "paint_enter", "result_sent": "result_send_enter", "close_enqueued": "close_request_enter", "swap_return": "swap_enter", "viewport_output_return": "viewport_output_enter",
             "run_native_return": "run_native_enter", "assuan_data_return": "assuan_data_enter"}
    for event in sorted(events, key=lambda e: e["seq"]):
        stage = event["stage"]
        if stage in pairs.values():
            opened.append(stage)
        elif stage in pairs and pairs[stage] in opened:
            opened.reverse()
            opened.remove(pairs[stage])
            opened.reverse()
    return opened[-1] if opened else None

def run_case(command, output, case, instrumented, lifetime=60.0, submitted_timeout=15.0):
    output = Path(output)
    output.mkdir(parents=True, exist_ok=False)
    reader, writer = os.pipe2(os.O_NONBLOCK | os.O_CLOEXEC)
    env = os.environ.copy()
    env.pop("WAYLAND_DISPLAY", None)
    env.pop("WAYLAND_SOCKET", None)
    env.update(T082_DUMMY_PROBE="1", T082_TRACE_FD=str(writer))
    try:
        process = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, env=env, pass_fds=(writer,))
    except OSError:
        os.close(reader)
        os.close(writer)
        raise
    os.close(writer)
    selector = selectors.DefaultSelector()
    for fd, kind in ((reader, "trace"), (process.stdout.fileno(), "protocol"), (process.stderr.fileno(), "stderr")):
        os.set_blocking(fd, False)
        selector.register(fd, selectors.EVENT_READ, kind)
    buffers = {"trace": b"", "protocol": b""}
    protocol = Protocol()
    events, seen = [], set()
    invalid = stderr_seen = False
    cause = None
    started = time.monotonic()
    submitted_at = None
    heartbeat_at = None
    terminal_at = None
    exit_at = None
    trace_log = (output / "stages.jsonl").open("x")

    def record(event):
        trace_log.write(json.dumps(event, separators=(",", ":")) + "\n")
        trace_log.flush()

    record(dict(event="case_start", pid=process.pid, case=case, instrumented=instrumented,
                unix_ns=time.time_ns(), lifetime_seconds=lifetime, submitted_timeout_seconds=submitted_timeout))
    try:
        try:
            process.stdin.write(("SETDESC T082 DIAGNOSTIC ONLY. " + INSTRUCTIONS[case] + "\nGETPIN\nBYE\n").encode())
            process.stdin.close()
        except BrokenPipeError:
            cause = "request_pipe_closed"
        while True:
            now = time.monotonic()
            exited = process.poll() is not None
            if exited and exit_at is None:
                exit_at = now
            # Drain buffered diagnostics after exit, but never wait on a leaked fd.
            if exited and (not selector.get_map() or now - exit_at > 0.2):
                break
            if not exited and now - started >= lifetime:
                cause = "case_deadline"
                break
            if not exited and submitted_at is not None and now - submitted_at >= submitted_timeout:
                cause = "submission_deadline"
                break
            for key, _ in selector.select(0.05):
                chunk = os.read(key.fd, 4096)
                if not chunk:
                    selector.unregister(key.fd)
                    continue
                kind = key.data
                if kind == "stderr":
                    stderr_seen = True
                    continue  # Never retain arbitrary client/library messages.
                buffers[kind] += chunk
                while b"\n" in buffers[kind]:
                    line, buffers[kind] = buffers[kind].split(b"\n", 1)
                    if len(line) > 1024:
                        invalid = True
                        cause = "diagnostic_overflow"
                        break
                    if kind == "protocol":
                        protocol.feed(line)
                        if protocol.terminal is not None and terminal_at is None:
                            terminal_at = time.monotonic()
                            record(dict(event="protocol_terminal", outcome=protocol.terminal,
                                        dummy_matches=protocol.dummy_matches, unix_ns=time.time_ns()))
                    else:
                        try:
                            event = parse_trace(line, process.pid)
                            if event["seq"] in seen or len(events) >= 66_200:
                                raise ValueError("duplicate or excess trace record")
                            seen.add(event["seq"])
                            events.append(event)
                            record(event)
                            if event["stage"] in SUBMIT and submitted_at is None:
                                submitted_at = time.monotonic()
                            if event["stage"] == "heartbeat":
                                heartbeat_at = time.monotonic()
                        except (ValueError, UnicodeError):
                            invalid = True
                if len(buffers[kind]) > 1024:
                    invalid = True
                    cause = "diagnostic_overflow"
                if cause:
                    break
            if cause:
                break
    except KeyboardInterrupt:
        cause = "harness_interrupted"
    finally:
        if process.poll() is None:
            record(dict(event="harness_termination", reason=cause or "harness_interrupted", unix_ns=time.time_ns()))
            process.terminate()
            try:
                process.wait(timeout=1)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait(timeout=2)
        selector.close()
        os.close(reader)
        process.stdout.close()
        process.stderr.close()
        try:
            process.stdin.close()
        except BrokenPipeError:
            pass
        trace_log.close()
    stages = [event["stage"] for event in events]
    loss = invalid or any(e["dropped"] for e in events) or bool(seen and (min(seen) != 0 or len(seen) != max(seen) + 1)) or any(buffers.values())
    trace_complete = instrumented and not loss and "process_exit" in stages
    fresh_heartbeat = heartbeat_at is not None and time.monotonic() - heartbeat_at < 1.5
    summary = dict(case=case, pid=process.pid, instrumented=instrumented, process_exit=process.returncode,
                   harness_termination=cause, protocol_terminal=protocol.terminal,
                   protocol_complete=protocol.phase == "done" and not protocol.invalid,
                   dummy_matches=protocol.dummy_matches, stderr_present=stderr_seen,
                   trace_complete=trace_complete, trace_loss=bool(loss),
                   heartbeat_fresh_at_cleanup=fresh_heartbeat,
                   last_observed_open_span=last_open_span(events),
                   last_application_stage=next((e["stage"] for e in reversed(events) if e["stage"] != "heartbeat"), None),
                   window_ids=sorted({e["value"] for e in events if e["stage"] == "window_created"}),
                   response_after_submit_ms=None if submitted_at is None or terminal_at is None else round((terminal_at-submitted_at)*1000, 3))
    expected = protocol.terminal == ("pin" if case in ("enter", "ok") else "cancelled")
    expected = expected and (case not in ("enter", "ok") or protocol.dummy_matches is True)
    expected = expected and (not instrumented or any(summary["window_ids"]))
    summary["result"] = "completed" if (not cause and process.returncode == 0 and expected and summary["protocol_complete"] and (not instrumented or trace_complete)) else "inconclusive" if loss or (instrumented and not trace_complete and not fresh_heartbeat) else "stalled_or_failed"
    (output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    return summary

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bundle", type=Path, required=True)
    parser.add_argument("--capture", type=Path, required=True)
    args = parser.parse_args()
    identity = json.loads((args.bundle / "identity.json").read_text())
    for name, digest in identity["binaries"].items():
        if hashlib.sha256((args.bundle / name).read_bytes()).hexdigest() != digest:
            raise SystemExit("Probe binary identity mismatch")
    # The installed pinentry is never launched. This baseline is the unchanged
    # archived source built with exactly the same locked dependency versions.
    cases = [("baseline", "enter")] + [("instrumented", case) for case in INSTRUCTIONS]
    for index, (variant, case) in enumerate(cases, 1):
        print(f"T082 {index}/{len(cases)} {variant}/{case}: {INSTRUCTIONS[case]}", flush=True)
        result = run_case([str(args.bundle / ("pinentry-" + variant))], args.capture / f"{index}-{variant}-{case}", case, variant == "instrumented")
        print("T082 outcome:", result["result"], "details:", args.capture, flush=True)
        # Still run the instrumented specimen after a failed baseline, but do
        # not automatically multiply a live instrumented failure.
        if variant == "instrumented" and result["result"] != "completed":
            break

if __name__ == "__main__":
    def interrupted(_signal, _frame):
        raise KeyboardInterrupt
    signal.signal(signal.SIGTERM, interrupted)
    main()
