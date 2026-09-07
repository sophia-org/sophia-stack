---
id: legacy-active-0198
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-25: Frame Service Is An Output-Scoped Engine Reduction

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6757–6789. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The status-bar work-area run completed 162 mixed exports with balanced target,
pipeline, and frame-surface counts, but the native service still selected work
from aggregate booleans such as “some output has retirement” and “some output
has a pending frame.” That representation discarded output identity and made
fairness, primary reservation, callback stalls, and exact effect ordering
implicit in backend control flow.

The local Niri source reinforces two reusable boundaries without changing
Sophia's product architecture: frame state belongs to each output, and
scheduling policy should be testable independently from renderer and KMS
execution. Sophia does not adopt Niri's Wayland state graph, compositor
authority, protocol objects, or damage implementation. Damage history and
leased buffer pools remain deferred until the post-soak efficiency milestone.

Engine now reduces a bounded immutable observation for every output into named
effects: poll one output's retirement, submit the queued primary presentation,
or submit one output's pending frame. It validates a unique stable output set
and exactly one stable primary, orders retirement before new submission,
reserves a queued primary presentation without starving ready secondary
outputs, reobserves backend state after every effect, never reissues an effect
that failed to advance within the pass, and fails closed at a derived effect
budget. Backend-live maps those effects to mechanism and contains no runtime
selection policy.

Deterministic Engine tests cover idle, mixed retirement/presentation/pending
ordering, secondary fairness, primary reservation, stalled observation,
invalid identity sets, mutation during a pass, and budget exhaustion. The
physical promotion boundary remains unchanged: the focused xmobar, four-Kitty,
and normal xmonad gates must pass from this same lifecycle commit before the
architecture is promoted.

<!-- END IMPORTED BODY -->
