---
id: legacy-active-0384
date: 2026-08-03
recorded_date: 2026-08-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-03: complete the shortened physical Firefox promotion

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11656–11685. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first shortened integrated run completed all six Firefox stages and all
  six Kitty retention checkpoints around normal and WM-forced status-zero
  Firefox exits. Session health reported zero protocol errors or pending work,
  layout health was clean, and application, frontend, namespace, and
  Xauthority teardown drained completely.
- The verifier exposed two stale assumptions rather than product failures. A
  damage-idle secondary output retained its proven synchronous startup modeset
  and correctly issued no redundant asynchronous page flip; output liveness is
  now proved per output at startup while the gate separately requires at least
  one asynchronous retirement. This preserves future damage-skip optimization.
- Firefox completed real DOM wheel handling and document displacement after
  one causally ordered post-navigation packet because GTK's XI2 absolute-axis
  baseline had already been established earlier in the same device session.
  The integrated gate now requires that causal packet plus browser-observed
  DOM completion. Focused protocol gates retain the stricter fresh-baseline
  coverage, without forcing redundant operator notches in promotion.
- Super+Space moved two surfaces but resized only Firefox, so the resize epoch
  correctly matched one configured surface while the workspace projection
  retained all three managed windows. Firefox reported its DOM resize after
  receiving ConfigureNotify and presented the exact new-size pixels shortly
  afterward. The verifier now keeps those distinct causal facts instead of
  requiring three resized surfaces or demanding pixel retirement before the
  client could report receiving the configure.
- Native page-flip and X11 focus diagnostics are tracing records and therefore
  carry timestamp/level prefixes in the production log, unlike owner-loop
  proof records. The verifier fixture now models that prefix and matches the
  embedded structured marker, preventing another fixture-only anchoring bug.

<!-- END IMPORTED BODY -->
