---
id: legacy-active-0570
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-30: damage-limited repaint is the default

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17807–17842. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`SOPHIA_ENABLE_BUFFER_AGE_DAMAGE=1` is retired as the opt-in switch; `=0` is
now the opt-out. The comment that made it opt-in named its own precondition --
a captured-pixel proof that a damage-limited render is byte-identical to a full
one -- and that proof exists. `tools/check_buffer_age_equivalence.sh` renders a
twelve-frame sequence twice on this host's GPU through a render node only, with
real partial repaints, requires the two byte-identical, and requires a lying
damage table to be caught by the same comparison. Signed native archive `0002`
then promoted the path on hardware: 129 partial repaints beside 627 full
fallbacks, 201 history records, zero invalidations.

The failure mode that justified opt-in -- a frame presentable, self-consistent,
and stale in one region -- is structurally unreachable rather than merely
untriggered. Every path that cannot prove a buffer age falls back to a full
repaint under a named reason (`UnknownBufferAge`, `NoHistory`,
`BeyondHistoryDepth`, `DamageUnavailable`, `PlanChoseFull`), and a partial write
records no history at all, so a slot whose export did not finish can never claim
an age.

That proof had zero callers. It was in no gate, no `justfile` recipe, and no
document, which is the same rot that let the pacing gate drift three schema
revisions. It now runs in `cargo xtask check`, reported rather than silently
skipped: exit 2 means this host has no writable render node and the question was
never asked, which is neither a pass nor a failure and should not read as
either.

The native gate no longer exports the variable, since the default supplies it.
Its matcher now refuses a gate that sets `=0`, so the promotion run cannot opt
out of the boundary it exists to promote while still reporting a pass.

Shared renderer workers and direct scanout stay opt-in. Each owes its own
promotion decision, and neither has the standing pixel-equivalence proof that
made this one safe to default. Making all three default at once would have put
three unproven changes behind one archive.

<!-- END IMPORTED BODY -->
