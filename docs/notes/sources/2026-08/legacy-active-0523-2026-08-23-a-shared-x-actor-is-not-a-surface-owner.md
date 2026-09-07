---
id: legacy-active-0523
date: 2026-08-23
recorded_date: 2026-08-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-23: a shared-X actor is not a surface owner

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15972–16015. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical run reached `Super+B` and committed the browser launch. Helium
opened several X connections, and one connection legally changed a surface
created by another in the classic-shared namespace. The live session had treated
the client that caused every transaction as that surface's owner. Metadata
admission detected the apparent ownership change and correctly refused it, but
the refusal ended the whole session.

The frontend route registry now records a separate passive owner fact for each
live surface. Engine-facing batches carry that fact for transaction and
presentation-intent surfaces. The batch still names its causing client for
request attribution; input, control, and metadata consumers use only the
creating client's route and admission. Duplicate live route registration and a
real owner change remain fatal. Authority-approved foreign destruction retires
the exact owner route, and session-side tombstones ignore late observations for
that nonreusable surface generation instead of resurrecting it.

A two-client classic regression creates a surface through client 1, then draws
into it through client 2 and observes client 2 as actor while client 1 and its
admission remain the owner. Client 2 can then destroy the surface without a route
failure; late actor observations cannot restore it. A route-registry regression
also proves reduced metadata candidates take the registered surface owner. The
full all-feature workspace suite passes locally. This fixes the observed crash
but does not promote the switcher row; the signed installed physical gate must
still pass.

The first signed rerun on `abd0a78c` showed only compositor chrome instead of
the Kitty guide. The session stayed alive, but the evidence made the failure
plain: the public WM snapshot grew from zero surfaces to three unmapped Kitty
helper windows, then four, and every layout request timed out waiting for those
helpers to answer toplevel resizes. No physical action had been committed.

The owner facts were correct but too broad. The frontend observation includes
passive state for every X surface touched by a request; passing every such route
to Engine made the blind public-policy filter mistake helper state for a WM
surface. Transaction batches now retain owner routes only for surfaces carrying
a transaction or presentation intent, the same presentation boundary the old
actor-derived table used. A transport regression keeps a reported helper in the
passive surface facts while excluding its route, and the two-client regression
now has the peer draw into the creator's surface to prove the owner route on a
real transaction. Both affected all-feature package suites pass. A new signed
physical rerun remains open.

<!-- END IMPORTED BODY -->
