---
id: legacy-active-0399
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-05: Stale WM replies require a fresh speculative peer

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12108–12136. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The external xmonad bridge applies each request to its private model before
  replying. Sophia formerly rejected a response when one of its fingerprinted
  surfaces disappeared, then immediately sent the next queued request to that
  already-mutated peer. A queued removal could therefore be planned against a
  synthetic surface that no longer existed and terminate the session with
  `UnknownSurface`.
- The committed workspace state could also retain a removed surface when the
  pending removal request was discarded during restart. Response-lifetime
  reconciliation now removes only fingerprinted surfaces that vanished from
  the Engine-owned persistent layout. A pending `ManageSurface` remains absent
  from committed state; an already-committed removal is deleted before reseed.
- A stale response now requests transport restart and suppresses queue pumping.
  The owner terminates the speculative peer, clears its in-flight and queued
  protocol work, starts a fresh bridge, and reseeds it from reconciled committed
  state. This is distinct from a later Engine proposal rejection, which already
  carried its source through the commit boundary and restarted there.
- The diskless `xmonad-stale-response` QEMU profile retains two Xterms and maps
  a third whose child exits after 50 ms, inside the bridge's 80 ms quiet period.
  The rebuilt production run observed the action surface and normal exit,
  rejected one stale Manage reply, restarted and reseeded once, preserved both
  persistent surfaces, completed a physical Super-J focus cycle, and logged a
  clean normal shutdown. The transport summary reported one stale response and
  zero pending work; schema 16 reported one WM restart, no degradation, and no
  native submission, retirement, callback, or cleanup failure. Mutation tests
  reject missing causal stages, duplicate restart, incorrect counters, the
  historical `UnknownSurface` error, and an exit that was never surface-backed.

<!-- END IMPORTED BODY -->
