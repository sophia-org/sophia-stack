---
id: legacy-active-0336
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling", "architecture"]
---
# 2026-07-26: Configuration Is Two Ownership Domains, Not an Override Stack

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10581–10617. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Sophia now resolves one strict KDL 2 source for session/Engine mechanism and a
separate source for Sophia-native WM policy. The user defaults are
`${XDG_CONFIG_HOME:-$HOME/.config}/sophia/config.kdl` and `wm.kdl`;
`/etc/sophia` and compiled snapshots are ordered fallbacks. There is no
include graph, field merge, KDL 1 fallback, or WM override of Engine policy.
External WMs continue to use their native configuration.

The new `sophia-config` crate contains only bounded passive schema data,
discovery, parsing, snapshots, deltas, last-known-good state, and a
parent-directory inotify source. Atomic editor replacement is therefore
observable without watching a stale inode. Unsafe ownership/mode, files over
one MiB, unknown or duplicate fields, broken references, invalid paths, and
the emergency chord fail closed.

Core candidates apply as one transaction. Application registry, repeat,
fallback chrome, and diagnostic changes are live-safe; mechanism changes mark
the complete candidate pending restart and do not leak its live-safe subset.
The session owner waits for an idle key ledger before replacement. Renderer
entry points now consume the Engine border style stored by the visual runtime
instead of recreating a default at each composition call.

The WM API advances to version 5. Negotiation carries a nonzero policy
generation and bounded chrome preference, while Engine continues to own
geometry, damage, rendering, and scanout. Generation-ordered update/ack
packets and an idle-shortcut reducer establish the hot-update contract.

The supervised transport now completes that contract. The socket worker
forwards immutable unsolicited candidates to the Engine owner and returns the
owner's exact-generation acknowledgement; it never applies policy itself. The
WM suspends new-policy request service until acknowledgement. Both ends also
handle the race where a bounded Engine request is already in the socket: the
worker accepts the intervening policy frame, the WM holds the request, and the
response follows the applied acknowledgement. Socket integration coverage
exercises atomic file replacement, generation delivery, request deferral, and
acknowledgement ordering.
<!-- END IMPORTED BODY -->
