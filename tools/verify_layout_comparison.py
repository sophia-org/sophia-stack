#!/usr/bin/env python3
"""Verify one retired comparison and the probe's matching Copy completion.

Inputs must belong to one bounded run on an identified candidate session. This
checks the evidence join, not session identity, capture health or task completion.
"""

import argparse
import json
from pathlib import Path
import re
import sys


MAX_BYTES = 64 * 1024 * 1024
MAX_LINE = 8192
MAX_RECORDS = 100_000
U64_MAX = (1 << 64) - 1
IMPLICIT = (1 << 56) - 1
FORMATS = {0x34325258, 0x34325241}
PREFIX = re.compile(
    r"(?:^|\s)(sophia_live_layout_probe|sophia_live_session_present_feedback|dri3_layout)\s"
)


class EvidenceError(ValueError):
    pass


def read_lines(path):
    total = 0
    with Path(path).open(encoding="utf-8") as stream:
        while line := stream.readline(MAX_LINE + 1):
            total += len(line.encode("utf-8"))
            if len(line) > MAX_LINE or total > MAX_BYTES:
                raise EvidenceError("evidence exceeds the bounded run size")
            yield line


def records(lines):
    result = []
    for count, line in enumerate(lines, 1):
        if count > MAX_RECORDS or len(line) > MAX_LINE:
            raise EvidenceError("evidence exceeds the bounded record count")
        match = PREFIX.search(line)
        if not match:
            continue
        fields = {"record": match[1]}
        for key, value in re.findall(r"([a-z_]+)=([^\s]+)", line[match.end():]):
            if key in fields:
                raise EvidenceError("duplicate field in evidence record")
            fields[key] = value
        result.append(fields)
    return result


def number(record, field, minimum=0, maximum=U64_MAX):
    value = record.get(field, "")
    if not re.fullmatch(r"[0-9]{1,20}", value):
        raise EvidenceError(f"missing or invalid {field}")
    parsed = int(value)
    if not minimum <= parsed <= maximum:
        raise EvidenceError(f"out-of-range {field}")
    return parsed


def one(candidates, name):
    if len(candidates) != 1:
        raise EvidenceError(f"expected one {name}, found {len(candidates)}")
    return candidates[0]


def layout(record):
    values = (
        number(record, "output", 1),
        number(record, "format", 1, (1 << 32) - 1),
        number(record, "original_modifier"),
        number(record, "alternative_modifier"),
    )
    if values[1] not in FORMATS or values[2] == values[3]:
        raise EvidenceError("comparison lacks distinct same-format layouts")
    if any(value in (IMPLICIT, U64_MAX) for value in values[2:]):
        raise EvidenceError("implicit layout cannot establish an explicit pair")
    return values


def verify(session_lines, probe_lines, transaction=None, expected_mode="copy"):
    if expected_mode not in ("copy", "suboptimal"):
        raise EvidenceError("unsupported expected completion mode")
    session = records(session_lines)
    probe = records(probe_lines)
    comparisons = [r for r in session if r["record"] == "sophia_live_layout_probe"]
    matched = one([
        r for r in comparisons
        if r.get("status") == "PreferenceMatched"
        and (transaction is None or number(r, "transaction", 1) == transaction)
    ], "matched transaction (select --transaction for a longer run)")
    if number(matched, "schema") != 2:
        raise EvidenceError("unsupported match schema")
    transaction = number(matched, "transaction", 1)
    pair = layout(matched)
    context = number(matched, "native_generation", 1)
    preference = number(matched, "preference_generation", 1)
    retired = one([
        r for r in comparisons
        if r.get("status") == "RetiredCopy"
        and number(r, "transaction", 1) == transaction
    ], "retired copied transaction")
    if (number(retired, "schema") != 2 or layout(retired) != pair
            or number(retired, "native_generation", 1) != context):
        raise EvidenceError("retirement and current comparison disagree")
    source = number(retired, "source_image", 1)
    scene = number(retired, "scene_generation", 1)
    tested = one([
        r for r in comparisons
        if r.get("status") == "Tested"
        and number(r, "source_image", 1) == source
        and layout(r) == pair
    ], "paired test for the exact source (repeated attempts are ambiguous)")
    schema = number(tested, "schema")
    if schema == 1 and tested.get("original_stage", "Atomic") == "Atomic":
        original_stage = "Atomic"
    elif schema == 3 and tested.get("original_stage") in ("Atomic", "Framebuffer"):
        original_stage = tested["original_stage"]
    else:
        raise EvidenceError("unsupported paired-test schema or original stage")
    if (number(tested, "scene_generation", 1) != scene
            or tested.get("original_status") != "Rejected"
            or tested.get("original_errno") != "22"
            or tested.get("alternative_status") != "Submitted"
            or tested.get("alternative_errno") != "none"):
        raise EvidenceError("missing fresh rejected-original/passing-alternative test")

    # Different diagnostic streams may be captured out of order. Join owned
    # identities, never proximity in the log or a shared scene generation alone.
    feedback = one([
        r for r in session
        if r["record"] == "sophia_live_session_present_feedback"
        and r.get("kind") == "complete"
        and number(r, "transaction", 1) == transaction
    ], "routed Complete")
    if number(feedback, "schema") != 1 or feedback.get("routed") != "true":
        raise EvidenceError("comparison did not route Complete")
    ust, msc = number(feedback, "ust", 1), number(feedback, "msc", 1)
    one([
        r for r in session
        if r["record"] == "sophia_live_session_present_feedback"
        and r.get("kind") == "complete" and r.get("routed") == "true"
        and number(r, "ust", 1) == ust and number(r, "msc", 1) == msc
    ], "server completion at the probe clock (shared clocks are ambiguous)")
    copied = one([
        r for r in probe
        if r["record"] == "dri3_layout" and r.get("event") == "complete"
        and number(r, "ust", 1) == ust and number(r, "msc", 1) == msc
    ], "probe Copy completion at the routed clock")
    if copied.get("mode") != expected_mode:
        raise EvidenceError("probe did not receive the required completion mode")
    server_mode = "SuboptimalCopy" if expected_mode == "suboptimal" else "Copy"
    if (feedback.get("mode", server_mode if expected_mode == "copy" else None)
            != server_mode):
        raise EvidenceError("server and client completion modes disagree")
    serial = number(copied, "serial", 0, (1 << 32) - 1)
    submitted = one([
        r for r in probe
        if r["record"] == "dri3_layout" and r.get("event") == "submit"
        and number(r, "serial", 0, (1 << 32) - 1) == serial
    ], "probe submission for the completed serial")
    if (number(submitted, "pixmap", 1) != number(copied, "pixmap", 1)
            or number(submitted, "format") != pair[1]
            or number(submitted, "modifier") != pair[2]):
        raise EvidenceError("probe submission differs from the tested original")
    buffer = number(submitted, "buffer", 0, 1)
    if expected_mode == "suboptimal" and number(submitted, "suboptimal", 0, 1) != 1:
        raise EvidenceError("probe did not opt into reallocation advice")
    allocated = one([
        r for r in probe
        if r["record"] == "dri3_layout" and r.get("event") == "allocation"
        and number(r, "buffer", 0, 1) == buffer
    ], "actual allocation for the submitted buffer")
    if number(allocated, "format") != pair[1] or number(allocated, "modifier") != pair[2]:
        raise EvidenceError("probe allocation differs from submitted metadata")
    finished = one([
        r for r in probe
        if r["record"] == "dri3_layout" and r.get("event") == "finished"
    ], "completed probe run")
    if finished.get("result") != "pass":
        raise EvidenceError("probe run did not finish successfully")
    return {
        "evidence": "retired_preference_comparison",
        "original_stage": original_stage,
        "completion_mode": expected_mode,
        "transaction": transaction, "source_image": source, "output": pair[0],
        "format": pair[1], "original_modifier": pair[2],
        "alternative_modifier": pair[3], "native_generation": context,
        "preference_generation": preference, "probe_serial": serial,
        "ust": ust, "msc": msc,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--session-log", required=True, type=Path)
    parser.add_argument("--probe-log", required=True, type=Path)
    parser.add_argument("--transaction", type=int)
    parser.add_argument("--expect-mode", choices=("copy", "suboptimal"), default="copy")
    args = parser.parse_args()
    try:
        result = verify(
            read_lines(args.session_log), read_lines(args.probe_log),
            args.transaction, args.expect_mode,
        )
    except (EvidenceError, OSError, UnicodeError) as error:
        print(f"layout comparison not verified: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
