---
id: legacy-active-0064
date: 2026-08-12
recorded_date: 2026-08-12
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-12: The reservation shrink edge is fixed at the state layer, not the reducer

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1908–1942. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Decision on the fail-open edge recorded yesterday: a mode change that leaves a
  root-relative reservation outside the new root currently releases it, so policy
  briefly sees the full output while a bar still occupies pixels. Two obvious
  answers were considered and both are wrong.
- **Fail closed by preserving is actively wrong.** A reduction returning `None`
  makes the caller keep the previous work area
  (`live_session/wm/work_area.rs`, `status=preserved reason=invalid_reduction`).
  That is safe when a malformed reservation arrives against unchanged geometry, and
  unsafe after a shrink: the preserved rectangle belongs to the *larger* output, so
  policy would lay out beyond the screen. Releasing the reservation is merely
  suboptimal; preserving a stale work area is incoherent. The existing fail-closed
  path is therefore not the fix, and reaching for it because the codebase is
  fail-closed elsewhere would have made things worse.
- **Clamping inside the pure reducer is also wrong**, though it looked right. A top
  bar of depth 40 is still a top bar after the output shrinks, and only its span was
  expressed against the old root, so clamping the span preserves the meaning. But
  the reducer cannot tell a span clamped by a shrink from a span that arrived
  malformed: `malformed_or_out_of_root_reservations_do_not_change_work_area` pins
  that a span starting at `-1` is rejected, and clamping would make that
  off-by-one publication take effect. Fixing a fail-open edge by weakening malformed
  rejection is a bad trade.
- **Decision: the fix belongs in `SurfaceOutputReservationState`**, which is the only
  layer holding what the reducer lacks — the reservations it previously admitted,
  and the root they were valid against. A reservation already admitted against a
  larger root is re-projected onto the smaller one when the root shrinks; a
  reservation arriving for the first time is validated against the current root and
  rejected if it does not fit. Same geometry, different provenance, different
  answer — which is exactly the distinction the pure function cannot make and this
  one can.
- Not implemented in this pass. It changes behavior for every bar, and the analysis
  that rules out the two shortcuts is worth more than a rushed version of the third.
  Current behavior stays pinned by a test that names it fail-open.

<!-- END IMPORTED BODY -->
