---
id: legacy-active-0216
date: 2026-07-13
recorded_date: 2026-07-13
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11"]
---
# 2026-07-13: Explicit X Session Profiles And Map Isolation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7258–7309. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The live X launcher now selects `classic` or `confined` explicitly. Classic
remains the default shared-X group. A confined run receives a fresh registry
namespace with explicit zero portal capabilities, and those immutable facts
flow through every connection admission. The session status record exposes the
selected profile and directional capability bitsets without exposing namespace
identity.

The first simultaneous confined socket proof assigned two clients distinct
namespaces and exposed a real leak: `MapWindow` changed lifecycle state without
checking the runtime resource table. The runtime now performs namespace-aware
window lookup before mapping, so the second client receives native `BadAccess`;
classic same-namespace mapping remains valid. The following socket expansion
closes properties, selections, metadata, event selection, and routed input.

The next socket expansion found the same missing-boundary pattern in property
and selection paths. `ChangeProperty` previously keyed a foreign XID under the
requester's namespace and could emit a metadata candidate without checking the
window owner. Selection ownership and conversion likewise trusted the owner or
requestor XID instead of the admitted namespace. Runtime/dispatch now validate
all three before mutation or portal construction. The wire proof requires
`BadAccess` for foreign property and owner changes, normal
`SelectionNotify(property=None)` for foreign conversion, and zero metadata
candidates.

The final confinement expansion found that the socket bridge updated its
authority-local keyboard target from `CWEventMask` before dispatch authorization.
A rejected foreign event subscription could therefore redirect later input in
the requester's private worker to another namespace's XID. Event-target changes
now occur only after namespace validation. The drawable validator also
classifies a resource once so a foreign window's `CrossNamespaceDenied` is not
overwritten by a failed pixmap fallback. A routed simultaneous-client proof
requires native `BadAccess`, sends a broker-addressed key to the requester, and
verifies that its event target remains the local root; the broker's separate
queue regressions prove delivery stays client-specific. This completes the
bounded Milestone 1 confinement matrix; full XKB, XI2, focus, and grab semantics
remain Milestone 3 work.

The final admission-lifecycle gap was targeted supervisor revocation. Concurrent
workers now report only their session-issued `ClientAdmissionId` to frontend
supervision and retain a cloned socket solely as a disconnect handle. A
`RevokeAdmission` service command shuts down that one socket; the worker still
owns writer shutdown, private-route removal, connection-ledger cleanup, surface
removal observation, and admission-lease revocation in that order. A
pre-admission command is retained until the matching worker attaches, closing
the allocation/worker-registration race. A simultaneous classic-client
regression revokes admission 1, observes its surface removal and inaccessible
old window, then creates another window through the uninterrupted peer. This
completes the namespace/admission foundation and makes the portal broker plus
X11 clipboard the active milestone.

<!-- END IMPORTED BODY -->
