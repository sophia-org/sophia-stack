---
id: legacy-active-0044
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "tooling"]
---
# 2026-08-16: mixed topology preparation belongs to the session owner

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1399–1435. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first three-head mixed-output run reached two admitted Kitty surfaces but
  failed before any modeset. Two ordinary scene advances made the bundled Rust
  reference receive `RejectedStale`; it treated that expected generation race
  as a process error and was restarted twice. Unlike Hagia and the xmonad
  bridge, this reference is a pure snapshot-to-projection function and mutates
  no private model. It now returns to fresh snapshot intake on the same
  connection only for `RejectedStale`; all other noncommitted outcomes remain
  fatal, and stateful peers retain the restart rule recorded on August 5.
- The fatal third attempt exposed a separate Engine/backend contract mismatch.
  The canonical display list may name a policy-admitted surface before it has a
  committed pixel state, and Engine's output snapshot deliberately filters that
  command. Retained native composition instead required every such command to
  be committed and reported `retained head plan lost a displayed surface`.
  Retained source membership is now the intersection of policy presentation
  order and Engine's committed surface table. A committed displayed surface
  without an authority-owned CPU or renderer source still fails closed.
- Output preparation had also been delegated backward across the authority
  boundary. The IPC client submitted up to sixteen distinct transactions while
  the backend reported that ordinary frames still owned scanout resources.
  Only the session owner can observe and advance those resources. It now accepts
  one effect, quarantines new ordinary authority and policy intake at existing
  bounded queues, services native retirement, and begins renderer preparation
  only after a backend-owned quiescence predicate becomes true. The wait is
  bounded at two seconds; cancellation takes precedence, and timeout rejects
  without KMS mutation. The output client's single proposal has an eight-second
  socket deadline.
- The old evidence condition required active DP-2 content in the first topology
  frame. That ordering was impossible: blind policy sees three logical outputs
  before publication and can partition the two proof surfaces only after the
  mirror plus singleton becomes two logical outputs. The first DP-2 frame may
  therefore be empty, but must already use exact mapping with no sampling or
  fallback. Visual acceptance now requires a later exact active DP-2 plan after
  the `placement=1,1` marker, joined causally through frame queue, KMS submit,
  callback, and retirement.

<!-- END IMPORTED BODY -->
