---
id: legacy-active-0082
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-08: Desktop status rides the layout commit

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2784–2829. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Engine cannot publish desktop status. Snapshots carry no workspace, tag, or view,
and 13.3 replaces workspaces with output projections outright. Only the policy
process knows. Left there, every window manager grows a shell-facing socket and
every shell grows a backend per window manager. Noctalia carries nine such
backends behind one interface, 12,435 lines, and that is the cost of the
alternative.

The decision is to attach an indicator descriptor to the layout proposal. Engine
commits it with the geometry and republishes it verbatim, never interpreting it.
No policy process serves a socket. That rule is now recorded in the load-bearing
ownership rules in `docs/architecture.md`.

The descriptor is deliberately not a workspace record. A scrolling policy has
columns and a kiosk has nothing, and forcing either into a workspace schema
produces a lie or a side channel. An indicator is an ordered labelled slot with
state flags and an optional action token. A shell renders slots and submits
tokens without learning what a workspace is. Noctalia's independently derived
`{id, name, coordinates, index, active, urgent, occupied}` fits inside that
without becoming the schema.

Two properties that an earlier design had to enforce now fall out of the
mechanism. A rejected proposal discards its indicators with its geometry, so no
observer reads a tag the screen never showed. Engine holds the descriptor, so
Engine clears it when the connection epoch changes and a replacement policy
cannot inherit its predecessor's published state. `ShellObservation.tla` refuted
the previous design in five steps when either explicit rule was removed; the new
design needs neither rule.

One consequence is scheduling, not design. `ProjectionBegin` must declare every
category count, so indicators require an `indicator_count` field there. Adding a
record kind is additive; adding a field to an existing message layout is not.
After 13.4 freezes `sophia_wm_v1`, this becomes a new interface family. It must
land in revision 1.

Rendering splits into tiers, which also resolves a standing contradiction between
`docs/architecture.md` and `docs/sophia-policy-ipc.md` over whether a shell is
compositor chrome or an external client. Both, at different tiers. Engine chrome
draws indicators at tier 0 and covers a status bar's whole job with no client
interface; `sophia_shell_v1` remains tier 1 for shells that need more. Tier 0
also removes the unresolved 64-KiB texture question from the critical path, since
that constraint binds tier 1 alone.

Contract and permanent bounds are in `docs/sophia-indicator-descriptor.md`.

<!-- END IMPORTED BODY -->
