---
id: legacy-active-0627
date: 2026-09-05
recorded_date: 2026-09-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security", "architecture"]
---
# 2026-09-05 — Generic scripting contract for the display server and its clients

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20199–20232. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The scripting discussion settled on a session-owned control interface that
applies to any conforming WM or shell. `docs/scripting.md` now owns the target
architecture and proposed `sophia msg` CLI. It is explicitly unimplemented:
documentation supplies neither a listener nor new protocol capabilities.
Policy clients continue to connect to Sophia and never serve scripts directly.
The session admits and authorizes callers, each authority owns command meaning,
and Engine retains visual and input authority. No reference-client vocabulary
or executable is part of the public contract.

The initial proposed scope is discovery and invocation of registered,
argument-free WM actions plus session profile reload and WM restart. Existing
role actions are reusable downstream of admission; a public control wire still
needs specification. Generic shell commands require a negotiated extension.
Parameterized setters, state queries, and event subscriptions require their
own bounded contracts and do not inherit permission from action invocation.

The security distinction is explicit: Sophia resource namespaces, OS process
protection domains, and scripting authorization enforce different boundaries.
X admission does not grant desktop control. A host-user mode would deliberately
trust reachable unconfined same-user processes, while selective delegation
requires an enforceable caller boundary. Namespace-scoped automation cannot
silently dispatch a global action. Desktop administration can affect windows
across namespaces without acquiring application-data or portal access. Caller
credentials and namespace identity remain outside the WM.

Wire layouts, authentication/grant delivery, default enablement, numeric bounds,
and command-specific completion remain implementation prerequisites. Required
evidence includes denial without effects, stale/replay handling, disconnect
ambiguity, reload/restart recovery, disclosure limits, and fair input/frame
service under overload. The roadmap retains implementation as candidate work;
the daily-driver critical path and its acceptance evidence are unchanged.

<!-- END IMPORTED BODY -->
