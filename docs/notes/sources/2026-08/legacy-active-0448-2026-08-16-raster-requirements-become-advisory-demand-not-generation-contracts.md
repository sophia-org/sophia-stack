---
id: legacy-active-0448
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "architecture"]
---
# 2026-08-16: raster requirements become advisory demand, not generation contracts

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13443–13484. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Attempt `0020` evidence closed the diagnosis. Every requirement failed with
  `cause=stale_content_generation`, never `logical_extent_mismatch`, so the
  generation check was the whole blocker. `observed_content_generation=72`
  stayed frozen while Engine's requested generation climbed 1 to 64: xterm did
  its drawing in a startup burst, and Engine then worked through committed
  generations at frame cadence. No requirement ever succeeded — the run
  contains no `variant=2`, no 750 selection, and no
  `sophia_live_surface_raster status=stale_response`, so nothing was produced
  and dropped on the return leg either.
- Root cause: both ends demanded exact generation equality between two
  counters that advance at different rates. X Authority advances a generation
  per draw; Engine commits authority transactions as a strict ordered chain
  (`visual_state.rs` requires `current == previous_committed_generation`) at
  frame cadence. A drawing client therefore leaves every requirement naming a
  generation the authority has already passed, and equality is reachable only
  if the client goes idle long enough for Engine to drain the backlog. The
  failure was total rather than intermittent for that reason.
- Decision: treat a requirement as advisory demand. The authority answers from
  current state, reports the generation the pixels were produced from rather
  than echoing the request, and anchors the reply at that generation.
  `accept_response` now refuses a reply older than the demand or naming a
  different requirement edge, and accepts one that leads it. This is safe
  because `apply_surface_raster_requirements` publishes a complete replacement
  transaction rather than amending committed content, and because replies
  travel the same ordered egress as ordinary draws, so a reply anchored ahead
  of Engine simply commits when the chain reaches it. Requiring a publication
  to match a generation Engine committed frames earlier was asking it to be
  retroactive.
- Rejected alternative: retaining per-generation journal state so the authority
  could replay a requested generation exactly. Correct, but it is real
  machinery bought to reproduce a frame the user stopped looking at several
  frames ago.
- Telemetry lesson, twice over. Collapsing generation and extent mismatch into
  one cause meant the first run could not say which check fired. Then the
  power-of-two coalescer fell silent after occurrence 64, which is exactly
  where Engine's climbing counter passed the authority's frozen one. Emission
  is now transition-sensitive — a changed cause, or the first success after a
  failing run, always emits — and satisfied requirements log at all, which
  they previously did not.

<!-- END IMPORTED BODY -->
