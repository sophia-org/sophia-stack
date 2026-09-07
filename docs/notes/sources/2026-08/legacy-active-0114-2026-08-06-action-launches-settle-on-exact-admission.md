---
id: legacy-active-0114
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-08-06: Action launches settle on exact admission

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3806–3824. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The repaired QEMU soak completed seven full application cycles and four
scheduled bridge recoveries. Cycle eight launched Firefox while a prior
terminal removal and bridge reseed were settling. The harness accepted the
launch action's own no-op layout and focus records as readiness, then sent the
Firefox close chord before Firefox's surface entered policy. Sophia correctly
closed the still-focused startup terminal; Firefox was admitted afterward and
the harness eventually reported a close timeout.

The action-launch barrier now waits for the session-owned schema-2 admission
record. That record is emitted only after the new surface is observed, policy
and visual admission settle, and focus is stable. Generic layout or focus
records cannot satisfy it. The same barrier covers terminal, Firefox, and
launcher churn, allowing future layout/reseed optimizations without reopening
this race. Its regression executes the same basic-grep numeric pattern as the
harness, preventing a regex-dialect mismatch from turning a valid admission
into a false timeout.

<!-- END IMPORTED BODY -->
