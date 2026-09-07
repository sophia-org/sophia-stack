---
id: legacy-active-0366
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy", "security"]
---
# 2026-08-02: physical focus transitions are cross-client X authority work

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11233–11264. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next physical Firefox run confirmed the Mirror action and every repeated
  `Super+J` focus proposal committed correctly. Firefox still could not advance
  its refocus stage because its DOM observed `FocusIn` without the preceding
  `FocusOut`. The routed frontend had stored the previously focused X window
  independently in each client writer and sent focus control only to the new
  client, so the old client could never receive its leave event.
- XLibre's `SetInputFocus`/`DoFocusEvents` path and yserver's native Rust
  crossing reducer both establish the same ownership rule: the X protocol
  authority resolves the old and new window routes and delivers both halves of
  a focus transition. Engine continues to expose only opaque focused
  `SurfaceId` state.
- The X frontend broker now retains one bounded physical-focus route. A
  cross-client change queues `FocusOut` on the old client's control queue and
  `FocusIn` on the new client's queue; a same-client change queues both events
  together so socket order is deterministic. Per-client FIFO ordering also
  preserves repeated A-to-B-to-A transitions while allowing a later optimized
  writer implementation behind the same passive transition packet.
- The same run ended cleanly with status 1 after repeated `Super+J` presses:
  an already in-flight action was correctly deduplicated by the WM owner queue,
  but the physical-input branch treated that expected bounded outcome as
  fatal. Repeated action requests now coalesce nonfatally, emit a reduced
  schema-3 record, and accumulate `action_coalesced` in the schema-2 WM
  transport summary. Capacity rejection remains distinct and bounded.
- A two-client X11 wire regression requires `FocusIn(A)`, then
  `FocusOut(A)/FocusIn(B)`, then `FocusOut(B)/FocusIn(A)`. A focused live-WM
  reducer regression locks the duplicate-to-coalesced disposition, and the
  hardware verifier fixture consumes the versioned transport summary. This
  temporary duplicate-to-coalesced policy was superseded by the ordered owner
  ingress work recorded on 2026-08-08 below.

<!-- END IMPORTED BODY -->
