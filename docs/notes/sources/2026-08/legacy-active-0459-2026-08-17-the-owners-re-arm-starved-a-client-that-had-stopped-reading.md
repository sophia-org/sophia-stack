---
id: legacy-active-0459
date: 2026-08-17
recorded_date: 2026-08-17
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-17: the owner's re-arm starved a client that had stopped reading

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13828–13863. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The re-arm fix held. Two stale rejections were logged with `rearmed=true` and
  the session survived both, where either one previously killed it, and restarts
  fell from three to one. The run still failed, in two new places.
- First, the reference client stopped servicing its policy connection. Once its
  proof surfaces were placed it blocked for up to ten seconds waiting for the
  topology outcome, deliberately, to avoid generating layout commits while
  native scanout drained. Its own comment claimed the pause could not strand
  anything. It stranded the owner's *write*: the owner, now correctly re-arming,
  kept issuing cycles into a socket nobody was draining until its write deadline
  fired, reported `restart_requested reason=public_transport_failed
  error=Io(...os error 11)`, and restarted the client in the middle of its own
  topology apply. That is why this run contains no
  `sophia_output_v1_reference` line at all: the output client had connected and
  was killed before it could submit. The re-arm did not create the fragility,
  but it supplied the write pressure that reached it.
- The client now collects the topology outcome without ever blocking, and
  tolerates the owner's expected quiet window while a topology transaction
  prepares rather than treating a read deadline as a fault. Re-proposing during
  the drain is harmless because its tiling is a pure projection of the received
  snapshot.
- Second, and initially misdiagnosed: `begin_recovery` failed the session with
  "layout recovery surface has no committed authority size". The first theory
  was that repeated recovery rounds drained the safe observations. That was
  wrong — `record_committed` seeds a safe observation and only `remove` clears
  one, so the absence means the coordinator no longer knows the surface at all.
  The all-or-nothing failure is therefore correct, and an Engine test already
  pinned it; relaxing it would have destroyed a real invariant to paper over a
  caller bug.
- The actual defect was an asymmetry in the caller. It filtered its fixed
  admission set by `safe_size(..).is_some()` but applied no such test to the
  requested sizes, so a surface withdrawn while its resize was outstanding took
  down the recovery of every surface that could still have been recovered. Both
  arguments are now filtered the same way, and the Engine invariant is untouched.

<!-- END IMPORTED BODY -->
