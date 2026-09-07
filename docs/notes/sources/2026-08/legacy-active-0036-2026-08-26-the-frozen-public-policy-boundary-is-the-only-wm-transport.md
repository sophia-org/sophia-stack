---
id: legacy-active-0036
date: 2026-08-26
recorded_date: 2026-08-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "architecture"]
---
# 2026-08-26: the frozen public policy boundary is the only WM transport

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1189–1204. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- API v7 was removed after the retained ledger, shared restart corpus,
  immutable revision-3 client, and signed frame-fed output archive closed its
  gate. Configuration accepts only `sophia_wm_v1` and rejects `api_v7`.
- The client-hosted socket, v7 message kinds and codecs, Engine transport and
  IPC restart path, policy reload exchange, demo server/process modes, and
  their transport-specific tests are deleted. Public policy remains
  session-hosted, peer-authenticated, bounded, and fail closed.
- Engine no longer owns a workspace reducer. That model now lives inside
  `sophia-x11-wm-bridge`, where it interprets the private synthetic-X WM and is
  adapted into complete public projections. Engine retains only authoritative
  scene validation, transaction settlement, rendering, and scanout.
- The full offline all-feature workspace suite passes after removal. The next
  product step is packaging and installing a new public-policy-only candidate.

<!-- END IMPORTED BODY -->
