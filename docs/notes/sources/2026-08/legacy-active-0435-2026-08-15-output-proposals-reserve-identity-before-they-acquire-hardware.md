---
id: legacy-active-0435
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: output proposals reserve identity before they acquire hardware

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13126–13153. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The output wire, native resolver, and topology reducer previously existed as
  three unjoined seams. Calling them independently would either publish a
  candidate before first presentation or consume a fresh `OutputId` when a
  later preparation step rejected it.
- `LiveOutputAuthorityOwner` is now the session orchestration record joining
  those seams without acquiring DRM authority. Validation resolves against a
  cloned allocator and returns an immediate validated settlement. Apply keeps
  the resolved targets, next snapshot, and allocator private while Engine
  tracks every prepared and applied opaque head.
- A candidate publishes only after every replacement logical output reports its
  first presentation. Preparation failure leaves no physical apply to undo;
  partial apply can settle only after rollback; rollback failure is terminal.
  Fresh identities become durable only with the committed snapshot, so rejected
  candidates leave both the visible topology and allocator unchanged.
- This owner deliberately performs no renderer allocation, KMS call, process
  supervision, or frontend update. Those effects remain in the live session and
  are the next cutover; keeping them out of this reducer preserves Engine's
  explicit effect boundary.
- The transport prerequisite for that supervision is now cancellable. Proposal
  bytes accumulate inside `OutputSessionTransport` until one complete bounded
  frame exists, and the optional service polls both accept and connected intake
  without blocking the visual owner. An absent client shuts down normally; a
  split header/payload remains ordered and lossless. The live supervisor still
  needs an explicit assignee and restart handoff before advertising this socket
  to a WM or shell.

<!-- END IMPORTED BODY -->
