---
id: legacy-active-0014
date: 2026-09-05
recorded_date: 2026-09-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security", "architecture"]
---
# 2026-09-05: scripting control v1 gets an independent wire contract

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 479–515. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Specified experimental `sophia_control_v1` major 1/revision 1 with the
  existing 24-byte LE envelope, kinds 128–134, complete bounded catalogs,
  owner-qualified argument-free invocation, and structured terminal outcomes.
  The session owns the endpoint and authorization; spatial meaning stays in
  the WM, and Engine remains the visual commit authority. WM/shell role wire
  layouts are unchanged. [The specification](../../../sophia-control-v1.md#design-sources)
  records lessons from i3/Sway, Niri, QMP, and D-Bus.
- Resolved access to disabled-by-default, explicitly enabled host administration;
  OS-confined/protected role callers are excluded and unverifiable host-domain
  membership must fail closed. A matching UID or Sophia resource namespace
  does not establish that domain. Descriptor forwarding/inheritance requires
  OS enforcement; this wire creates no confined delegation mechanism.
- Resolved concurrency to one outstanding request per reusable connection,
  monotonically increasing connection-local IDs, and exact catalog mapping
  generations. Success requires owner settlement, not queuing or physical
  presentation retirement. Expected replacement inside restart/reload is
  session-correlated; uncertain post-dispatch effects are indeterminate and
  must not be automatically replayed.
- Added schema-derived tables and valid/malformed frames to the existing
  generator plus an independent stdlib Python client and offline checks.
  `tools/check_control_protocol.sh` checks artifact freshness, framing,
  decoding, fragmentation, catalog limits, correlation, failure handling, and
  client non-replay. These checks implement no server admission or owner
  settlement. Runtime security, reload recovery, pressure/fairness, and physical
  acceptance remain explicitly listed implementation gates. No endpoint,
  parser option, installed CLI, or live session was enabled by this work.
- Validation: the control check passes 20 offline tests; workspace formatting,
  metadata, and local link/whitespace checks pass. The full workspace test run
  stops on four existing `LayerSnapshot` test initializers missing `output`
  in `crates/sophia-protocol/tests/protocol/data_model.rs` (226, 280, 359, 419).
  The source-layout audit also reports existing overlength/inline-test
  violations outside this change. The shared generator now avoids redundant
  u32 reserved-field casts, matching the already checked-in WM codec without
  changing that codec or any WM/shell golden bytes.

<!-- END IMPORTED BODY -->
