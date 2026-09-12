#!/usr/bin/env python3
"""Bend one rule in the indicator corpus so the independent decoder can be
shown to notice. Each mutation targets a check that exists for a reason, not an
arbitrary byte."""
import sys

MUTATIONS = {
    # A cleared active-output flag carrying a stale identity.
    "stale-active": ("empty0", 16, (9).to_bytes(8, "little")),
    # Label padding beyond the declared length must be zero.
    "label-padding": ("snapshot3", 58, b"\x41"),
    # A declared count that disagrees with the records that follow.
    "bad-count": ("snapshot0", 26, (5).to_bytes(2, "little")),
}


def main() -> int:
    name, source, target = sys.argv[1], sys.argv[2], sys.argv[3]
    frame_name, offset, replacement = MUTATIONS[name]
    out = []
    for line in open(source).read().splitlines():
        label, payload = line.split("|")
        data = bytearray.fromhex(payload)
        if label == frame_name:
            at = 24 + offset
            data[at : at + len(replacement)] = replacement
        out.append(f"{label}|{data.hex()}")
    open(target, "w").write("\n".join(out) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
