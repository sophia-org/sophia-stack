---
id: legacy-archive-0018
date: 2026-07-08
recorded_date: 2026-07-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "security"]
---
# 2026-07-08: Engine-Centered Authority Reframe

Historical source, not a current status claim. <a href="../../../history/research-log-archive-2026-09-06.txt">Original snapshot</a>,
lines 310–340. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The XLibre-centered framing is superseded. Sophia is now defined around Sophia
Engine as the permanent visual authority: physical input, scene graph, atomic
surface transactions, rendering, and scanout.

Client protocols live behind protocol authorities:

- Sophia X Authority: long-term modern X subset, inspired by Phoenix.
- Sophia Wayland Authority: future frontend for Wayland-only applications.
- Sophia Native Authority: possible future protocol for Sophia-first tools.

Authorities terminate client protocols and own protocol resources, but they do
not own layout, global shortcuts, compositor chrome, cross-namespace policy, or
scanout. They emit namespace-checked surface transactions and sanitized metadata
candidates to Sophia Engine and metadata/chrome.

Phoenix is important evidence that a clean-room, practical X server subset can
run real applications without carrying the full Xorg/XLibre object graph.
Sophia should learn from that approach while retaining its stricter process and
policy separation.

The macOS/WindowServer lesson becomes an invariant: Sophia must not present new
geometry unless it has matching committed pixels for that geometry. Slow clients
should fail closed by leaving the last committed visual state on screen rather
than exposing black borders, half-resized buffers, or torn layout.

XLibre work remains useful as prototype evidence for X11 semantics,
Xnamespace-style isolation, routed-input experiments, and XComposite/Damage
lessons, but it is no longer the target architecture.

<!-- END IMPORTED BODY -->
