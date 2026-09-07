---
id: legacy-active-0449
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-16: attempt 0021 crashed on a per-frame requirement edge

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13485–13514. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Signed source `016347168e5521c7ab4a50fe6f6f411b9ed6bdaa` reached further than
  any prior attempt and then failed at runtime with `production CPU scene is
  missing 1 committed buffer(s)`. The advisory-demand change worked on the
  authority side — replies were produced for the first time — but Engine
  rejected all 70 of them as `stale_response`, and the session died.
- First defect: `reconcile` treated the committed content generation as part of
  the demand's identity, so a drawing client minted a fresh requirement edge
  every frame. A reply produced against edge R therefore always arrived after
  Engine had moved to R+1, and `accept_response` refused it on the exact
  requirement-generation match. Relaxing the content generation alone had only
  relocated the same structural mismatch onto the neighbouring field. The
  demand is now the extent and the classes; the committed generation is only
  the vantage it was observed from, so one edge stays outstanding per distinct
  demand and a single reply is in flight at a time.
- Second defect, and the fatal one: a rejected batch was discarded whole, which
  dropped its `cpu_buffer_updates` along with its transaction. The authority
  had already advanced its generation and retains the derived variants in every
  later content set, so Engine went on to commit a scene naming buffer handles
  it had never received. Refusing a demand must not desynchronize buffer
  ownership: the transaction is now declined while the buffer updates are kept.
  This path had never executed before, because prior builds never produced a
  reply to reject.
- Telemetry lesson, a third time. `report_satisfied` only logged a success that
  ended a failing run, so a requirement satisfied from the first attempt logged
  nothing at all — the run showed 70 rejections and zero evidence that the
  authority side had started working. Steady-state silence is right for
  failures and wrong for a transition that has never been observed before.

<!-- END IMPORTED BODY -->
