---
id: legacy-active-0194
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "security"]
---
# 2026-07-26: Clipboard Routing Is Workspace-Blind And Namespace-Explicit

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6643–6683. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical workflow reported that Ctrl-Shift-C did not work on
workspace 3. The retained session did not show a swallowed chord or lost
focus: workspace 3 kept a focused surface, six selection-owner changes and 21
conversions reached the X authority, and the session exited cleanly. Kitty did,
however, report four failed selection conversions around workspace
transitions. The normal-session verifier previously rejected ownership failure
but could still accept this conversion failure.

The socket audit found a protocol-routing defect independent of Kitty and
xmonad. A client writing a property on another client's selection requestor
received the resulting `PropertyNotify` itself. X11 requires delivery to each
client that selected `PropertyChangeMask` on that window. The routed frontend
now retains bounded per-client core event subscriptions, routes property
changes to those subscribers, removes subscriptions with their client or
window, and preserves synchronous reply/event order when the requester is also
the subscriber. This state remains inside the X frontend; Engine, workspaces,
the WM, and the compositor never receive selection objects or payloads.

The same audit tightened namespace resolution. A request first resolves an
owner in its own admitted namespace and uses ordinary X11 transfer semantics.
Only the absence of a local owner permits a cross-namespace portal request.
Owner generations are globally monotonic, and portal source capture plus
target execution revalidate the exact source namespace and generation instead
of whichever namespace changed most recently. Explicit owner clear and
disconnect cleanup also retain that namespace instead of clearing an
arbitrarily ordered owner. Policy still sees only bounded facts; the runtime
executor alone handles correlated clipboard bytes.

The same-namespace socket regression now follows Kitty's material request
shape: distinct target and property atoms, requester-side property
subscription, `AnyPropertyType`, deletion, and a maximum `long_length`. The
cross-namespace regression uses the same distinct target/property and
requester subscription while proving broker-mediated payload capture and
handoff. The strict physical verifier now rejects Kitty's conversion-failure
diagnostic as well as ownership failure and requires at least two ownership
changes plus two conversions. The operator sequence now copies workspace 1 to
workspace 3 and back before continuing the normal promotion workflow. A new
physical pass remains required before promotion.

<!-- END IMPORTED BODY -->
