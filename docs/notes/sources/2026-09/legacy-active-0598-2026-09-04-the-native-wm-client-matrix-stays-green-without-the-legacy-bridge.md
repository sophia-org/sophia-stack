---
id: legacy-active-0598
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-09-04: the native WM client matrix stays green without the legacy bridge

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18937–18955. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

After the legacy X11 WM bridge was removed, the canonical
`tools/check_policy_client_matrix.sh` gate completed against signed Sophia
commit `ba96298c` and signed Hagia commit `a1a352bc`. The Rust client, current
handwritten C client, archived revision-3 C client, and independent Hagia Nim
client all passed the shared golden and malformed wire corpora. All eleven
behavior scenarios passed with sequential transactions, ordered actions,
timeout/stale/invalid discard and recovery, a fresh connection epoch after
restart, and preservation of the last committed projection across the
two-process restart corpus.

This result proves that retiring the compatibility bridge did not regress the
native WM policy boundary. It does not close CP-15.1 or CP-15.2: the emitted
schema-8 record correctly says `revision_freeze=false`, and this gate covers
`sophia_wm_v1`, not the complete WM, shell, and output protocol family. The
role-by-role lifecycle audit and one family-level conformance entry point
remain separate work after the active Milestone 14 comparison gate.

<!-- END IMPORTED BODY -->
