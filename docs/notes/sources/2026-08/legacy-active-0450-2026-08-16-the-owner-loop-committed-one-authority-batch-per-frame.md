---
id: legacy-active-0450
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "security"]
---
# 2026-08-16: the owner loop committed one authority batch per frame

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13515–13557. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Attempt `0022` on `f0ab320d` cleared every raster-path defect: zero sampled
  fallbacks, zero stale responses, no crash, and DP-2 selected an exact
  750-density variant with a real second content variant. It still failed
  visual confirmation, and the reason was cadence rather than density. The 750
  variant appeared at committed generations 71 and 72 — the last two frames of
  the run. Everything before it selected the 1000-density handle downsampled.
- Diagnosis: the live owner loop popped exactly one authority batch per
  `'session` iteration and ran one production cycle for it, so xterm's 72-draw
  startup burst needed 72 frames to reach the screen. Engine never imposed
  that: `commit_authority_batches` maps over a whole slice before one compose,
  and `rebase_authority_groups_to_committed` chains
  `previous_committed_generation` across a vector precisely so a same-surface
  run applies in one cycle. The constraint was the loop shape alone. The raster
  reply, anchored at the generation the burst ended on, simply waited its turn
  behind the backlog — correct, and 70 frames late.
- `defer_cpu_frame` did not help: it only skips rasterization, never merges
  commits, and its one-deep lookahead needs the queue already populated, which
  the drain — running only in the `Ok` arm and stopping at the first empty poll
  — frequently failed to provide.
- Decision: one owner iteration now drains and commits every immediately
  available coalescable batch in a single cycle. Two guards keep it fail
  closed. Only batches carrying pure client content may join, so Present work,
  removals, resource traffic, presentation intents, and output reservations
  keep their own cycle in order. And a run is admitted only while the admission
  pipeline is quiescent, because `projected_batch` drains the released-group
  queue: a release landing between two projections of one cycle would emit a
  quarantined group twice. A repeated transaction identity also ends a run,
  since production groups are bucketed per projection call. A raster response
  may open a run but never join one, because merging accepts every response
  before any commit.
- Method note worth keeping: `live_session` is behind the
  `atomic-scanout-live` feature, so a plain `cargo test` compiles none of it.
  Several verification passes in this session reported green without ever
  building the code under change. Feature-gated crates need
  `-p sophia-cli --features atomic-scanout-live`, which `tools/
  check_atomic_scanout_local.sh` already does for its own targets.
- Telemetry: `report_satisfied` reported only recoveries, so a density path
  that worked from the outset logged nothing — attempt `0022` showed 70
  rejections and no evidence the authority had started answering. The first
  satisfied requirement per surface now always reports.

<!-- END IMPORTED BODY -->
