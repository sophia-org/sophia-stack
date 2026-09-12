---
id: g8c2ey1f
date: 2026-09-12
kind: investigation
status: investigating
tags: [investigation, x11, selection, lifecycle, conformance]
---
# Peer-owned child selections outlive a disconnected parent

## Independent finding

On runtime 3f4b0462, client A creates a parent and client B creates a child
beneath it. Each owns a different selection; client C watches both. After A
closes, C receives the parent's XFixes client-close notification, but never the
child's window-destroy notification. The child still answers GetGeometry with
a successful reply after the observed parent teardown. B remains connected.

Private-socket evidence in `.artifacts/x11-xfixes-pressure-before/`:
`peer-descendant.json` records the missing notification in both byte orders;
`peer-descendant-detail.json` identifies the missing subtype;
`peer-descendant-lifecycle.json` records the successful reply where BadDrawable
was required. These are isolated frontend results, not an installed-session
observation. The independent case is `xfixes_selection_peer_descendant`.

## Cause and required repair

At 1a631234, `runtime/windows.rs::release_client_resource_range` enumerates
only the departing client's resource range and calls named-window destruction.
It does not destroy a peer-owned descendant outside that range. This is a
lifecycle omission, not merely a dropped selection event: sending the child
notification without destroying the child would announce a false transition.

Task t091 requires recursive destruction across the shared namespace while
preserving the surviving peer's unrelated resources. The parent's selection
ends because its client closed (subtype 2); the peer child's selection ends
because its window was destroyed (subtype 1). Both report owner None and retain
the original ownership timestamp. Every resulting retirement must travel with
the release under its runtime lock rather than waiting in a shared queue for
another connection to drain. Namespace admission rules remain in force.

## Acceptance

Require both byte orders to observe the two distinct causes, no duplicate
notification, missing child geometry, cleared selection owners, and a healthy
surviving peer. The case initially fails; it is not covered by the earlier
same-client descendant and ordinary owner-close passes. The bounded pressure
case remains a separate t063 obligation.

## Connections

- [todo.md](../../../todo.md): t091 belongs in the highest-priority protocol tranche.
- [Earlier destroy-family record](ksbt5d8f-the-window-destroy-family-is-incomplete-beyond-destroynotify.md):
  its closed source findings and selected t087 acceptance did not cover this
  mixed-owner disconnect case.
- [Independent conformance evidence](wzxlxbok-independent-x11-socket-conformance-exposes-missing-client-completions.md):
  the expanded manifest retains this newly exposed lifecycle obligation.
- [t063](../plans/queue-11-parallel-production-readiness.md#t063): XFixes teardown
  makes the missing lifecycle externally visible, but cannot replace it.
