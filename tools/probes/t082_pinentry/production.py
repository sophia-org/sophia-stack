#!/usr/bin/env python3
"""Attended dummy-only acceptance for an explicitly identified production binary."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import signal
import select
import sys
import subprocess
import time
from production_protocol import Exchange, ProtocolFailure, encode_argument, stop

UNICODE_DUMMY = "café🔑%"
CANCELLED = 83886179

def dialog(name, command="GETPIN", action="enter", fixture="ascii"):
    return dict(id=name, command=command, action=action, fixture=fixture)

MATRIX = [
    dict(id="enter", dialogs=[dialog("enter")]),
    dict(id="ok", dialogs=[dialog("ok", action="ok")]),
    dict(id="cancel", dialogs=[dialog("cancel", action="cancel", fixture="none")]),
    dict(id="escape", dialogs=[dialog("escape", action="escape", fixture="none")]),
    dict(id="wm-close", dialogs=[dialog("wm-close", action="wm-close", fixture="none")]),
    dict(id="unicode", dialogs=[dialog("unicode", fixture="unicode")]),
    dict(id="confirm-ok", dialogs=[dialog("confirm-ok", "CONFIRM", "ok", "none")]),
    dict(id="confirm-cancel", dialogs=[dialog("confirm-cancel", "CONFIRM", "cancel", "none")]),
    dict(id="consecutive", dialogs=[dialog("consecutive-1"),
                                   dialog("consecutive-2", "CONFIRM", "ok", "none"),
                                   dialog("consecutive-3")]),
]

def instruction(spec):
    action = {"enter": "press Enter", "ok": "click OK", "cancel": "click Cancel",
              "escape": "press Escape", "wm-close": "use the normal window-manager close action"}[spec["action"]]
    if spec["command"] == "GETPIN" and spec["fixture"] != "none":
        dummy = UNICODE_DUMMY if spec["fixture"] == "unicode" else "test"
        return f"Enter only the public dummy value {dummy}, then {action}."
    return f"Without entering any secret, {action}."

def expected(spec, response):
    kind, code, data, seen = response
    if spec["action"] in ("cancel", "escape", "wm-close"):
        return kind == "error" and code == CANCELLED and not seen
    if spec["command"] == "CONFIRM":
        return kind == "ok" and not seen
    value = UNICODE_DUMMY if spec["fixture"] == "unicode" else "test"
    return kind == "ok" and seen and data == value.encode("utf-8")

def digest(path):
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()

def stage_candidate(manifest_path, destination):
    source = json.loads(manifest_path.read_text())
    if (source.get("schema") != 1 or source.get("instrumented") is not False
            or source.get("inspection_feature", False) is not False
            or not re.fullmatch("[0-9a-f]{64}", source.get("sha256", ""))
            or not re.fullmatch("[0-9a-f]{40}", source.get("source_commit", ""))):
        raise ValueError("Production candidate manifest identity is incomplete")
    binary = Path(source["binary_path"])
    if not binary.is_absolute() or not binary.is_file() or not os.access(binary, os.X_OK):
        raise ValueError("Production candidate must be an absolute executable path")
    if digest(binary) != source["sha256"]:
        raise ValueError("Production candidate checksum mismatch")
    destination.mkdir(mode=0o700, parents=True, exist_ok=False)
    candidate = destination / "pinentry-production"
    shutil.copyfile(binary, candidate)
    candidate.chmod(0o500)
    if digest(candidate) != source["sha256"]:
        raise ValueError("Production candidate changed during snapshot")
    identity = {key: source[key] for key in ("schema", "binary_path", "sha256", "source_commit", "instrumented")}
    identity["manifest_sha256"] = digest(manifest_path)
    identity["candidate_kind"] = "uninstrumented-production"
    (destination / "identity.json").write_text(json.dumps(identity, indent=2) + "\n")
    return candidate, identity

def run_case(command, output, case, observer=None, dialog_timeout=60, setup_timeout=5, exit_timeout=3, hints=False):
    output.mkdir(mode=0o700, parents=True, exist_ok=False)
    env = {k: v for k, v in os.environ.items()
           if not k.startswith(("T082_", "SOPHIA_", "HAGIA_"))
           and k not in ("WAYLAND_DISPLAY", "WAYLAND_SOCKET", "RUST_LOG", "EGUI_INSPECTION")}
    started = time.monotonic()
    child = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                             stderr=subprocess.PIPE, env=env)
    # No trace FD, injected input, GPG connection or fabricated submission clock.
    exchange = Exchange(child, started + len(case["dialogs"]) * (dialog_timeout + 6 * setup_timeout) + exit_timeout + 10)
    results, failure = [], None
    hint_probe = None
    try:
        exchange.ok(seconds=setup_timeout)
        exchange.send(b"GETINFO pid")
        kind, _, data, seen = exchange.response(setup_timeout)
        if kind != "ok" or not seen or data != str(child.pid).encode():
            raise ProtocolFailure("candidate_pid_mismatch")
        for spec in case["dialogs"]:
            commands = [
                b"RESET",
                b"SETTITLE " + encode_argument("T082 production " + spec["id"]),
                b"SETDESC " + encode_argument("DUMMY ACCEPTANCE ONLY. " + instruction(spec)),
                b"SETPROMPT " + encode_argument("Public dummy:" if spec["fixture"] != "unicode" else "Unicode café 🔑 %:"),
            ]
            for command_line in commands:
                exchange.ok(command_line, setup_timeout)
            print(f'{spec["id"]}: {instruction(spec)}', flush=True)
            requested = time.monotonic()
            exchange.send(spec["command"].encode())
            if hints:
                from production_hints import Observation
                print('Wait for Live dialog hints captured before acting (at most 60s).', flush=True)
                hint_probe = Observation(child.pid)
            response = exchange.response(dialog_timeout)
            live_hints = hint_probe.finish() if hint_probe else None
            hint_probe = None
            accepted = expected(spec, response)
            result = dict(dialog=spec["id"], requested_action=spec["action"],
                          requested_command=spec["command"], fixture=spec["fixture"],
                          protocol_result="PASS" if accepted else "FAIL",
                          response_matches=accepted, live_hints=live_hints,
                          command_to_response_ms=round((time.monotonic() - requested) * 1000, 3),
                          operator_action="not_recorded")
            # Discard raw response bytes; they never enter a result or exception.
            del response
            results.append(result)
            if not accepted:
                raise ProtocolFailure("unexpected_dialog_response")
        exchange.finish(exit_timeout)
    except ProtocolFailure as error:
        failure = str(error)
    except KeyboardInterrupt:
        failure = "operator_interrupted"
    finally:
        if hint_probe is not None:
            hint_probe.finish()
        cleanup = child.poll() is None
        stop(child)
        exchange.close()
        for stream in (child.stdin, child.stdout, child.stderr):
            try:
                stream.close()
            except BrokenPipeError:
                pass
    complete = failure is None and child.returncode == 0 and len(results) == len(case["dialogs"])
    if complete and observer is not None:
        # The candidate has exited before we ask anything. Operator response
        # time cannot leave an unbounded live child waiting for another command.
        try:
            for spec, result in zip(case["dialogs"], results):
                result["operator_action"] = observer(spec)
                if result["operator_action"] != "confirmed":
                    break
        except (KeyboardInterrupt, EOFError):
            failure = "operator_interrupted"
    attested = bool(results) and all(r["operator_action"] == "confirmed" for r in results)
    summary = dict(schema=1, candidate_kind="uninstrumented-production", case=case["id"],
                   pid=child.pid, process_exit=child.returncode, failure=failure,
                   harness_termination=cleanup, stderr_present=exchange.stderr_present,
                   protocol_result="PASS" if complete else "FAIL", dialogs=results,
                   acceptance="PASS" if complete and attested and failure is None else "NOT_ACCEPTED",
                   submission_latency_ms=None,
                   placement_evidence="not_recorded", elapsed_seconds=time.monotonic() - started)
    (output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    return summary

def ask(question):
    print(question + " [y/n/skip; 60s] ", end="", flush=True)
    if not select.select([sys.stdin], [], [], 60)[0]:
        print("not recorded")
        return "not_recorded"
    answer = sys.stdin.readline().strip().lower()
    return "confirmed" if answer == "y" else "rejected" if answer == "n" else "not_recorded"

def attest(spec):
    return ask(f'{spec["id"]}: did you use the requested action and see the dialog close normally?')

def verdicts(results):
    complete = [r["case"] for r in results] == [c["id"] for c in MATRIX]
    actions = complete and all(r["acceptance"] == "PASS" for r in results)
    placement = complete and all(r.get("placement_evidence") == "confirmed" for r in results)
    unicode = any(r["case"] == "unicode" and r.get("unicode_label_evidence") == "confirmed" for r in results)
    hints = complete and all(d.get("live_hints") and d["live_hints"]["dialog"]
        and d["live_hints"]["transient_absent"] for r in results for d in r["dialogs"])
    return dict(hint_acceptance="PASS" if hints else "NOT_ACCEPTED", action_acceptance="PASS" if actions else "NOT_ACCEPTED",
                placement_acceptance="PASS" if placement else "NOT_ACCEPTED",
                acceptance="PASS" if actions and placement and unicode and hints else "NOT_ACCEPTED")

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--candidate-manifest", type=Path, required=True)
    parser.add_argument("--capture", type=Path, required=True)
    parser.add_argument("--release-manifest", type=Path, required=True)
    args = parser.parse_args()
    if not os.isatty(0):
        parser.error("Production acceptance requires an attended terminal")
    release = dict(line.split("=", 1) for line in args.release_manifest.read_text().splitlines())
    if not re.fullmatch("[0-9a-f]{40}", release.get("commit", "")):
        raise ValueError("Release manifest has no exact commit")
    candidate, identity = stage_candidate(args.candidate_manifest, args.capture / "candidate")
    results = []
    for case in MATRIX:
        if digest(candidate) != identity["sha256"]:
            raise RuntimeError("Candidate snapshot changed before launch")
        result = run_case([str(candidate)], args.capture / "cases" / case["id"], case, attest, hints=True)
        result["placement_evidence"] = "not_recorded"
        if result["acceptance"] == "PASS":
            result["placement_evidence"] = ask(f'{case["id"]}: did every prompt appear floating, outside the tiled layout?')
            if case["id"] == "unicode":
                result["unicode_label_evidence"] = ask('Did the prompt label display café 🔑 % correctly?')
            (args.capture / "cases" / case["id"] / "summary.json").write_text(json.dumps(result, indent=2) + "\n")
        results.append(result)
        print(f'{case["id"]}: {result["protocol_result"]}; acceptance {result["acceptance"]}', flush=True)
        if result["acceptance"] != "PASS":
            break
    missing = [case["id"] for case in MATRIX[len(results):]]
    report = dict(schema=1, candidate=identity, release=release, cases=results, unexecuted_cases=missing,
                  **verdicts(results),
                  native_scope="protocol completion, live PID-matched XID Dialog/no-transient properties, and operator-attested actions/placement; t077 remains separate",
                  harness_sha256={name: digest(Path(__file__).parent / name) for name in ("production.py", "production_protocol.py", "production_hints.py")})
    (args.capture / "production-report.json").write_text(json.dumps(report, indent=2) + "\n")
    return 0 if report["acceptance"] == "PASS" else 1

if __name__ == "__main__":
    def interrupted(_signal, _frame):
        raise KeyboardInterrupt
    signal.signal(signal.SIGTERM, interrupted)
    raise SystemExit(main())
