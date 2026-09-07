---
id: legacy-milestone-0019
date: 2026-07-13
recorded_date: 2026-07-13
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-07-13 Namespace And X Admission Foundation

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 442–464.

<!-- BEGIN IMPORTED BODY -->

- [x] Added immutable namespace profiles, directional capabilities, admission
  contexts, and a session-owned generation-safe namespace registry.
- [x] Replaced the production listener-wide namespace shortcut with
  per-connection admission after cookie authentication and kernel peer
  credential checks.
- [x] Added fresh owner-only Xauthority publication, launchable classic and
  confined profiles, disjoint connection XID ranges, and cleanup-attributed
  client identities.
- [x] Proved classic shared-resource access and confined denial for resource
  lookup, properties, selections, event selection, routed input, and metadata.
- [x] Added supervisor-triggered admission revocation that disconnects one
  worker and follows the normal route, resource, surface, selection, and lease
  cleanup path without disrupting a classic peer.

The milestone exit is satisfied: production sessions allocate namespace
identity through the registry; every policy-admitted connection retains an
immutable context; disconnect and targeted supervision converge on the same
fail-closed teardown sequence.

---

<!-- END IMPORTED BODY -->
