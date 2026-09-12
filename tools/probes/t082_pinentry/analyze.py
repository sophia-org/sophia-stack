#!/usr/bin/env python3
"""Report observed boundaries; never turn missing telemetry into a root cause."""
import argparse
import json
from pathlib import Path

def boundary(summary, events):
    if summary["trace_loss"]:
        return "Inconclusive: diagnostic records were lost or invalid."
    if not summary["instrumented"]:
        return "Baseline protocol outcome only; no native transition trace."
    if summary["result"] == "completed":
        return "GUI returned and the expected dummy protocol response completed."
    if not summary["trace_complete"] and not summary["heartbeat_fresh_at_cleanup"]:
        return "Inconclusive: no fresh heartbeat or complete process-exit trace."
    stages = [event["stage"] for event in sorted(events, key=lambda e: e["seq"])]
    if "run_native_error" in stages:
        return "run_native reported an error; this is not evidence of a blocked swap."
    opened = summary["last_observed_open_span"]
    if opened and opened != "run_native_enter":
        return f"Observed entry without a recorded return: {opened}. Review the last heartbeat and timeout boundary."
    if "destroy_return" in stages and "event_loop_exit_requested" not in stages:
        returned = max(e["seq"] for e in events if e["stage"] == "destroy_return")
        later = [e for e in events if e["seq"] > returned and e["stage"] == "window_event_kind"]
        return ("Teardown returned; no later window event was recorded. A buffered destroy and missing server notification remain distinct possibilities; correlate the XID and server dispatch trace."
                if not later else "Teardown returned and a later window event arrived; inspect running-state and Exit markers.")
    for earlier, later, text in (
        ("event_loop_return", "run_native_return", "Event loop returned; run_native has no recorded return (including teardown)."),
        ("event_loop_exit_requested", "event_loop_return", "Event-loop exit was requested; no event-loop return was recorded."),
        ("close_accepted", "event_loop_exit_requested", "Close was accepted; no event-loop exit request was recorded."),
        ("close_processed", "close_observed", "Close entered viewport events; no subsequent update observed it."),
        ("close_enqueued", "close_processed", "App queued Close; viewport processing has not recorded it."),
        ("result_sent", "close_enqueued", "Result-channel send returned; Close was not recorded as queued."),
    ):
        if earlier in stages and later not in stages:
            return text
    return "No decisive boundary yet; inspect the ordered stages and protocol outcome."

def report(capture):
    lines = ["# T082 dummy capture", "", "These are observed boundaries, not an automatic root-cause verdict.", ""]
    for path in sorted((capture / "cases").glob("*/summary.json")):
        summary = json.loads(path.read_text())
        records = [json.loads(line) for line in (path.parent / "stages.jsonl").read_text().splitlines()]
        events = [r for r in records if r["event"] == "trace"]
        lines += [f"## {path.parent.name}", "", f"PID {summary['pid']}; GUI XIDs {summary['window_ids']}; clipboard XIDs {summary.get('clipboard_window_ids', [])}; outcome `{summary['result']}`.", "", boundary(summary, events), "",
                  f"Protocol: `{summary['protocol_terminal']}`. Harness termination: `{summary['harness_termination']}`.", "",
                  "Last 24 application markers (heartbeat excluded; full stream remains in stages.jsonl):", "", "| Sequence | Monotonic ns | Stage |", "| --- | --- | --- |"]
        for event in [e for e in events if e["stage"] != "heartbeat"][-24:]:
            lines.append(f"| {event['seq']} | {event['monotonic_ns']} | {event['stage']} |")
        lines += ["", "Open spans (thread, opaque connection and request sequence where applicable):", "", json.dumps(summary.get("open_spans", [])), ""]
    return "\n".join(lines)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("capture", type=Path)
    print(report(parser.parse_args().capture))
