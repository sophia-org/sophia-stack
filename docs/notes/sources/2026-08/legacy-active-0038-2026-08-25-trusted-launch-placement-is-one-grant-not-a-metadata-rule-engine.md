---
id: legacy-active-0038
date: 2026-08-25
recorded_date: 2026-08-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "security"]
---
# 2026-08-25: trusted launch placement is one grant, not a metadata rule engine

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1238–1260. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- A registered application's optional nonzero `placement-class` enters the
  session launch intent. Only the first new surface observed while that launch
  owns admission receives it; later dialogs or additional toplevels receive no
  implicit placement. CLI-added executables carry no class, so the authority is
  available only to the trusted core registry.
- Capability bit 10 gates snapshot extension kind `0xFF00`. Its 16-byte records
  contain only the generational surface handle and opaque `u64` class. Frozen
  begin/end counts still name the ordinary chunk prefix; extensions append with
  dense ordinals before `SnapshotEnd`. The assembler refuses the kind without
  negotiation, and a producer pin proves gated-off output is byte-identical.
- The public owner retains the grant through stale, invalid, timeout,
  disconnect, and supervised restart paths. It filters withdrawn surfaces and
  consumes a grant only after the matching Manage projection commits. No title,
  app ID, PID, executable path, namespace, or match expression is available to
  policy.
- Hagia requests the capability and maps the retained classes 1..9 to view
  slots on the active output without switching the active view. Unknown classes
  remain advisory. Its independent uncounted-extension socket fixture, adapter
  one-shot test, `nph`, and full serial `nimble test` pass; Sophia's config,
  protocol, runtime, and CLI all-features suites pass as well.

<!-- END IMPORTED BODY -->
