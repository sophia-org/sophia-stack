---
id: legacy-roadmap-0012
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 13.3 Replace Workspaces With Output Projections

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 664–699.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0009-milestone-13-public-policy-protocol-and-hagia.md).

<!-- BEGIN IMPORTED BODY -->

- [x] Introduce one canonical Engine reducer for complete scene snapshots and
  complete affected-output projection proposals. Validate generations,
  capability, constraints, geometry, uniqueness, one-output-per-surface, and
  visible focus before one logical commit.
- [x] Keep full snapshots and complete projections as stable semantics. Permit
  only model-equivalent chunking, coalescing, caching, or later delta encoding;
  no transport optimization may expose partial policy state.
- [x] Add the API v7-to-projection adapter and prove the dormant Rust reference
  WM and generic X11 WM bridge against the canonical reducer.
- [x] Add an explicitly selected Hagia live profile through the public
  transport and canonical reducer, with no silent API-v7 fallback.
- [x] Promote that profile to the installed native default while retaining
  Kitty, xmonad, and the previous immutable release as recovery routes.
- [x] Remove v7 and Engine-owned workspace state. Xmonad runs through the public
  compatibility adapter, the complete restart/last-layout corpus passes, and
  signed frame-fed archive `0001` has closed the final retained-ledger gate.
- [x] Preserve registered physical actions and session operations as opaque,
  capability-gated tokens. Keep raw input, executable commands, client
  metadata, protocol objects, namespaces, pixels, and renderer handles out of
  policy IPC.
- [x] Add a two-stage canonical reducer: validate a complete proposal against a
  cloned successor, preserve last-good authority, and reject promotion if its
  connection, request, scene generation, or earlier commit was superseded.
- [x] Wire staged projections through production frontend configure and
  renderable-content settlement. Emit `committed` only when authoritative
  state matches; otherwise request a fresh snapshot without silently changing
  policy geometry.
- [x] Bind the owner-only endpoint before a supervised peer starts, authorize
  its exact UID/PID afterward, and prove that ownership order through the
  independent C and Hagia conformance host.
- [x] Host the production endpoint in the Sophia session, supervise exactly one
  admitted peer, preserve the committed scene across replacement, and keep
  policy checkpoints private to that peer.

<!-- END IMPORTED BODY -->
