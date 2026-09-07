---
id: legacy-active-0291
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-07-18: WM API v2 Foundation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9072–9093. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Milestones 7 and 8 split interactive policy enablement from daily-driver
promotion. The normative WM contract now fixes Engine ownership of physical
input, nine workspace slots, named session actions, opaque metadata, and
one-visible-workspace-per-output semantics.

The protocol carries a versioned hello, bounded binding registrations, session
descriptor, opaque action activation, workspace activation, and named session
action requests. Engine rejects unsupported capabilities, duplicate bindings,
invalid action/key values, and Ctrl-Alt-Backspace. Its shortcut registry
consumes matching press/release pairs and suppresses repeats without leaking raw
input. The native demo WM performs the startup handshake and exercises focus,
workspace, and terminal actions. Engine now owns per-seat physical modifier
state, consumes registered chords after the emergency chord check, and sends
opaque action activations through the live WM transport. A nine-slot workspace
policy validates workspace swaps, surface moves, visible focus, layout commands,
and advertised session tokens. The profiled legacy bridge registers the bundled
xmonad chord set and translates bounded workspace and named-action requests.
The full offline all-feature suite passes. Atomic delayed-commit persistence,
named-action execution, xmonad focus/layout synthesis, and QEMU remain open.

<!-- END IMPORTED BODY -->
