---
id: psf52z1x
date: 2026-09-12
kind: investigation
status: investigating
tags: [investigation, x11, containment, conformance]
---
# A stalled protocol recipient can escape X11 client containment

## Source finding

At 3f4b0462, two older peer-delivery paths in
`crates/sophia-x-authority/src/x11_socket/connection/dispatch.rs` convert every
`route_protocol` error into an unclassified `X11SetupSocketError::new`:
peer Present NotifyMSC delivery and DestroyNotify delivery during client release.
A recipient queue failure can therefore escape the client worker through the
same service-reaping boundary implicated by t089. The sender or departing owner
is not necessarily the failed recipient.

This is a source finding, not a reproduced queue-pressure incident. The ordinary
destroy and peer-close cases pass in the 94-execution independent gate; they do
not fill a recipient queue. No live display was contacted to investigate this.

## Required repair and proof

Task t090 owns this separate routing-containment gap. Reproduce it using bounded
private fixtures, then require the affected recipient to be disconnected on
queue exhaustion while healthy senders, recipients and new admissions continue.
An already departed recipient must not make its sender or the service fail.
Do not silently drop mandatory events for a client that remains connected.
Keep poisoned shared state and other authority failures fatal.

Cover both named delivery paths and retain a negative control that makes the
fixture fail when recipient containment is removed. Check that teardown still
retires subscriptions and resources even when a recipient has gone. The current
94-case baseline is insufficient to close this task.

## Connections

- [todo.md](../../../todo.md): t090 is in the highest-priority protocol tranche.
- [Setup-disconnect incident](kwhei4x4-preflight-setup-disconnect-precedes-an-authority-exit.md):
  t089 covers setup/request failure classification and installed acceptance;
  it does not certify every asynchronous recipient path.
- [Independent conformance evidence](wzxlxbok-independent-x11-socket-conformance-exposes-missing-client-completions.md):
  ordinary destroy delivery passes; queue-pressure coverage remains separate.
- [t063](../plans/queue-11-parallel-production-readiness.md#t063): the new XFixes
  path must disconnect a stalled watcher rather than silently dropping its
  events. That work stays with the XFixes repair, not this older-path task.
