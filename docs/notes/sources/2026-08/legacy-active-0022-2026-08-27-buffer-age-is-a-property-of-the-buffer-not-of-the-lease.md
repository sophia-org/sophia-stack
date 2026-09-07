---
id: legacy-active-0022
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-27: buffer age is a property of the buffer, not of the lease

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 746–785. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Milestone 14's next step opens at the model boundary, as the previous one did.
  `VisualDamageHistory.tla` abstracts the output as a region partition and
  content as a generation mark per region, never pixels. A scene generation
  damages regions whether or not it is ever rendered, so a deferred generation
  that is superseded still contributes the work a later slot write owes. The
  property is that a slot brought up to the current scene holds what a full
  repaint would have produced, stated once over the result and once over the
  damage so a counterexample names the region that was owed rather than the
  frame that was wrong.
- The first configuration carried the lease incarnation into history and did not
  finish: 14 million distinct states at depth 12 with the queue still growing.
  Removing it was not only a state-space decision. A slot's buffer keeps its
  content across release and reacquisition, and that persistence is exactly what
  makes buffer age worth anything, so history dies with the bundle or with an
  incomplete write rather than with the lease. `VisualRetirementSlots` already
  owns the lease identity that rejects a stale release. The reduced
  configuration checks 3,643,747 generated and 415,585 distinct states to depth
  12 in six seconds.
- Two negative controls confirm the model is load-bearing. Narrowing a partial
  write to the current generation's damage alone violated
  `RepaintMatchesFullRepaint` at depth 6; letting a rebuilt bundle keep the
  generation its lost pixels were written for violated it at depth 4. A third
  control checked `PartialWriteIsReachable` and found it violated at depth 4,
  which is the answer to the question a safety-only model cannot ask: the
  optimization is admissible here, not vacuously safe. The first attempt at the
  rebuild control passed, because the mutation also cleared the recorded
  generation and a slot claiming generation zero already owes the whole output;
  a control that passes is a control that tested nothing.
- One finding belongs to the implementation and not the model. A slot's frame
  surface is an EGL/GBM window surface driven by `eglSwapBuffers`, so the slot
  does not own one buffer: the surface rotates through its own set, and a later
  render into the same slot may receive a buffer two or three swaps old. Keying
  history by slot alone therefore under-computes damage in the normal case.
  Either query `EGL_BUFFER_AGE_EXT` per acquired back buffer, or stop swapping
  and manage each slot's buffer explicitly so slot identity and buffer identity
  coincide. The model says what a repaint owes given some content age; whichever
  mechanism supplies that age has to be right about it.

<!-- END IMPORTED BODY -->
